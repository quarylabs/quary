use std::convert::TryFrom;

use bytes::{Buf, Bytes};
use polars_core::frame::DataFrame;
use polars_io::ipc::IpcStreamReader;
use polars_io::json::{JsonFormat, JsonReader};
use polars_io::SerReader;
use serde::de::Error;
use serde_json::{Map, Value};
use thiserror::Error;

use crate::{JsonResult, RawQueryResult};

#[derive(Error, Debug)]
pub enum PolarsCastError {
    #[error(transparent)]
    SerdeError(#[from] serde_json::Error),

    #[error(transparent)]
    PolarsError(#[from] polars_core::error::PolarsError),
}

impl RawQueryResult {
    pub fn to_polars(self) -> Result<DataFrame, PolarsCastError> {
        match self {
            RawQueryResult::Bytes(bytes) => dataframe_from_bytes(bytes),
            RawQueryResult::Json(json) => dataframe_from_json(&json),
            RawQueryResult::Empty => Ok(DataFrame::empty()),
        }
    }
}

fn dataframe_from_json(json_result: &JsonResult) -> Result<DataFrame, PolarsCastError> {
    let objects = arrays_to_objects(json_result)?;
    // fixme: serializing json again, is it possible to keep bytes? or implement casting?
    let json_string = serde_json::to_string(&objects)?;
    let reader = std::io::Cursor::new(json_string.as_bytes());
    let df = JsonReader::new(reader)
        .with_json_format(JsonFormat::Json)
        .infer_schema_len(Some(5))
        .finish()?;
    Ok(df)
}

/// This is required because the polars json reader expects an array of objects, and
/// the snowflake json response is an array of arrays (without real column names).
///
/// This is apparent if you run a system query (not a select) like `SHOW DATABASES;`.
fn arrays_to_objects(json_result: &JsonResult) -> Result<Value, PolarsCastError> {
    let arrays: &Vec<Value> = json_result
        .value
        .as_array()
        .ok_or(serde_json::Error::custom("Input must be array an array"))?;
    let names: Vec<String> = json_result.schema.iter().map(|s| s.name.clone()).collect();

    let objects: Result<Vec<Value>, PolarsCastError> = arrays
        .iter()
        .map(|array| {
            array
                .as_array()
                .ok_or(serde_json::Error::custom("Input must be array of array"))
                .map(|array| {
                    // fixme: lots of copying
                    let map: Map<String, Value> =
                        names.clone().into_iter().zip(array.clone()).collect();
                    Value::Object(map)
                })
                .map_err(PolarsCastError::SerdeError)
        })
        .collect();

    objects.map(Value::Array)
}

fn dataframe_from_bytes(bytes: Vec<Bytes>) -> Result<DataFrame, PolarsCastError> {
    let mut df = DataFrame::empty();
    for b in bytes {
        let df_chunk = IpcStreamReader::new(b.reader()).finish()?;
        df.vstack_mut(&df_chunk)?;
    }
    df.align_chunks();
    Ok(df)
}

impl TryFrom<RawQueryResult> for DataFrame {
    type Error = PolarsCastError;

    fn try_from(value: RawQueryResult) -> Result<Self, Self::Error> {
        value.to_polars()
    }
}
