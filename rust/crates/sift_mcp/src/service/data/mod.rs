use std::{
    collections::HashMap,
    io::Write,
    mem,
    path::PathBuf,
    sync::Arc,
    time::{SystemTime, UNIX_EPOCH},
};

use anyhow::{Context, Result, bail};
use arrow::{
    array::{
        ArrayBuilder, BinaryBuilder, BooleanBuilder, Float32Builder, Float64Builder, Int32Builder,
        Int64Builder, RecordBatch, StringBuilder, UInt32Builder, UInt64Builder,
    },
    datatypes::{DataType, Field, Schema},
};
use parquet::arrow::ArrowWriter;
use pbjson_types::Timestamp;
use polars::{
    prelude::{LazyFrame, ParquetWriter, PlRefPath},
    sql::SQLContext,
};
use prost::Message;
use sift_rs::{
    SiftChannel,
    calculated_channels::v1::{ExpressionChannelReference, ExpressionMode, ExpressionRequest},
    channels::v3::Channel,
    data::v2::{
        BitFieldElementValues, BitFieldValue, BitFieldValues, BoolValue, BoolValues, BytesValue,
        BytesValues, CalculatedChannelQuery, ChannelQuery, DoubleValue, DoubleValues, EnumValue,
        EnumValues, FloatValue, FloatValues, GetDataRequest, GetDataResponse, Int32Value,
        Int32Values, Int64Value, Int64Values, Query, StringValue, StringValues, Uint32Value,
        Uint32Values, Uint64Value, Uint64Values, data_service_client::DataServiceClient,
        query::Query as QueryKind,
    },
    runs::v2::Run,
};

use crate::policy::{RetryPolicy, with_retry};
use crate::service::common::{
    BIT_FIELD_METADATA_KEY, ColumnName, ENUM_METADATA_KEY, PAGE_SIZE, TS_COLUMN_NAME,
    secs_and_subsec_nanos_to_unix_nanos, unix_nanos_to_secs_and_subsec_nanos,
};

#[cfg(test)]
mod test;

const ROW_FLUSH_THRESHOLD: usize = 1_000_000;
const SIZE_FLUSH_THRESHOLD: usize = 64 << 20;

#[derive(Clone)]
pub struct DataService {
    channel: SiftChannel,
    policy: RetryPolicy,
}

pub enum ChannelInput {
    Raw(Box<Channel>),
    #[allow(dead_code)]
    Calculation {
        name: String,
        input_channels: Vec<(String, Channel)>,
        expression: String,
    },
}

pub enum TimeRange {
    Run {
        run: Box<Run>,
        start_time_unix_nanos: Option<i64>,
        end_time_unix_nanos: Option<i64>,
    },
    Asset {
        start_time_unix_nanos: i64,
        end_time_unix_nanos: i64,
    },
}

struct ChannelColumn {
    json_metadata: Option<String>,
    /// unix nanos to value
    values: Vec<(i64, ChannelValue)>,
    data_type: DataType,
}

enum ChannelValue {
    F32(f32),
    F64(f64),
    I32(i32),
    I64(i64),
    U32(u32),
    U64(u64),
    String(String),
    Bool(bool),
    Enum(String),
    BitField(u32),
    Bytes(Vec<u8>),
}

impl DataService {
    pub fn new(channel: SiftChannel, policy: RetryPolicy) -> Self {
        Self { channel, policy }
    }

    pub fn sql<W: Write>(
        input_parquet_files: Vec<PathBuf>,
        output_buffer: &mut W,
        table_name: &str,
        query: &str,
    ) -> Result<()> {
        if input_parquet_files.is_empty() {
            bail!("input parquet files cannot be empty");
        }

        let paths = {
            let mut paths = Vec::with_capacity(input_parquet_files.len());

            for path in input_parquet_files {
                let pl_path = PlRefPath::try_from_pathbuf(path)
                    .context("invalid parquet file path provided")?;

                paths.push(pl_path);
            }

            paths
        };

        let df = LazyFrame::scan_parquet_files(paths.into(), Default::default())
            .context("failed to initialize data frame from parquet files")?;

        let mut ctx = SQLContext::new();

        ctx.register(table_name, df);

        let mut df = ctx
            .execute(query)
            .with_context(|| format!("failed to apply SQL query on {table_name}"))
            .and_then(|lazy| {
                lazy.collect()
                    .context("failed to execute query on data frame")
            })?;

        ParquetWriter::new(output_buffer)
            .finish(&mut df)
            .context("failed ot write data frame to parquet")?;

        Ok(())
    }

