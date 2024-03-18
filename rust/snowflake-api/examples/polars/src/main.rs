use anyhow::Result;
use polars::frame::DataFrame;

use snowflake_api::SnowflakeApi;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();
    tracing_subscriber::fmt::init();

    let api = SnowflakeApi::from_env()?;

    // run a query that returns a tabular arrow response
    run_and_print(
        &api,
        r"
            select
                count(query_id) as num_queries,
                user_name
            from snowflake.account_usage.query_history
            where
                start_time > current_date - 7
            group by user_name;
    ",
    )
    .await?;

    // run a query that returns a json response
    run_and_print(&api, r"SHOW DATABASES;").await?;

    Ok(())
}

async fn run_and_print(api: &SnowflakeApi, sql: &str) -> Result<()> {
    let res = api.exec_raw(sql).await?;

    let df = DataFrame::try_from(res)?;
    // alternatively, you can use the `try_into` method on the response
    // let df: DataFrame = res.try_into()?;

    println!("{:?}", df);

    Ok(())
}
