use std::{fs::File, path::Path, sync::Arc};

use arrow::array::{Array, AsArray, Int64Array, RecordBatch};
use arrow::datatypes::{DataType, Field, Float64Type, Int64Type, Schema, SchemaRef};
use bytes::Bytes;
use parquet::arrow::{ArrowWriter, arrow_reader::ParquetRecordBatchReaderBuilder};
use pbjson_types::{Any, Timestamp};
use prost::Message;
use sift_rs::{
    channels::v3::Channel,
    data::v2::{
        DoubleValue, DoubleValues, GetDataResponse, Metadata,
        data_service_server::DataServiceServer, metadata,
    },
    runs::v2::Run,
};
use sift_test_util::{grpc::memory_sift_channel, mock::data::v2::MockDataServiceImpl};
use tempdir::TempDir;
use tokio::task::JoinHandle;
use tonic::{Response, Status, transport::Server};

use super::{ChannelInput, DataService, TimeRange};
use crate::service::common::unix_nanos_to_secs_and_subsec_nanos;

async fn service_with_mock(mock: MockDataServiceImpl) -> (DataService, JoinHandle<()>) {
    let (client, server) = tokio::io::duplex(1024);
    let channel = memory_sift_channel(client).await;

    let handle = tokio::spawn(async move {
        Server::builder()
            .add_service(DataServiceServer::new(mock))
            .serve_with_incoming(tokio_stream::once(Ok::<_, std::io::Error>(server)))
            .await
            .unwrap();
    });

    (DataService::new(channel), handle)
}

fn raw_channel(channel_id: &str) -> ChannelInput {
    ChannelInput::Raw(Box::new(Channel {
        channel_id: channel_id.into(),
        ..Default::default()
    }))
}

fn asset_range(start_nanos: i64, end_nanos: i64) -> TimeRange {
    TimeRange::Asset {
        start_time_unix_nanos: start_nanos,
        end_time_unix_nanos: end_nanos,
    }
}

fn double_page(channel_id: &str, channel_name: &str, samples: Vec<(i64, f64)>) -> Any {
    let values = samples
        .into_iter()
        .map(|(ts_nanos, value)| {
            let (seconds, nanos) = unix_nanos_to_secs_and_subsec_nanos(ts_nanos);
            DoubleValue {
                timestamp: Some(Timestamp { seconds, nanos }),
                value,
            }
        })
        .collect();

    let payload = DoubleValues {
        metadata: Some(Metadata {
            channel: Some(metadata::Channel {
                channel_id: channel_id.into(),
                name: channel_name.into(),
                ..Default::default()
            }),
            ..Default::default()
        }),
        values,
    };

    Any {
        type_url: "sift.data.v2.DoubleValues".into(),
        value: Bytes::from(payload.encode_to_vec()),
    }
}

fn read_parquet(buffer: Vec<u8>) -> Vec<arrow::array::RecordBatch> {
    let reader = ParquetRecordBatchReaderBuilder::try_new(Bytes::from(buffer))
        .expect("failed to open parquet")
        .build()
        .expect("failed to build parquet reader");
    reader
        .collect::<Result<Vec<_>, _>>()
        .expect("failed to read record batches")
}

#[tokio::test]
async fn get_data_empty_inputs_errors() {
    let mock = MockDataServiceImpl::new();
    let (service, _h) = service_with_mock(mock).await;

    let mut buffer = Vec::new();
    let err = service
        .get_data(&[], asset_range(0, 1_000_000_000), 0, &mut buffer)
        .await
        .expect_err("expected error on empty channel_inputs");

    assert!(err.to_string().contains("channel inputs cannot be empty"));
}

#[tokio::test]
async fn get_data_writes_double_channel_to_parquet() {
    let mut mock = MockDataServiceImpl::new();
    mock.expect_get_data().times(1).returning(|req| {
        let req = req.into_inner();
        assert_eq!(req.queries.len(), 1);
        Ok(Response::new(GetDataResponse {
            data: vec![double_page(
                "c1",
                "temp",
                vec![(1_000_000_000, 10.0), (2_000_000_000, 11.0)],
            )],
            next_page_token: String::new(),
        }))
    });

    let (service, _h) = service_with_mock(mock).await;
    let mut buffer = Vec::new();
    service
        .get_data(
            &[raw_channel("c1")],
            asset_range(0, 3_000_000_000),
            0,
            &mut buffer,
        )
        .await
        .expect("get_data failed");

    let batches = read_parquet(buffer);
    assert_eq!(batches.len(), 1);
    let batch = &batches[0];
    assert_eq!(batch.num_rows(), 2);
    assert_eq!(batch.num_columns(), 2);

    let schema = batch.schema();
    assert_eq!(schema.field(0).name(), "timestamp_unix_nanos");
    assert_eq!(schema.field(0).data_type(), &DataType::Int64);
    assert_eq!(schema.field(1).data_type(), &DataType::Float64);

    let timestamps = batch.column(0).as_primitive::<Int64Type>();
    assert_eq!(timestamps.values(), &[1_000_000_000, 2_000_000_000]);

    let temps = batch.column(1).as_primitive::<Float64Type>();
    assert_eq!(temps.value(0), 10.0);
    assert_eq!(temps.value(1), 11.0);
    assert!(!temps.is_null(0) && !temps.is_null(1));
}

