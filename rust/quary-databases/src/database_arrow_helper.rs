use chrono::{DateTime, Utc};
use duckdb::arrow::array::{array, Array, RecordBatch};
use quary_core::databases::{QueryError, QueryResult};
use std::sync::Arc;

pub(crate) fn convert_array_to_vec_string(
    array: &[Arc<dyn Array>],
) -> Result<Vec<Vec<String>>, String> {
    let num_rows = array[0].len();
    let num_columns = array.len();
    let mut rows = Vec::with_capacity(num_rows);
    for _ in 0..num_rows {
        let row = vec!["".to_string(); num_columns];
        rows.push(row);
    }

    for (i, row) in rows.iter_mut().enumerate() {
        for (j, value) in row.iter_mut().enumerate() {
            let array = &array[j];
            if let Some(array) = array.as_any().downcast_ref::<array::StringArray>() {
                if array.is_null(i) {
                    *value = "NULL".to_string();
                } else {
                    *value = array.value(i).to_string();
                }
            } else if let Some(array) = array.as_any().downcast_ref::<array::Int32Array>() {
                if array.is_null(i) {
                    *value = "NULL".to_string();
                } else {
                    *value = array.value(i).to_string();
                }
            } else if let Some(array) = array.as_any().downcast_ref::<array::Int64Array>() {
                if array.is_null(i) {
                    *value = "NULL".to_string();
                } else {
                    *value = array.value(i).to_string();
                }
            } else if let Some(array) = array.as_any().downcast_ref::<array::Float32Array>() {
                if array.is_null(i) {
                    *value = "NULL".to_string();
                } else {
                    *value = array.value(i).to_string();
                }
            } else if let Some(array) = array.as_any().downcast_ref::<array::Float64Array>() {
                if array.is_null(i) {
                    *value = "NULL".to_string();
                } else {
                    *value = array.value(i).to_string();
                }
            } else if let Some(array) = array.as_any().downcast_ref::<array::BooleanArray>() {
                if array.is_null(i) {
                    *value = "NULL".to_string();
                } else {
                    *value = array.value(i).to_string();
                }
            } else if let Some(array) = array.as_any().downcast_ref::<array::Date64Array>() {
                if array.is_null(i) {
                    *value = "NULL".to_string();
                } else {
                    *value = array.value(i).to_string();
                }
            } else if let Some(array) = array.as_any().downcast_ref::<array::Date32Array>() {
                if array.is_null(i) {
                    *value = "NULL".to_string();
                } else {
                    *value = array.value(i).to_string();
                }
            } else if let Some(array) = array
                .as_any()
                .downcast_ref::<array::TimestampMicrosecondArray>()
            {
                if array.is_null(i) {
                    *value = "NULL".to_string();
                } else {
                    let timestamp_micros = array.value(i);
                    let datetime_utc = DateTime::<Utc>::from_timestamp(
                        timestamp_micros / 1_000_000,
                        (timestamp_micros % 1_000_000) as u32 * 1_000,
                    )
                    .ok_or("error converting timestamp to datetime")?;
                    *value = datetime_utc.format("%Y-%m-%d %H:%M:%S%.6f %Z").to_string();
                }
            } else {
                let array_type = array.data_type();
                return Err(format!("Unsupported array type: {:?}", array_type));
            }
        }
    }

    // Example for a specific array type, e.g., StringArray
    Ok(rows)
}

pub(crate) fn record_batches_to_result(
    query: &str,
    rbs: Vec<RecordBatch>,
) -> Result<QueryResult, QueryError> {
    if rbs.is_empty() {
        return Ok(QueryResult {
            columns: vec![],
            rows: vec![],
        });
    }

    let columns = rbs[0]
        .schema()
        .fields()
        .iter()
        .map(|f| f.name().clone())
        .collect::<Vec<String>>();

    let mut rows = Vec::new();
    for rb in &rbs {
        let batch_rows = convert_array_to_vec_string(rb.columns())
            .map_err(|e| QueryError::new(query.to_string(), e))?;
        rows.extend(batch_rows);
    }

    Ok(QueryResult {
        columns: columns.into_iter().map(|c| (c, None)).collect(),
        rows,
    })
}