    /// Retrieves data for provided parameters and writes out the data in a buffered manner to the
    /// provided buffer as arrow. The first column is int64, `timestamp_unix_nanos`.
    pub async fn get_data<W: Write + Send>(
        &self,
        channel_inputs: &[ChannelInput],
        time_range: TimeRange,
        sample_ms: u32,
        buffer: &mut W,
    ) -> Result<()> {
        if channel_inputs.is_empty() {
            bail!("channel inputs cannot be empty");
        }

        let mut run_id = None;

        let (start_time, end_time) = match time_range {
            TimeRange::Run {
                run,
                start_time_unix_nanos,
                end_time_unix_nanos,
            } => {
                run_id = Some(run.run_id);

                let Some(run_start) = run.start_time else {
                    bail!("provided run doesn't have a start time")
                };

                let run_end = run.stop_time.unwrap_or_else(|| {
                    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();

                    Timestamp {
                        seconds: now.as_secs() as i64,
                        nanos: now.subsec_nanos() as i32,
                    }
                });

                let start = start_time_unix_nanos
                    .map(unix_nanos_to_secs_and_subsec_nanos)
                    .map(|(seconds, nanos)| Timestamp { seconds, nanos })
                    .unwrap_or(run_start);

                let end = end_time_unix_nanos
                    .map(unix_nanos_to_secs_and_subsec_nanos)
                    .map(|(seconds, nanos)| Timestamp { seconds, nanos })
                    .unwrap_or(run_end);

                (start, end)
            }
            TimeRange::Asset {
                start_time_unix_nanos,
                end_time_unix_nanos,
            } => {
                let start = {
                    let (seconds, nanos) =
                        unix_nanos_to_secs_and_subsec_nanos(start_time_unix_nanos);
                    Timestamp { seconds, nanos }
                };

                let end = {
                    let (seconds, nanos) = unix_nanos_to_secs_and_subsec_nanos(end_time_unix_nanos);
                    Timestamp { seconds, nanos }
                };

                (start, end)
            }
        };

        let queries = channel_inputs
            .iter()
            .map(|input| match input {
                ChannelInput::Raw(channel) => Query {
                    query: Some(QueryKind::Channel(ChannelQuery {
                        channel_id: channel.channel_id.clone(),
                        run_id: run_id.clone(),
                    })),
                },
                ChannelInput::Calculation {
                    name,
                    input_channels,
                    expression,
                } => {
                    let expr_channel_refs = input_channels
                        .iter()
                        .map(|(cref, channel)| ExpressionChannelReference {
                            channel_id: channel.channel_id.clone(),
                            channel_reference: cref.clone(),
                            calculated_channel_reference: None,
                        })
                        .collect::<Vec<_>>();

                    Query {
                        query: Some(QueryKind::CalculatedChannel(CalculatedChannelQuery {
                            channel_key: name.clone(),
                            expression: Some(ExpressionRequest {
                                expression: expression.clone(),
                                expression_channel_references: expr_channel_refs,
                                ..Default::default()
                            }),
                            run_id: run_id.clone(),
                            mode: Some(ExpressionMode::CalculatedChannels.into()),
                            combine_run_data: Some(false),
                        })),
                    }
                }
            })
            .collect::<Vec<_>>();

        let mut page_token = String::new();
        let mut columns = HashMap::<ColumnName, ChannelColumn>::new();

        loop {
            let channel = self.channel.clone();
            let queries = queries.clone();
            let token = page_token.clone();
            let start_time = start_time;
            let end_time = end_time;

            let resp = with_retry(&self.policy, move || {
                let channel = channel.clone();
                let queries = queries.clone();
                let token = token.clone();
                async move {
                    let mut data_service = DataServiceClient::new(channel);
                    data_service
                        .get_data(GetDataRequest {
                            sample_ms,
                            page_token: token,
                            queries,
                            start_time: Some(start_time),
                            end_time: Some(end_time),
                            page_size: PAGE_SIZE,
                        })
                        .await
                        .map(|resp| resp.into_inner())
                }
            })
            .await
            .context("failed to get data")?;

            let GetDataResponse {
                data,
                next_page_token,
            } = resp;

            for channel_page in data {
                match channel_page.type_url.as_str() {
                    "sift.data.v2.BytesValues" => {
                        let BytesValues { metadata, values } =
                            BytesValues::decode(channel_page.value)
                                .context("failed to decode channel page")?;

                        let Some(metadata) = metadata else {
                            bail!("unexpected missing channel page metadata");
                        };

                        let Some(channel) = metadata.channel else {
                            bail!("unexpected missing channel from metadata");
                        };

                        let column_name = ColumnName::builder(&channel.name, &channel.channel_id)
                            .run(run_id.as_deref())
                            .units(channel.unit.as_ref().map(|u| u.name.as_str()))
                            .build();

                        let values = values
                            .into_iter()
                            .flat_map(|BytesValue { timestamp, value }| {
                                timestamp.map(|Timestamp { seconds, nanos }| {
                                    (
                                        secs_and_subsec_nanos_to_unix_nanos(seconds, nanos),
                                        ChannelValue::Bytes(value),
                                    )
                                })
                            })
                            .collect::<Vec<_>>();

                        if let Some(channel_column) = columns.get_mut(&column_name) {
                            channel_column.values.extend(values);
                        } else {
                            columns.insert(
                                column_name,
                                ChannelColumn {
                                    values,
                                    json_metadata: None,
                                    data_type: DataType::Binary,
                                },
                            );
                        }
                    }
                    "sift.data.v2.EnumValues" => {
                        let EnumValues { metadata, values } =
                            EnumValues::decode(channel_page.value)
                                .context("failed to decode channel page")?;

                        let Some(metadata) = metadata else {
                            bail!("unexpected missing channel page metadata");
                        };

                        let Some(channel) = metadata.channel else {
                            bail!("unexpected missing channel from metadata");
                        };

                        if channel.enum_types.is_empty() {
                            bail!("unexpected enum channel with a missing enum config")
                        }

                        let mut enum_config = HashMap::new();

                        for enum_type in &channel.enum_types {
                            enum_config.insert(enum_type.key, enum_type.name.clone());
                        }

                        let enum_config_md = serde_json::to_string(&channel.enum_types)
                            .context("failed to serialize enum config to JSON")?;

                        let column_name = ColumnName::builder(&channel.name, &channel.channel_id)
                            .run(run_id.as_deref())
                            .units(channel.unit.as_ref().map(|u| u.name.as_str()))
                            .build();

                        let values = values
                            .into_iter()
                            .flat_map(|EnumValue { timestamp, value }| {
                                timestamp.map(|Timestamp { seconds, nanos }| {
                                    (
                                        secs_and_subsec_nanos_to_unix_nanos(seconds, nanos),
                                        ChannelValue::Enum(
                                            enum_config
                                                .get(&value)
                                                .map_or_else(String::new, String::clone),
                                        ),
                                    )
                                })
                            })
                            .collect::<Vec<_>>();

                        if let Some(channel_column) = columns.get_mut(&column_name) {
                            channel_column.values.extend(values);
                        } else {
                            columns.insert(
                                column_name,
                                ChannelColumn {
                                    values,
                                    json_metadata: Some(enum_config_md),
                                    data_type: DataType::Utf8,
                                },
                            );
                        }
                    }
                    "sift.data.v2.BitFieldValues" => {
                        let BitFieldValues { metadata, values } =
                            BitFieldValues::decode(channel_page.value)
                                .context("failed to decode channel page")?;

                        let Some(metadata) = metadata else {
                            bail!("unexpected missing channel page metadata");
                        };

                        let Some(channel) = metadata.channel else {
                            bail!("unexpected missing channel from metadata");
                        };

                        if channel.bit_field_elements.is_empty() {
                            bail!(
                                "unexpected bit field channel with a missing bit field element config"
                            )
                        }

                        let bit_field_element_md = serde_json::to_string(
                            &channel.bit_field_elements,
                        )
                        .context("failed to serialize bit field element metadata to JSON")?;

                        for BitFieldElementValues { name, values } in values {
                            let column_name =
                                ColumnName::builder(&channel.name, &channel.channel_id)
                                    .bit_field_element(Some(&name))
                                    .run(run_id.as_deref())
                                    .units(channel.unit.as_ref().map(|u| u.name.as_str()))
                                    .build();

                            let values = values
                                .into_iter()
                                .flat_map(|BitFieldValue { timestamp, value }| {
                                    timestamp.map(|Timestamp { seconds, nanos }| {
                                        (
                                            secs_and_subsec_nanos_to_unix_nanos(seconds, nanos),
                                            ChannelValue::BitField(value),
                                        )
                                    })
                                })
                                .collect::<Vec<_>>();

                            if let Some(channel_column) = columns.get_mut(&column_name) {
                                channel_column.values.extend(values);
                            } else {
                                columns.insert(
                                    column_name,
                                    ChannelColumn {
                                        values,
                                        json_metadata: Some(bit_field_element_md.clone()),
                                        data_type: DataType::UInt32,
                                    },
                                );
                            }
                        }
                    }
                    "sift.data.v2.DoubleValues" => {
                        let DoubleValues { metadata, values } =
                            DoubleValues::decode(channel_page.value)
                                .context("failed to decode channel page")?;

                        let Some(metadata) = metadata else {
                            bail!("unexpected missing channel page metadata");
                        };

                        let Some(channel) = metadata.channel else {
                            bail!("unexpected missing channel from metadata");
                        };

                        let column_name = ColumnName::builder(&channel.name, &channel.channel_id)
                            .run(run_id.as_deref())
                            .units(channel.unit.as_ref().map(|u| u.name.as_str()))
                            .build();

                        let values = values
                            .into_iter()
                            .flat_map(|DoubleValue { timestamp, value }| {
                                timestamp.map(|Timestamp { seconds, nanos }| {
                                    (
                                        secs_and_subsec_nanos_to_unix_nanos(seconds, nanos),
                                        ChannelValue::F64(value),
                                    )
                                })
                            })
                            .collect::<Vec<_>>();

                        if let Some(channel_column) = columns.get_mut(&column_name) {
                            channel_column.values.extend(values);
                        } else {
                            columns.insert(
                                column_name,
                                ChannelColumn {
                                    values,
                                    json_metadata: None,
                                    data_type: DataType::Float64,
                                },
                            );
                        }
                    }
                    "sift.data.v2.FloatValues" => {
                        let FloatValues { metadata, values } =
                            FloatValues::decode(channel_page.value)
                                .context("failed to decode channel page")?;

                        let Some(metadata) = metadata else {
                            bail!("unexpected missing channel page metadata");
                        };

                        let Some(channel) = metadata.channel else {
                            bail!("unexpected missing channel from metadata");
                        };

                        let column_name = ColumnName::builder(&channel.name, &channel.channel_id)
                            .run(run_id.as_deref())
                            .units(channel.unit.as_ref().map(|u| u.name.as_str()))
                            .build();

                        let values = values
                            .into_iter()
                            .flat_map(|FloatValue { timestamp, value }| {
                                timestamp.map(|Timestamp { seconds, nanos }| {
                                    (
                                        secs_and_subsec_nanos_to_unix_nanos(seconds, nanos),
                                        ChannelValue::F32(value),
                                    )
                                })
                            })
                            .collect::<Vec<_>>();

                        if let Some(channel_column) = columns.get_mut(&column_name) {
                            channel_column.values.extend(values);
                        } else {
                            columns.insert(
                                column_name,
                                ChannelColumn {
                                    values,
                                    json_metadata: None,
                                    data_type: DataType::Float32,
                                },
                            );
                        }
                    }
                    "sift.data.v2.StringValues" => {
                        let StringValues { metadata, values } =
                            StringValues::decode(channel_page.value)
                                .context("failed to decode channel page")?;

                        let Some(metadata) = metadata else {
                            bail!("unexpected missing channel page metadata");
                        };

                        let Some(channel) = metadata.channel else {
                            bail!("unexpected missing channel from metadata");
                        };

                        let column_name = ColumnName::builder(&channel.name, &channel.channel_id)
                            .run(run_id.as_deref())
                            .units(channel.unit.as_ref().map(|u| u.name.as_str()))
                            .build();

                        let values = values
                            .into_iter()
                            .flat_map(|StringValue { timestamp, value }| {
                                timestamp.map(|Timestamp { seconds, nanos }| {
                                    (
                                        secs_and_subsec_nanos_to_unix_nanos(seconds, nanos),
                                        ChannelValue::String(value),
                                    )
                                })
                            })
                            .collect::<Vec<_>>();

                        if let Some(channel_column) = columns.get_mut(&column_name) {
                            channel_column.values.extend(values);
                        } else {
                            columns.insert(
                                column_name,
                                ChannelColumn {
                                    values,
                                    json_metadata: None,
                                    data_type: DataType::Utf8,
                                },
                            );
                        }
                    }
                    "sift.data.v2.BoolValues" => {
                        let BoolValues { metadata, values } =
                            BoolValues::decode(channel_page.value)
                                .context("failed to decode channel page")?;

                        let Some(metadata) = metadata else {
                            bail!("unexpected missing channel page metadata");
                        };

                        let Some(channel) = metadata.channel else {
                            bail!("unexpected missing channel from metadata");
                        };

                        let column_name = ColumnName::builder(&channel.name, &channel.channel_id)
                            .run(run_id.as_deref())
                            .units(channel.unit.as_ref().map(|u| u.name.as_str()))
                            .build();

                        let values = values
                            .into_iter()
                            .flat_map(|BoolValue { timestamp, value }| {
                                timestamp.map(|Timestamp { seconds, nanos }| {
                                    (
                                        secs_and_subsec_nanos_to_unix_nanos(seconds, nanos),
                                        ChannelValue::Bool(value),
                                    )
                                })
                            })
                            .collect::<Vec<_>>();

                        if let Some(channel_column) = columns.get_mut(&column_name) {
                            channel_column.values.extend(values);
                        } else {
                            columns.insert(
                                column_name,
                                ChannelColumn {
                                    values,
                                    json_metadata: None,
                                    data_type: DataType::Boolean,
                                },
                            );
                        }
                    }
                    "sift.data.v2.Int32Values" => {
                        let Int32Values { metadata, values } =
                            Int32Values::decode(channel_page.value)
                                .context("failed to decode channel page")?;

                        let Some(metadata) = metadata else {
                            bail!("unexpected missing channel page metadata");
                        };

                        let Some(channel) = metadata.channel else {
                            bail!("unexpected missing channel from metadata");
                        };

                        let column_name = ColumnName::builder(&channel.name, &channel.channel_id)
                            .run(run_id.as_deref())
                            .units(channel.unit.as_ref().map(|u| u.name.as_str()))
                            .build();

                        let values = values
                            .into_iter()
                            .flat_map(|Int32Value { timestamp, value }| {
                                timestamp.map(|Timestamp { seconds, nanos }| {
                                    (
                                        secs_and_subsec_nanos_to_unix_nanos(seconds, nanos),
                                        ChannelValue::I32(value),
                                    )
                                })
                            })
                            .collect::<Vec<_>>();

                        if let Some(channel_column) = columns.get_mut(&column_name) {
                            channel_column.values.extend(values);
                        } else {
                            columns.insert(
                                column_name,
                                ChannelColumn {
                                    values,
                                    json_metadata: None,
                                    data_type: DataType::Int32,
                                },
                            );
                        }
                    }
                    "sift.data.v2.Int64Values" => {
                        let Int64Values { metadata, values } =
                            Int64Values::decode(channel_page.value)
                                .context("failed to decode channel page")?;

                        let Some(metadata) = metadata else {
                            bail!("unexpected missing channel page metadata");
                        };

                        let Some(channel) = metadata.channel else {
                            bail!("unexpected missing channel from metadata");
                        };

                        let column_name = ColumnName::builder(&channel.name, &channel.channel_id)
                            .run(run_id.as_deref())
                            .units(channel.unit.as_ref().map(|u| u.name.as_str()))
                            .build();

                        let values = values
                            .into_iter()
                            .flat_map(|Int64Value { timestamp, value }| {
                                timestamp.map(|Timestamp { seconds, nanos }| {
                                    (
                                        secs_and_subsec_nanos_to_unix_nanos(seconds, nanos),
                                        ChannelValue::I64(value),
                                    )
                                })
                            })
                            .collect::<Vec<_>>();

                        if let Some(channel_column) = columns.get_mut(&column_name) {
                            channel_column.values.extend(values);
                        } else {
                            columns.insert(
                                column_name,
                                ChannelColumn {
                                    values,
                                    json_metadata: None,
                                    data_type: DataType::Int64,
                                },
                            );
                        }
                    }
                    "sift.data.v2.Uint32Values" => {
                        let Uint32Values { metadata, values } =
                            Uint32Values::decode(channel_page.value)
                                .context("failed to decode channel page")?;

                        let Some(metadata) = metadata else {
                            bail!("unexpected missing channel page metadata");
                        };

                        let Some(channel) = metadata.channel else {
                            bail!("unexpected missing channel from metadata");
                        };

                        let column_name = ColumnName::builder(&channel.name, &channel.channel_id)
                            .run(run_id.as_deref())
                            .units(channel.unit.as_ref().map(|u| u.name.as_str()))
                            .build();

                        let values = values
                            .into_iter()
                            .flat_map(|Uint32Value { timestamp, value }| {
                                timestamp.map(|Timestamp { seconds, nanos }| {
                                    (
                                        secs_and_subsec_nanos_to_unix_nanos(seconds, nanos),
                                        ChannelValue::U32(value),
                                    )
                                })
                            })
                            .collect::<Vec<_>>();

                        if let Some(channel_column) = columns.get_mut(&column_name) {
                            channel_column.values.extend(values);
                        } else {
                            columns.insert(
                                column_name,
                                ChannelColumn {
                                    values,
                                    json_metadata: None,
                                    data_type: DataType::UInt32,
                                },
                            );
                        }
                    }
                    "sift.data.v2.Uint64Values" => {
                        let Uint64Values { metadata, values } =
                            Uint64Values::decode(channel_page.value)
                                .context("failed to decode channel page")?;

                        let Some(metadata) = metadata else {
                            bail!("unexpected missing channel page metadata");
                        };

                        let Some(channel) = metadata.channel else {
                            bail!("unexpected missing channel from metadata");
                        };

                        let column_name = ColumnName::builder(&channel.name, &channel.channel_id)
                            .run(run_id.as_deref())
                            .units(channel.unit.as_ref().map(|u| u.name.as_str()))
                            .build();

                        let values = values
                            .into_iter()
                            .flat_map(|Uint64Value { timestamp, value }| {
                                timestamp.map(|Timestamp { seconds, nanos }| {
                                    (
                                        secs_and_subsec_nanos_to_unix_nanos(seconds, nanos),
                                        ChannelValue::U64(value),
                                    )
                                })
                            })
                            .collect::<Vec<_>>();

                        if let Some(channel_column) = columns.get_mut(&column_name) {
                            channel_column.values.extend(values);
                        } else {
                            columns.insert(
                                column_name,
                                ChannelColumn {
                                    values,
                                    json_metadata: None,
                                    data_type: DataType::UInt64,
                                },
                            );
                        }
                    }
                    _ => bail!("queried an unsupported channel type"),
                }
            }

            if next_page_token.is_empty() {
                break;
            }
            page_token = next_page_token;
        }

        if columns.is_empty() {
            bail!("no channel data for given input parameters")
        }

        let columns = columns.into_iter().collect::<Vec<_>>();

        let mut fields = vec![Field::new(TS_COLUMN_NAME, DataType::Int64, false)];

        let mut array_builders = Vec::<Box<dyn ArrayBuilder>>::new();
        let mut time_col_builder = Int64Builder::new();

        for (column_name, column) in &columns {
            let Some((_, first_val)) = column.values.first() else {
                bail!("unexpected empty column encountered")
            };

            match &column.data_type {
                DataType::Float32 => array_builders.push(Box::new(Float32Builder::new())),
                DataType::Float64 => array_builders.push(Box::new(Float64Builder::new())),
                DataType::Int32 => array_builders.push(Box::new(Int32Builder::new())),
                DataType::Int64 => array_builders.push(Box::new(Int64Builder::new())),
                DataType::UInt32 => array_builders.push(Box::new(UInt32Builder::new())),
                DataType::UInt64 => array_builders.push(Box::new(UInt64Builder::new())),
                DataType::Utf8 => array_builders.push(Box::new(StringBuilder::new())),
                DataType::Boolean => array_builders.push(Box::new(BooleanBuilder::new())),
                DataType::Binary => array_builders.push(Box::new(BinaryBuilder::new())),
                _ => bail!("unsupported column data type: {:?}", column.data_type),
            }

            let mut field = Field::new(column_name.clone(), column.data_type.clone(), true);

            if let Some(json_metadata) = column.json_metadata.as_ref() {
                match first_val {
                    ChannelValue::BitField(_) => {
                        field = field.with_metadata(HashMap::from_iter([(
                            BIT_FIELD_METADATA_KEY.to_string(),
                            json_metadata.clone(),
                        )]));
                    }
                    ChannelValue::Enum(_) => {
                        field = field.with_metadata(HashMap::from_iter([(
                            ENUM_METADATA_KEY.to_string(),
                            json_metadata.clone(),
                        )]));
                    }
                    _ => (),
                }
            }

            fields.push(field);
        }

        let schema = Arc::new(Schema::new(fields));

        let mut col_iters = columns
            .into_iter()
            .map(|(_, col)| (col.data_type, col.values.into_iter().peekable()))
            .collect::<Vec<_>>();

        let mut rows_since_flush = 0;
        let mut size_since_flush = 0;

        let mut arrow_writer = ArrowWriter::try_new(buffer, schema.clone(), None)
            .context("failed to initialize arrow writer")?;

        macro_rules! write_batch {
            () => {
                let mut columns = Vec::with_capacity(1 + array_builders.len());
                let time_column = <Int64Builder as ArrayBuilder>::finish(&mut time_col_builder);
                columns.push(time_column);

                columns.extend(array_builders.iter_mut().map(|b| b.finish()));

                let record_batch = RecordBatch::try_new(schema.clone(), columns)
                    .context("failed to create arrow record batch")?;

                arrow_writer
                    .write(&record_batch)
                    .context("failed to write arrow record batch to arrow writer")?;
            };
        }

        loop {
            let maybe_current_ts = col_iters
                .iter_mut()
                .filter_map(|(_, it)| it.peek().map(|(ts, _)| *ts))
                .min();

            let Some(current_ts) = maybe_current_ts else {
                break;
            };

            time_col_builder.append_value(current_ts);

            for (i, (data_type, col_iter)) in col_iters.iter_mut().enumerate() {
                let builder = &mut array_builders[i];

                let Some((ts, val)) = col_iter.peek() else {
                    Self::append_null_to_builder(data_type, builder)?;
                    continue;
                };

                if *ts == current_ts {
                    Self::append_to_builder(val, builder)?;
                    size_since_flush += val.data_size();

                    // Advance the iterator
                    col_iter.next();
                } else {
                    Self::append_null_to_builder(data_type, builder)?;
                }
            }
            rows_since_flush += 1;

            if rows_since_flush >= ROW_FLUSH_THRESHOLD || size_since_flush >= SIZE_FLUSH_THRESHOLD {
                write_batch!();
                rows_since_flush = 0;
                size_since_flush = 0;
            }
        }

        if rows_since_flush > 0 {
            write_batch!();
        }

        arrow_writer
            .close()
            .context("failed to finalize arrow writer")?;

        Ok(())
    }