#[tokio::test]
async fn get_data_merges_disjoint_timestamps_with_nulls() {
    let mut mock = MockDataServiceImpl::new();
    mock.expect_get_data().times(1).returning(|_| {
        Ok(Response::new(GetDataResponse {
            data: vec![
                double_page("c1", "a", vec![(1_000_000_000, 1.0), (3_000_000_000, 3.0)]),
                double_page("c2", "b", vec![(2_000_000_000, 20.0)]),
            ],
            next_page_token: String::new(),
        }))
    });

    let (service, _h) = service_with_mock(mock).await;
    let mut buffer = Vec::new();
    service
        .get_data(
            &[raw_channel("c1"), raw_channel("c2")],
            asset_range(0, 4_000_000_000),
            0,
            &mut buffer,
        )
        .await
        .expect("get_data failed");

    let batches = read_parquet(buffer);
    let total_rows: usize = batches.iter().map(|b| b.num_rows()).sum();
    assert_eq!(total_rows, 3, "expected 3 distinct timestamps");

    // Iteration order over the columns HashMap is unspecified, so look up by name.
    let batch = &batches[0];
    let schema = batch.schema();
    let (a_idx, _) = schema
        .fields()
        .iter()
        .enumerate()
        .find(|(_, f)| f.name().starts_with("a "))
        .expect("missing column for channel a");
    let (b_idx, _) = schema
        .fields()
        .iter()
        .enumerate()
        .find(|(_, f)| f.name().starts_with("b "))
        .expect("missing column for channel b");

    let timestamps = batch.column(0).as_primitive::<Int64Type>();
    assert_eq!(
        timestamps.values(),
        &[1_000_000_000, 2_000_000_000, 3_000_000_000]
    );

    let a = batch.column(a_idx).as_primitive::<Float64Type>();
    assert!(!a.is_null(0) && a.value(0) == 1.0);
    assert!(a.is_null(1));
    assert!(!a.is_null(2) && a.value(2) == 3.0);

    let b = batch.column(b_idx).as_primitive::<Float64Type>();
    assert!(b.is_null(0));
    assert!(!b.is_null(1) && b.value(1) == 20.0);
    assert!(b.is_null(2));
}

#[tokio::test]
async fn get_data_paginates_until_token_empty() {
    let mut mock = MockDataServiceImpl::new();
    mock.expect_get_data().returning(|req| {
        let req = req.into_inner();
        let (pages, next) = match req.page_token.as_str() {
            "" => (
                vec![double_page("c1", "temp", vec![(1_000_000_000, 1.0)])],
                "page-2".to_string(),
            ),
            "page-2" => (
                vec![double_page("c1", "temp", vec![(2_000_000_000, 2.0)])],
                String::new(),
            ),
            other => return Err(Status::invalid_argument(format!("bad token: {other}"))),
        };
        Ok(Response::new(GetDataResponse {
            data: pages,
            next_page_token: next,
        }))
    });

    let (service, _h) = service_with_mock(mock).await;
    let mut buffer = Vec::new();
    service
        .get_data(
            &[raw_channel("c1")],
            asset_range(0, 3_000_000_000),
            0,
            &mut buffer,
        )
        .await
        .expect("get_data failed");

    let batches = read_parquet(buffer);
    let timestamps: Vec<i64> = batches
        .iter()
        .flat_map(|b| b.column(0).as_primitive::<Int64Type>().values().to_vec())
        .collect();
    assert_eq!(timestamps, vec![1_000_000_000, 2_000_000_000]);
}

#[tokio::test]
async fn get_data_propagates_grpc_error() {
    let mut mock = MockDataServiceImpl::new();
    mock.expect_get_data()
        .returning(|_| Err(Status::internal("boom")));

    let (service, _h) = service_with_mock(mock).await;
    let mut buffer = Vec::new();
    let err = service
        .get_data(
            &[raw_channel("c1")],
            asset_range(0, 1_000_000_000),
            0,
            &mut buffer,
        )
        .await
        .expect_err("expected error");

    assert!(err.to_string().contains("failed to get data"));
}

#[tokio::test]
async fn get_data_run_without_start_time_errors() {
    let mock = MockDataServiceImpl::new();
    let (service, _h) = service_with_mock(mock).await;

    let time_range = TimeRange::Run {
        run: Box::new(Run {
            run_id: "r1".into(),
            start_time: None,
            stop_time: None,
            ..Default::default()
        }),
        start_time_unix_nanos: None,
        end_time_unix_nanos: None,
    };

    let mut buffer = Vec::new();
    let err = service
        .get_data(&[raw_channel("c1")], time_range, 0, &mut buffer)
        .await
        .expect_err("expected error for run with no start_time");

    assert!(err.to_string().contains("doesn't have a start time"));
}

