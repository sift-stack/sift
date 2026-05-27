use std::{
    collections::HashMap,
    hash::{DefaultHasher, Hasher},
    sync::Arc,
};

use anyhow::{Context, Result, anyhow, bail};
use arrow::{
    array::{
        Array, BinaryArray, BooleanArray, Float32Array, Float64Array, Int32Array, Int64Array,
        RecordBatchReader, StringArray, UInt32Array, UInt64Array,
    },
    datatypes::{DataType, Field},
};
use clap::{crate_name, crate_version};
use parquet::{arrow::arrow_reader::ParquetRecordBatchReaderBuilder, file::reader::ChunkReader};
use pbjson_types::{Empty, Timestamp};
use prost::Message;
use serde::Deserialize;
use sift_rs::{
    SiftChannel,
    common::r#type::v1::{ChannelBitFieldElement, ChannelDataType, ChannelEnumType},
    ingest::v1::{
        IngestWithConfigDataChannelValue, IngestWithConfigDataStreamRequest,
        ingest_service_client::IngestServiceClient, ingest_with_config_data_channel_value::Type,
    },
    ingestion_configs::v2::{
        ChannelConfig, CreateIngestionConfigRequest, CreateIngestionConfigResponse, FlowConfig,
        ingestion_config_service_client::IngestionConfigServiceClient,
    },
    metadata::v1::MetadataValue,
    runs::v2::{CreateRunRequest, CreateRunResponse, run_service_client::RunServiceClient},
};
use tokio::sync::mpsc::channel;

use crate::service::common::{
    BIT_FIELD_METADATA_KEY, ColumnName, ENUM_METADATA_KEY, TS_COLUMN_NAME,
    unix_nanos_to_secs_and_subsec_nanos,
};

#[derive(Clone)]
pub struct IngestService {
    channel: SiftChannel,
}

#[derive(Deserialize)]
pub struct RunForm {
    pub name: String,
    pub tags: Vec<String>,
    pub metadata: Vec<MetadataValue>,
}

/// Summary of a successful `upload_dataset` call. Returned to callers so they
/// can surface server-assigned IDs (and confirm the asset/run that was used).
pub struct UploadedDataset {
    pub asset_name: String,
    pub asset_id: String,
    pub run_name: Option<String>,
    pub run_id: Option<String>,
}

/// Determines how a Parquet column's value is serialized onto the wire. Built
/// alongside the per-column [`ChannelConfig`] so the two stay in sync.
enum ColumnEncoding {
    /// Send the column value as-is via the matching `Type` variant.
    Plain,
    /// `UInt32` column whose values should be sent as `Type::BitField` bytes.
    BitField,
    /// `Utf8` column whose string values should be mapped back to `Type::Enum`
    /// integer keys using this lookup table.
    Enum { name_to_key: HashMap<String, u32> },
}

impl IngestService {
    pub fn new(channel: SiftChannel) -> Self {
        Self { channel }
    }