    fn append_null_to_builder(
        data_type: &DataType,
        builder: &mut Box<dyn ArrayBuilder>,
    ) -> Result<()> {
        match data_type {
            DataType::Float32 => {
                builder
                    .as_any_mut()
                    .downcast_mut::<Float32Builder>()
                    .context("failed to downcast array builder to float32")?
                    .append_null();
            }
            DataType::Float64 => {
                builder
                    .as_any_mut()
                    .downcast_mut::<Float64Builder>()
                    .context("failed to downcast array builder to float64")?
                    .append_null();
            }
            DataType::Binary => {
                builder
                    .as_any_mut()
                    .downcast_mut::<BinaryBuilder>()
                    .context("failed to downcast array builder to binary")?
                    .append_null();
            }
            DataType::Boolean => {
                builder
                    .as_any_mut()
                    .downcast_mut::<BooleanBuilder>()
                    .context("failed to downcast array builder to boolean")?
                    .append_null();
            }
            DataType::Int32 => {
                builder
                    .as_any_mut()
                    .downcast_mut::<Int32Builder>()
                    .context("failed to downcast array builder to int32")?
                    .append_null();
            }
            DataType::Int64 => {
                builder
                    .as_any_mut()
                    .downcast_mut::<Int64Builder>()
                    .context("failed to downcast array builder to int64")?
                    .append_null();
            }
            DataType::UInt32 => {
                builder
                    .as_any_mut()
                    .downcast_mut::<UInt32Builder>()
                    .context("failed to downcast array builder to uint32")?
                    .append_null();
            }
            DataType::UInt64 => {
                builder
                    .as_any_mut()
                    .downcast_mut::<UInt64Builder>()
                    .context("failed to downcast array builder to uint64")?
                    .append_null();
            }
            DataType::Utf8 => {
                builder
                    .as_any_mut()
                    .downcast_mut::<StringBuilder>()
                    .context("failed to downcast array builder to utf8")?
                    .append_null();
            }
            _ => bail!("unsupported column data type while appending null: {data_type:?}"),
        }
        Ok(())
    }

