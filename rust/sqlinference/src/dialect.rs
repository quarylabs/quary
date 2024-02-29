use sqlparser::dialect::{
    BigQueryDialect, Dialect as ParseDialect, DuckDbDialect, PostgreSqlDialect, SQLiteDialect,
    SnowflakeDialect,
};
use std::sync::Arc;

pub enum Dialect {
    SQLite,
    BigQuery,
    DuckDB,
    Snowflake,
    Postgres,
}

impl Dialect {
    pub fn get_dialect(&self) -> Arc<dyn ParseDialect> {
        match self {
            Dialect::SQLite => Arc::new(SQLiteDialect {}),
            Dialect::BigQuery => Arc::new(BigQueryDialect {}),
            Dialect::Snowflake => Arc::new(SnowflakeDialect {}),
            Dialect::DuckDB => Arc::new(DuckDbDialect {}),
            Dialect::Postgres => Arc::new(PostgreSqlDialect {}),
        }
    }
}
