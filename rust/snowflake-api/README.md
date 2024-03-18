# snowflake-rs

Snowflake library for undocumented public API. If you want to query documented public [SQL REST API](https://docs.snowflake.com/developer-guide/sql-api/intro) use [snowflake-jwt](https://crates.io/crates/snowflake-jwt) together with your favourite request library, see [./jwt/examples](../jwt/examples) for how it's done.

## Features

Since it does a lot of I/O the library is async-only, and currently has hard dependency on [tokio](https://tokio.rs/) as a runtime due to use of [reqwest](https://github.com/seanmonstar/reqwest).

- [x] Single statements [example](./examples/run_sql.rs)
- [ ] Multiple statements
- [ ] Async requests (is it needed if whole library is async?)
- [x] Query results in [Arrow](https://arrow.apache.org/)
- [x] Chunked query results
- [x] Password, certificate, env auth
- [ ] Browser-auth
- [x] Closing session
- [x] Token renewal
- [x] PUT support [example](./examples/filetransfer.rs)
- [ ] GET support
- [x] AWS integration
- [ ] GCloud integration
- [ ] Azure integration
- [ ] Parallel uploading of small files
- [x] Polars support [example](./examples/polars/src/main.rs)
- [x] Tracing / custom reqwest middlware [example](./examples/tracing/src/main.rs)

## Why

Snowflake has 2 public APIs, one is [SQL REST API](https://docs.snowflake.com/developer-guide/sql-api/intro), which is limited in its support of [PUT](https://docs.snowflake.com/en/sql-reference/sql/put) and [GET](https://docs.snowflake.com/en/sql-reference/sql/get) statements and another undocumented API, which is used by official [Drivers](https://docs.snowflake.com/en/developer-guide/drivers) with the support for both.

This implementation emulates [gosnowflake](https://github.com/snowflakedb/gosnowflake) library, as each official driver comes with a different set of internal flags and defaults (which are selected by `CLIENT_APP_ID`) the Go implementation is the only one currently outputting Arrow by-default.

We've chosen not to generate bindings for C/C++ [libsnowflakeclient](https://github.com/snowflakedb/libsnowflakeclient) library (which backs ODBC driver) as it is in active development and building it under macOS M1 is bigger effort than writing our own API wrapper.

## Usage

In your Cargo.toml:

```toml
[dependencies]
snowflake-api = "0.7.0"
```

Check [examples](./examples) for working programs using the library.

```rust
use anyhow::Result;
use snowflake_api::{QueryResult, SnowflakeApi};

async fn run_query(sql: &str) -> Result<QueryResult> {
    let mut api = SnowflakeApi::with_password_auth(
        "ACCOUNT_IDENTIFIER",
        Some("WAREHOUSE"),
        Some("DATABASE"),
        Some("SCHEMA"),
        "USERNAME",
        Some("ROLE"),
        "PASSWORD",
    )?;
    let res = api.exec(sql).await?;

    Ok(res)
}
```

Or using environment variables:

```rust
 use anyhow::Result;
use snowflake_api::{QueryResult, SnowflakeApi};

async fn run_query(sql: &str) -> Result<QueryResult> {
    let mut api = SnowflakeApi::from_env()?;
    let res = api.exec(sql).await?;

    Ok(res)
}
```

## PUT / GET

[PUT](https://docs.snowflake.com/en/sql-reference/sql/put)/[GET](https://docs.snowflake.com/en/sql-reference/sql/get) statements allow you to access Snowflake-owned storage instead of provisioning your own when doing [COPY INTO](https://docs.snowflake.com/en/sql-reference/sql/copy-into-table). Storage provider depends on which cloud your Snowflake account was provisioned in, hence the need to support multiple cloud backends.
