use anyhow::Result;
use arrow::util::pretty::pretty_format_batches;
use clap::Parser;
use snowflake_api::{QueryResult, SnowflakeApi};
use std::fs;

extern crate snowflake_api;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to RSA PEM private key
    #[arg(long)]
    private_key: Option<String>,

    /// Password if certificate is not present
    #[arg(long)]
    password: Option<String>,

    /// <account_identifier> in Snowflake format, uppercase
    #[arg(short, long)]
    account_identifier: String,

    /// Database name
    #[arg(short, long)]
    database: String,

    /// Schema name
    #[arg(long)]
    schema: String,

    /// Warehouse
    #[arg(short, long)]
    warehouse: String,

    /// username to whom the private key belongs to
    #[arg(short, long)]
    username: String,

    /// role which user will assume
    #[arg(short, long)]
    role: String,

    #[arg(long)]
    csv_path: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    pretty_env_logger::init();

    let args = Args::parse();

    let mut api = match (&args.private_key, &args.password) {
        (Some(pkey), None) => {
            let pem = fs::read_to_string(pkey)?;
            SnowflakeApi::with_certificate_auth(
                &args.account_identifier,
                Some(&args.warehouse),
                Some(&args.database),
                Some(&args.schema),
                &args.username,
                Some(&args.role),
                &pem,
            )?
        }
        (None, Some(pwd)) => SnowflakeApi::with_password_auth(
            &args.account_identifier,
            Some(&args.warehouse),
            Some(&args.database),
            Some(&args.schema),
            &args.username,
            Some(&args.role),
            pwd,
        )?,
        _ => {
            panic!("Either private key path or password must be set")
        }
    };

    log::info!("Creating table");
    api.exec(
        "CREATE OR REPLACE TABLE OSCAR_AGE_MALE(Index integer, Year integer, Age integer, Name varchar, Movie varchar);"
    ).await?;

    log::info!("Uploading CSV file");
    api.exec(&format!("PUT file://{} @%OSCAR_AGE_MALE;", &args.csv_path))
        .await?;

    log::info!("Create temporary file format");
    api.exec(
        "CREATE OR REPLACE TEMPORARY FILE FORMAT CUSTOM_CSV_FORMAT TYPE = CSV COMPRESSION = NONE FIELD_DELIMITER = ',' FILE_EXTENSION = 'csv' SKIP_HEADER = 1 FIELD_OPTIONALLY_ENCLOSED_BY = '\"' TRIM_SPACE = TRUE SKIP_BLANK_LINES = TRUE;"
    ).await?;

    log::info!("Copying into table");
    api.exec("COPY INTO OSCAR_AGE_MALE FILE_FORMAT = CUSTOM_CSV_FORMAT;")
        .await?;

    log::info!("Querying for results");
    let res = api.exec("SELECT * FROM OSCAR_AGE_MALE;").await?;

    match res {
        QueryResult::Arrow(a) => {
            println!("{}", pretty_format_batches(&a).unwrap());
        }
        QueryResult::Empty => {
            println!("Nothing was returned");
        }
        QueryResult::Json(j) => {
            println!("{j}");
        }
    }

    api.close_session().await?;

    Ok(())
}