// --- sql ---

fn sample_schema() -> SchemaRef {
    Arc::new(Schema::new(vec![
        Field::new("ts", DataType::Int64, false),
        Field::new("value", DataType::Float64, false),
    ]))
}

fn write_sample_parquet(path: &Path, ts: &[i64], values: &[f64]) {
    assert_eq!(ts.len(), values.len());
    let schema = sample_schema();
    let batch = RecordBatch::try_new(
        schema.clone(),
        vec![
            Arc::new(Int64Array::from(ts.to_vec())),
            Arc::new(arrow::array::Float64Array::from(values.to_vec())),
        ],
    )
    .expect("failed to build record batch");

    let file = File::create(path).expect("failed to create parquet file");
    let mut writer = ArrowWriter::try_new(file, schema, None).expect("failed to init writer");
    writer.write(&batch).expect("failed to write batch");
    writer.close().expect("failed to close writer");
}

async fn run_sql(
    inputs: Vec<std::path::PathBuf>,
    table_name: &'static str,
    query: &'static str,
) -> anyhow::Result<Vec<u8>> {
    // polars' sync API spins up its own tokio runtime, which panics when called
    // from inside a tokio test. spawn_blocking sidesteps that by running on the
    // blocking thread pool.
    tokio::task::spawn_blocking(move || {
        let mut out = Vec::new();
        DataService::sql(inputs, &mut out, table_name, query).map(|_| out)
    })
    .await
    .expect("spawn_blocking panicked")
}

#[tokio::test(flavor = "multi_thread")]
async fn sql_empty_inputs_errors() {
    let err = run_sql(Vec::new(), "t", "SELECT 1")
        .await
        .expect_err("expected error on empty input list");

    assert!(
        err.to_string()
            .contains("input parquet files cannot be empty")
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn sql_executes_filter_query_on_single_file() {
    let tmp = TempDir::new("sql_filter").expect("tempdir");
    let path = tmp.path().join("data.parquet");
    write_sample_parquet(&path, &[1, 2, 3, 4], &[10.0, 20.0, 30.0, 40.0]);

    let out = run_sql(vec![path], "t", "SELECT ts, value FROM t WHERE value > 20")
        .await
        .expect("sql failed");

    let batches = read_parquet(out);
    let total_rows: usize = batches.iter().map(|b| b.num_rows()).sum();
    assert_eq!(total_rows, 2);

    let ts: Vec<i64> = batches
        .iter()
        .flat_map(|b| b.column(0).as_primitive::<Int64Type>().values().to_vec())
        .collect();
    assert_eq!(ts, vec![3, 4]);

    let vals: Vec<f64> = batches
        .iter()
        .flat_map(|b| b.column(1).as_primitive::<Float64Type>().values().to_vec())
        .collect();
    assert_eq!(vals, vec![30.0, 40.0]);
}

#[tokio::test(flavor = "multi_thread")]
async fn sql_unions_multiple_files() {
    let tmp = TempDir::new("sql_union").expect("tempdir");
    let p1 = tmp.path().join("a.parquet");
    let p2 = tmp.path().join("b.parquet");
    write_sample_parquet(&p1, &[1, 2], &[1.0, 2.0]);
    write_sample_parquet(&p2, &[3, 4, 5], &[3.0, 4.0, 5.0]);

    let out = run_sql(vec![p1, p2], "t", "SELECT ts FROM t ORDER BY ts")
        .await
        .expect("sql failed");

    let batches = read_parquet(out);
    let ts: Vec<i64> = batches
        .iter()
        .flat_map(|b| b.column(0).as_primitive::<Int64Type>().values().to_vec())
        .collect();
    assert_eq!(ts, vec![1, 2, 3, 4, 5]);
}

#[tokio::test(flavor = "multi_thread")]
async fn sql_invalid_query_errors() {
    let tmp = TempDir::new("sql_bad_query").expect("tempdir");
    let path = tmp.path().join("data.parquet");
    write_sample_parquet(&path, &[1], &[1.0]);

    let err = run_sql(vec![path], "t", "NOT A QUERY")
        .await
        .expect_err("expected error from invalid SQL");

    assert!(err.to_string().contains("failed to apply SQL query on t"));
}

#[tokio::test(flavor = "multi_thread")]
async fn sql_missing_input_file_errors() {
    let tmp = TempDir::new("sql_missing").expect("tempdir");
    let missing = tmp.path().join("does_not_exist.parquet");

    let err = run_sql(vec![missing], "t", "SELECT * FROM t")
        .await
        .expect_err("expected error from missing input file");

    // Either path validation OR scan errors out; both are acceptable failures.
    let msg = err.to_string();
    assert!(
        msg.contains("invalid parquet file path provided")
            || msg.contains("failed to initialize data frame")
            || msg.contains("failed to apply SQL query")
            || msg.contains("failed to execute query"),
        "unexpected error: {msg}"
    );
}