    /// Takes in data-sets in Parquet format generated from
    /// [crate::service::data::DataService::get_data] and [crate::service::data::DataService::sql].
    pub async fn upload_dataset<R: ChunkReader + 'static>(
        &self,
        asset: String,
        run: Option<RunForm>,
        reader: R,
    ) -> Result<UploadedDataset> {
        let mut parquet_reader = ParquetRecordBatchReaderBuilder::try_new(reader)
            .context("failed to initialize parquet reader builder")
            .and_then(|pbld| pbld.build().context("failed to build parquet reader"))?;

        let schema = parquet_reader.schema();
        let fields = schema.fields();

        if fields.len() < 2 {
            bail!("parquet file must have at least two columns")
        }

        let ts_field = &fields[0];
        if ts_field.name() != TS_COLUMN_NAME {
            bail!(
                "first column of parquet file must be `{TS_COLUMN_NAME}`, got `{}`",
                ts_field.name()
            )
        }
        if ts_field.data_type() != &DataType::Int64 {
            bail!(
                "first column `{TS_COLUMN_NAME}` must be Int64, got {:?}",
                ts_field.data_type()
            )
        }
        if ts_field.is_nullable() {
            bail!("first column `{TS_COLUMN_NAME}` must not be nullable")
        }

        let mut channel_configs = Vec::with_capacity(fields.len() - 1);
        let mut column_encodings = Vec::with_capacity(fields.len() - 1);

        let mut hasher = DefaultHasher::new();

        // Skip the time column
        for field in fields.iter().skip(1) {
            let (config, encoding) = Self::arrow_field_to_channel_config(field)?;
            let wf = config.encode_to_vec();
            hasher.write(&wf);
            channel_configs.push(config);
            column_encodings.push(encoding);
        }

        let hash = format!(
            "{}/{}/{:X}",
            crate_name!(),
            crate_version!(),
            hasher.finish()
        );

        let flow_config = FlowConfig {
            name: hash.clone(),
            channels: channel_configs,
        };

        let mut ingestion_config_service = IngestionConfigServiceClient::new(self.channel.clone());

        let ingestion_config = {
            let resp = ingestion_config_service
                .create_ingestion_config(CreateIngestionConfigRequest {
                    asset_name: asset.clone(),
                    flows: vec![flow_config],
                    ..Default::default()
                })
                .await
                .context("failed to create ingestion config")?;

            let CreateIngestionConfigResponse { ingestion_config } = resp.into_inner();

            ingestion_config.ok_or_else(|| anyhow!("expected ingestion config to be present"))?
        };

        let asset_id = ingestion_config.asset_id.clone();
        let ingestion_config_id = ingestion_config.ingestion_config_id.clone();

        let run = match run {
            None => None,
            Some(RunForm {
                name,
                tags,
                metadata,
            }) => {
                let mut run_service = RunServiceClient::new(self.channel.clone());

                let resp = run_service
                    .create_run(CreateRunRequest {
                        name,
                        tags,
                        metadata,
                        ..Default::default()
                    })
                    .await
                    .context("failed to create run")?;

                let CreateRunResponse { run } = resp.into_inner();

                run
            }
        };

        let run_name_opt = run.as_ref().map(|r| r.name.clone());
        let run_id_opt = run.map(|r| r.run_id);
        let run_id_for_stream = run_id_opt.clone().unwrap_or_default();

        let mut ingest_service = IngestServiceClient::new(self.channel.clone());
        let (tx, rx) = channel(10_000);
        let stream = tokio_stream::wrappers::ReceiverStream::new(rx);

        let producer = tokio::spawn(async move {
            let run_id = run_id_for_stream;

            for next_res in parquet_reader.by_ref() {
                let record_batch =
                    next_res.context("failed to read record batch from parquet file")?;

                let columns = record_batch.columns();
                let data_columns = &columns[1..];

                if data_columns.len() != column_encodings.len() {
                    bail!(
                        "parquet record batch has {} data columns but channel config expected {}",
                        data_columns.len(),
                        column_encodings.len(),
                    );
                }

                let time_col = columns[0]
                    .as_any()
                    .downcast_ref::<Int64Array>()
                    .ok_or_else(|| anyhow!("failed to downcast time column to int64 array"))?;

                for i in 0..record_batch.num_rows() {
                    let unix_nanos = time_col.value(i);
                    let (seconds, nanos) = unix_nanos_to_secs_and_subsec_nanos(unix_nanos);
                    let ts = Timestamp { seconds, nanos };

                    let mut channel_values = Vec::with_capacity(data_columns.len());

                    for (col, encoding) in data_columns.iter().zip(column_encodings.iter()) {
                        let val = Self::channel_value_from_arrow_array(encoding, col, i)?;
                        channel_values.push(val);
                    }

                    let req = IngestWithConfigDataStreamRequest {
                        ingestion_config_id: ingestion_config_id.clone(),
                        run_id: run_id.clone(),
                        flow: hash.clone(),
                        timestamp: Some(ts),
                        channel_values,
                        ..Default::default()
                    };

                    tx.send(req)
                        .await
                        .context("failed to write request to stream")?;
                }
            }

            Ok::<(), anyhow::Error>(())
        });

        let stream_result = ingest_service
            .ingest_with_config_data_stream(stream)
            .await
            .context("failure encountered while streaming data set to Sift");

        // Once the stream call has returned, the producer is either finished or
        // has had its `tx` dropped via the rx side closing; either way joining
        // it surfaces any error it encountered rather than silently dropping it.
        let producer_result = producer.await.context("ingest producer task panicked")?;

        stream_result?;
        producer_result?;

        Ok(UploadedDataset {
            asset_name: asset,
            asset_id,
            run_name: run_name_opt,
            run_id: run_id_opt,
        })
    }

    fn channel_value_from_arrow_array(
        encoding: &ColumnEncoding,
        acol: &Arc<dyn Array>,
        idx: usize,
    ) -> Result<IngestWithConfigDataChannelValue> {
        match acol.data_type() {
            DataType::Float32 => acol
                .as_any()
                .downcast_ref::<Float32Array>()
                .ok_or_else(|| anyhow!("failed to downcast to float32 array"))
                .map(|arr| {
                    if arr.is_null(idx) {
                        IngestWithConfigDataChannelValue {
                            r#type: Some(Type::Empty(Empty {})),
                        }
                    } else {
                        IngestWithConfigDataChannelValue {
                            r#type: Some(Type::Float(arr.value(idx))),
                        }
                    }
                }),
            DataType::Float64 => acol
                .as_any()
                .downcast_ref::<Float64Array>()
                .ok_or_else(|| anyhow!("failed to downcast to float64 array"))
                .map(|arr| {
                    if arr.is_null(idx) {
                        IngestWithConfigDataChannelValue {
                            r#type: Some(Type::Empty(Empty {})),
                        }
                    } else {
                        IngestWithConfigDataChannelValue {
                            r#type: Some(Type::Double(arr.value(idx))),
                        }
                    }
                }),
            DataType::Binary => acol
                .as_any()
                .downcast_ref::<BinaryArray>()
                .ok_or_else(|| anyhow!("failed to downcast to binary array"))
                .map(|arr| {
                    if arr.is_null(idx) {
                        IngestWithConfigDataChannelValue {
                            r#type: Some(Type::Empty(Empty {})),
                        }
                    } else {
                        IngestWithConfigDataChannelValue {
                            r#type: Some(Type::Bytes(arr.value(idx).to_vec())),
                        }
                    }
                }),
            DataType::Boolean => acol
                .as_any()
                .downcast_ref::<BooleanArray>()
                .ok_or_else(|| anyhow!("failed to downcast to boolean array"))
                .map(|arr| {
                    if arr.is_null(idx) {
                        IngestWithConfigDataChannelValue {
                            r#type: Some(Type::Empty(Empty {})),
                        }
                    } else {
                        IngestWithConfigDataChannelValue {
                            r#type: Some(Type::Bool(arr.value(idx))),
                        }
                    }
                }),
            DataType::Int32 => acol
                .as_any()
                .downcast_ref::<Int32Array>()
                .ok_or_else(|| anyhow!("failed to downcast to int32 array"))
                .map(|arr| {
                    if arr.is_null(idx) {
                        IngestWithConfigDataChannelValue {
                            r#type: Some(Type::Empty(Empty {})),
                        }
                    } else {
                        IngestWithConfigDataChannelValue {
                            r#type: Some(Type::Int32(arr.value(idx))),
                        }
                    }
                }),
            DataType::Int64 => acol
                .as_any()
                .downcast_ref::<Int64Array>()
                .ok_or_else(|| anyhow!("failed to downcast to int64 array"))
                .map(|arr| {
                    if arr.is_null(idx) {
                        IngestWithConfigDataChannelValue {
                            r#type: Some(Type::Empty(Empty {})),
                        }
                    } else {
                        IngestWithConfigDataChannelValue {
                            r#type: Some(Type::Int64(arr.value(idx))),
                        }
                    }
                }),
            DataType::UInt32 => acol
                .as_any()
                .downcast_ref::<UInt32Array>()
                .ok_or_else(|| anyhow!("failed to downcast to uint32 array"))
                .map(|arr| {
                    if arr.is_null(idx) {
                        IngestWithConfigDataChannelValue {
                            r#type: Some(Type::Empty(Empty {})),
                        }
                    } else {
                        let v = arr.value(idx);
                        let t = match encoding {
                            ColumnEncoding::BitField => Type::BitField(v.to_le_bytes().to_vec()),
                            _ => Type::Uint32(v),
                        };
                        IngestWithConfigDataChannelValue { r#type: Some(t) }
                    }
                }),
            DataType::UInt64 => acol
                .as_any()
                .downcast_ref::<UInt64Array>()
                .ok_or_else(|| anyhow!("failed to downcast to uint64 array"))
                .map(|arr| {
                    if arr.is_null(idx) {
                        IngestWithConfigDataChannelValue {
                            r#type: Some(Type::Empty(Empty {})),
                        }
                    } else {
                        IngestWithConfigDataChannelValue {
                            r#type: Some(Type::Uint64(arr.value(idx))),
                        }
                    }
                }),
            DataType::Utf8 => {
                let arr = acol
                    .as_any()
                    .downcast_ref::<StringArray>()
                    .ok_or_else(|| anyhow!("failed to downcast to utf8 array"))?;
                if arr.is_null(idx) {
                    Ok(IngestWithConfigDataChannelValue {
                        r#type: Some(Type::Empty(Empty {})),
                    })
                } else {
                    let s = arr.value(idx);
                    let t = match encoding {
                        ColumnEncoding::Enum { name_to_key } => {
                            let key = name_to_key.get(s).copied().with_context(|| {
                                format!("enum value `{s}` has no key in the channel's enum config")
                            })?;
                            Type::Enum(key)
                        }
                        _ => Type::String(s.to_string()),
                    };
                    Ok(IngestWithConfigDataChannelValue { r#type: Some(t) })
                }
            }
            _ => {
                bail!("unsupported column data type encountered while converting to channel value")
            }
        }
    }

    fn arrow_field_to_channel_config(
        field: &Arc<Field>,
    ) -> Result<(ChannelConfig, ColumnEncoding)> {
        let mut enum_config = None;
        let mut bit_field_elements = None;
        let mut encoding = ColumnEncoding::Plain;

        let column_name = ColumnName::try_from(field.name().as_str())
            .context("encountered an invalid column name in parquet file")?;

        let data_type = match field.data_type() {
            DataType::Float32 => ChannelDataType::Float,
            DataType::Float64 => ChannelDataType::Double,
            DataType::Binary => ChannelDataType::Bytes,
            DataType::Boolean => ChannelDataType::Bool,
            DataType::Int32 => ChannelDataType::Int32,
            DataType::Int64 => ChannelDataType::Int64,
            DataType::UInt32 => {
                if let Some(json) = field.metadata().get(BIT_FIELD_METADATA_KEY) {
                    let bfconf = serde_json::from_str::<Vec<ChannelBitFieldElement>>(json)
                        .context("failed to deserialize bit field element config json")?;
                    bit_field_elements = Some(bfconf);
                    encoding = ColumnEncoding::BitField;
                    ChannelDataType::BitField
                } else {
                    ChannelDataType::Uint32
                }
            }
            DataType::UInt64 => ChannelDataType::Uint64,
            DataType::Utf8 => {
                if let Some(json) = field.metadata().get(ENUM_METADATA_KEY) {
                    let econf = serde_json::from_str::<Vec<ChannelEnumType>>(json)
                        .context("failed to deserialize enum config json")?;
                    let name_to_key = econf
                        .iter()
                        .map(|e| (e.name.clone(), e.key))
                        .collect::<HashMap<_, _>>();
                    encoding = ColumnEncoding::Enum { name_to_key };
                    enum_config = Some(econf);
                    ChannelDataType::Enum
                } else {
                    ChannelDataType::String
                }
            }
            _ => bail!(
                "encountered unsupported column data type `{:?}` for column `{}`",
                field.data_type(),
                field.name(),
            ),
        };

        let config = ChannelConfig {
            name: column_name.name().to_string(),
            unit: column_name.units().unwrap_or_default().to_string(),
            data_type: data_type.into(),
            enum_types: enum_config.unwrap_or_default(),
            bit_field_elements: bit_field_elements.unwrap_or_default(),
            ..Default::default()
        };

        Ok((config, encoding))
    }
}
