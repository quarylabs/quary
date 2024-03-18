use anyhow::Result;
use arrow::util::pretty::pretty_format_batches;
use opentelemetry::global;
use opentelemetry::KeyValue;
use opentelemetry_otlp::WithExportConfig;
use opentelemetry_sdk::{runtime, trace as sdktrace, Resource};
use reqwest_middleware::Extension;
use reqwest_tracing::{OtelName, SpanBackendWithUrl};
use tracing_subscriber::layer::SubscriberExt;

use snowflake_api::connection::Connection;
use snowflake_api::{AuthArgs, QueryResult, SnowflakeApiBuilder};

#[tokio::main]
async fn main() -> Result<()> {
    let tracer = opentelemetry_otlp::new_pipeline()
        .tracing()
        .with_exporter(
            opentelemetry_otlp::new_exporter()
                .tonic()
                .with_endpoint("http://localhost:4317"),
        )
        .with_trace_config(
            sdktrace::config().with_resource(Resource::new(vec![KeyValue::new(
                opentelemetry_semantic_conventions::resource::SERVICE_NAME,
                "snowflake-rust-client-demo",
            )])),
        )
        .install_batch(runtime::Tokio)?;

    let telemetry = tracing_opentelemetry::layer().with_tracer(tracer.clone());
    let subscriber = tracing_subscriber::Registry::default().with(telemetry);
    tracing::subscriber::set_global_default(subscriber)?;

    dotenv::dotenv().ok();

    let mut client = Connection::default_client_builder()?;
    client = client
        .with_init(Extension(OtelName(std::borrow::Cow::Borrowed(
            "snowflake-api",
        ))))
        .with(reqwest_tracing::TracingMiddleware::<SpanBackendWithUrl>::new());

    let builder = SnowflakeApiBuilder::new(AuthArgs::from_env()?).with_client(client.build());
    let api = builder.build()?;

    run_in_span(&api).await?;

    global::shutdown_tracer_provider();

    Ok(())
}

#[tracing::instrument(name = "snowflake_api", skip(api))]
async fn run_in_span(api: &snowflake_api::SnowflakeApi) -> anyhow::Result<()> {
    let res = api.exec("select 'hello from snowflake' as col1;").await?;

    match res {
        QueryResult::Arrow(a) => {
            println!("{}", pretty_format_batches(&a).unwrap());
        }
        QueryResult::Json(j) => {
            println!("{}", j);
        }
        QueryResult::Empty => {
            println!("Query finished successfully")
        }
    }

    Ok(())
}
