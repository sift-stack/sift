use crate::cmd::import::parquet::detect_parquet_schema;
use parquet::arrow::arrow_writer::ArrowWriter;
use std::io::{ Write, Seek };
use arrow_array::{ Float64Array, Int32Array, RecordBatch, StringArray, TimestampSecondArray };
use arrow_schema::{ TimeUnit, DataType, Field, Schema};
use std::sync::Arc;

// Helpers
fn create_test_batch() -> Result<RecordBatch, Box<dyn std::error::Error>> {
    let schema = Arc::new(Schema::new(vec![
        Field::new("time", DataType::Timestamp(TimeUnit::Second, None), false),
        Field::new("a", DataType::Int32, false),
        Field::new("b", DataType::Float64, true),
        Field::new("c", DataType::Utf8, false),
    ]));

    let batch = RecordBatch::try_new(
        schema,
        vec![
            Arc::new(TimestampSecondArray::from(vec![1, 2, 3])),
            Arc::new(Int32Array::from(vec![1, 2, 3])),
            Arc::new(Float64Array::from(vec![Some(4.0), None, Some(5.0)])),
            Arc::new(StringArray::from(vec!["alpha", "beta", "gamma"])),
        ],
    )?;
    Ok(batch)
}

fn write_to_parquet_memory(batch: &RecordBatch) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let mut buffer = Vec::new();
    let mut writer = ArrowWriter::try_new(&mut buffer, batch.schema(), None)?;
    writer.write(batch)?;
    writer.close()?;
    Ok(buffer)
}

// Tests
#[test]
fn test_detect_parquet_on_import() -> Result<(), Box<dyn std::error::Error>> {
    let batch = create_test_batch()?;

    let parquet_bytes = write_to_parquet_memory(&batch)?;

    let mut file = tempfile::tempfile()?;
    file.write_all(&parquet_bytes)?;
    file.rewind()?;

    let config = detect_parquet_schema::detect_flat_dataset_config(&file)?;

    let time_col = match config.time_column {
        Some(col) => col,
        None => return Err("no time column detected".into()),
    };

    assert_eq!(time_col.path, "time");

    assert_eq!(config.data_columns.len(), 4);
    assert_eq!(config.data_columns[1].path, "a");
    assert_eq!(config.data_columns[2].path, "b");
    assert_eq!(config.data_columns[3].path, "c");

    Ok(())
}