    fn append_to_builder(
        channel_value: &ChannelValue,
        builder: &mut Box<dyn ArrayBuilder>,
    ) -> Result<()> {
        match channel_value {
            ChannelValue::F32(value) => {
                builder
                    .as_any_mut()
                    .downcast_mut::<Float32Builder>()
                    .context("failed to downcast array builder to float32")?
                    .append_value(*value);
            }
            ChannelValue::F64(value) => {
                builder
                    .as_any_mut()
                    .downcast_mut::<Float64Builder>()
                    .context("failed to downcast array builder to float64")?
                    .append_value(*value);
            }
            ChannelValue::I32(value) => {
                builder
                    .as_any_mut()
                    .downcast_mut::<Int32Builder>()
                    .context("failed to downcast array builder to int32")?
                    .append_value(*value);
            }
            ChannelValue::I64(value) => {
                builder
                    .as_any_mut()
                    .downcast_mut::<Int64Builder>()
                    .context("failed to downcast array builder to int64")?
                    .append_value(*value);
            }
            ChannelValue::U32(value) | ChannelValue::BitField(value) => {
                builder
                    .as_any_mut()
                    .downcast_mut::<UInt32Builder>()
                    .context("failed to downcast array builder to uint32")?
                    .append_value(*value);
            }
            ChannelValue::U64(value) => {
                builder
                    .as_any_mut()
                    .downcast_mut::<UInt64Builder>()
                    .context("failed to downcast array builder to uint64")?
                    .append_value(*value);
            }
            ChannelValue::String(value) | ChannelValue::Enum(value) => {
                builder
                    .as_any_mut()
                    .downcast_mut::<StringBuilder>()
                    .context("failed to downcast array builder to utf8")?
                    .append_value(value);
            }
            ChannelValue::Bool(value) => {
                builder
                    .as_any_mut()
                    .downcast_mut::<BooleanBuilder>()
                    .context("failed to downcast array builder to boolean")?
                    .append_value(*value);
            }
            ChannelValue::Bytes(value) => {
                builder
                    .as_any_mut()
                    .downcast_mut::<BinaryBuilder>()
                    .context("failed to downcast array builder to binary")?
                    .append_value(value);
            }
        }
        Ok(())
    }
}

impl ChannelValue {
    /// Byte size of underlying data that gets written to arrow
    pub fn data_size(&self) -> usize {
        match self {
            Self::F32(_) => mem::size_of::<f32>(),
            Self::F64(_) => mem::size_of::<f64>(),
            Self::I32(_) => mem::size_of::<i32>(),
            Self::I64(_) => mem::size_of::<i64>(),
            Self::U32(_) => mem::size_of::<u32>(),
            Self::U64(_) => mem::size_of::<u64>(),
            Self::String(val) => val.len() * mem::size_of::<char>(),
            Self::Bool(_) => mem::size_of::<bool>(),
            Self::Enum(val) => val.len() * mem::size_of::<char>(),
            Self::BitField(_) => mem::size_of::<u32>(),
            Self::Bytes(val) => val.len() * mem::size_of::<u8>(),
        }
    }
}
