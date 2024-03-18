use reqwest::header::{self, HeaderMap, HeaderName, HeaderValue};
use reqwest_middleware::ClientWithMiddleware;
use reqwest_retry::policies::ExponentialBackoff;
use reqwest_retry::RetryTransientMiddleware;
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};
use thiserror::Error;
use url::Url;
use uuid::Uuid;

#[derive(Error, Debug)]
pub enum ConnectionError {
    #[error(transparent)]
    RequestError(#[from] reqwest::Error),

    #[error(transparent)]
    RequestMiddlewareError(#[from] reqwest_middleware::Error),

    #[error(transparent)]
    UrlParsing(#[from] url::ParseError),

    #[error(transparent)]
    Deserialization(#[from] serde_json::Error),

    #[error(transparent)]
    InvalidHeader(#[from] header::InvalidHeaderValue),
}

/// Container for query parameters
/// This API has different endpoints and MIME types for different requests
struct QueryContext {
    path: &'static str,
    accept_mime: &'static str,
}

pub enum QueryType {
    LoginRequest,
    TokenRequest,
    CloseSession,
    JsonQuery,
    ArrowQuery,
}

impl QueryType {
    const fn query_context(&self) -> QueryContext {
        match self {
            Self::LoginRequest => QueryContext {
                path: "session/v1/login-request",
                accept_mime: "application/json",
            },
            Self::TokenRequest => QueryContext {
                path: "/session/token-request",
                accept_mime: "application/snowflake",
            },
            Self::CloseSession => QueryContext {
                path: "session",
                accept_mime: "application/snowflake",
            },
            Self::JsonQuery => QueryContext {
                path: "queries/v1/query-request",
                accept_mime: "application/json",
            },
            Self::ArrowQuery => QueryContext {
                path: "queries/v1/query-request",
                accept_mime: "application/snowflake",
            },
        }
    }
}

/// Connection pool
/// Minimal session will have at least 2 requests - login and query
pub struct Connection {
    // no need for Arc as it's already inside the reqwest client
    client: ClientWithMiddleware,
}

impl Connection {
    pub fn new() -> Result<Self, ConnectionError> {
        let client = Self::default_client_builder()?;

        Ok(Self::new_with_middware(client.build()))
    }

    /// Allow a user to provide their own middleware
    ///
    /// Users can provide their own middleware to the connection like this:
    /// ```rust
    /// use snowflake_api::connection::Connection;
    /// let mut client = Connection::default_client_builder();
    ///  // modify the client builder here
    /// let connection = Connection::new_with_middware(client.unwrap().build());
    /// ```
    /// This is not intended to be called directly, but is used by `SnowflakeApiBuilder::with_client`
    pub fn new_with_middware(client: ClientWithMiddleware) -> Self {
        Self { client }
    }

    pub fn default_client_builder() -> Result<reqwest_middleware::ClientBuilder, ConnectionError> {
        let retry_policy = ExponentialBackoff::builder().build_with_max_retries(3);

        let client = reqwest::ClientBuilder::new()
            .user_agent("Rust/0.0.1")
            .gzip(true)
            .referer(false);

        #[cfg(debug_assertions)]
        let client = client.connection_verbose(true);

        let client = client.build()?;

        Ok(reqwest_middleware::ClientBuilder::new(client)
            .with(RetryTransientMiddleware::new_with_policy(retry_policy)))
    }

    /// Perform request of given query type with extra body or parameters
    // todo: implement soft error handling
    // todo: is there better way to not repeat myself?
    pub async fn request<R: serde::de::DeserializeOwned>(
        &self,
        query_type: QueryType,
        account_identifier: &str,
        extra_get_params: &[(&str, &str)],
        auth: Option<&str>,
        body: impl serde::Serialize,
    ) -> Result<R, ConnectionError> {
        let context = query_type.query_context();

        let request_id = Uuid::new_v4();
        let request_guid = Uuid::new_v4();
        let client_start_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs()
            .to_string();
        // fixme: update uuid's on the retry
        let request_id = request_id.to_string();
        let request_guid = request_guid.to_string();

        let mut get_params = vec![
            ("clientStartTime", client_start_time.as_str()),
            ("requestId", request_id.as_str()),
            ("request_guid", request_guid.as_str()),
        ];
        get_params.extend_from_slice(extra_get_params);

        let url = format!(
            "https://{}.snowflakecomputing.com/{}",
            &account_identifier, context.path
        );
        let url = Url::parse_with_params(&url, get_params)?;

        let mut headers = HeaderMap::new();

        headers.append(
            header::ACCEPT,
            HeaderValue::from_static(context.accept_mime),
        );
        if let Some(auth) = auth {
            let mut auth_val = HeaderValue::from_str(auth)?;
            auth_val.set_sensitive(true);
            headers.append(header::AUTHORIZATION, auth_val);
        }

        // todo: persist client to use connection polling
        let resp = self
            .client
            .post(url)
            .headers(headers)
            .json(&body)
            .send()
            .await?;

        Ok(resp.json::<R>().await?)
    }

    pub async fn get_chunk(
        &self,
        url: &str,
        headers: &HashMap<String, String>,
    ) -> Result<bytes::Bytes, ConnectionError> {
        let mut header_map = HeaderMap::new();
        for (k, v) in headers {
            header_map.insert(
                HeaderName::from_bytes(k.as_bytes()).unwrap(),
                HeaderValue::from_bytes(v.as_bytes()).unwrap(),
            );
        }
        let bytes = self
            .client
            .get(url)
            .headers(header_map)
            .send()
            .await?
            .bytes()
            .await?;
        Ok(bytes)
    }
}
