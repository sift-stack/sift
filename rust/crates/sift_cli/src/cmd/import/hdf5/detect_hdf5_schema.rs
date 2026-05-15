use std::path::Path;

use anyhow::{Context as AnyhowContext, Result, anyhow};
use hdf5::types::{FloatSize, IntSize, TypeDescriptor};
use hdf5::{Dataset, File};
use sift_rs::{
    common::r#type::v1::{ChannelConfig, ChannelDataType},
    data_imports::v2::Hdf5DataConfig,
};

use crate::cli::hdf5::Hdf5Schema;

const TIME_NAMES: &[&str] = &["time", "timestamp", "timestamps", "ts"];

pub(super) fn is_time_dataset_name(name: &str) -> bool {
    let trimmed = name.trim_start_matches('/').to_ascii_lowercase();
    TIME_NAMES.iter().any(|n| *n == trimmed)
}

/// Supported HDF5 channel types. Anything outside this set is rejected with a
/// client-side error so users get clear feedback before upload.
pub(super) const SUPPORTED_TYPES_BLURB: &str =
    "bool, int8/16/32/64, uint8/16/32/64, float32, float64";

pub(super) fn hdf5_to_sift_data_type(ty: &TypeDescriptor) -> Option<ChannelDataType> {
    match ty {
        TypeDescriptor::Boolean => Some(ChannelDataType::Bool),
        TypeDescriptor::Integer(IntSize::U1)
        | TypeDescriptor::Integer(IntSize::U2)
        | TypeDescriptor::Integer(IntSize::U4) => Some(ChannelDataType::Int32),
        TypeDescriptor::Integer(IntSize::U8) => Some(ChannelDataType::Int64),
        TypeDescriptor::Unsigned(IntSize::U1)
        | TypeDescriptor::Unsigned(IntSize::U2)
        | TypeDescriptor::Unsigned(IntSize::U4) => Some(ChannelDataType::Uint32),
        TypeDescriptor::Unsigned(IntSize::U8) => Some(ChannelDataType::Uint64),
        TypeDescriptor::Float(FloatSize::U4) => Some(ChannelDataType::Float),
        TypeDescriptor::Float(FloatSize::U8) => Some(ChannelDataType::Double),
        _ => None,
    }
}

pub(super) fn detect_config(
    path: &Path,
    schema: Hdf5Schema,
    time_index: u64,
    time_field: Option<&str>,
) -> Result<(Vec<Hdf5DataConfig>, Vec<ChannelConfig>)> {
    let file = File::open(path).map_err(|e| anyhow!("failed to open hdf5 file: {e}"))?;
    let datasets = file
        .datasets()
        .map_err(|e| anyhow!("failed to enumerate datasets: {e}"))?;

    let result = match schema {
        Hdf5Schema::OneD => detect_one_d(&datasets),
        Hdf5Schema::TwoD => detect_two_d(&datasets, time_index),
        Hdf5Schema::Compound => detect_compound(&datasets, time_index, time_field),
    };

    match result {
        Ok((data, _)) if data.is_empty() => {
            Err(no_match_error(&datasets, schema, time_index, time_field))
        }
        Ok(other) => Ok(other),
        Err(e) => Err(e),
    }
}

fn no_match_error(
    datasets: &[Dataset],
    selected: Hdf5Schema,
    time_index: u64,
    time_field: Option<&str>,
) -> anyhow::Error {
    let alternatives: &[(Hdf5Schema, &str)] = &[
        (Hdf5Schema::OneD, "one-d"),
        (Hdf5Schema::TwoD, "two-d"),
        (Hdf5Schema::Compound, "compound"),
    ];

    let suggestions: Vec<&str> = alternatives
        .iter()
        .filter(|(s, _)| *s != selected)
        .filter_map(|(s, name)| {
            let probe = match s {
                Hdf5Schema::OneD => detect_one_d(datasets),
                Hdf5Schema::TwoD => detect_two_d(datasets, time_index),
                Hdf5Schema::Compound => detect_compound(datasets, time_index, time_field),
            };
            match probe {
                Ok((data, _)) if !data.is_empty() => Some(*name),
                _ => None,
            }
        })
        .collect();

    let selected_label = match selected {
        Hdf5Schema::OneD => "one-d",
        Hdf5Schema::TwoD => "two-d",
        Hdf5Schema::Compound => "compound",
    };

    if suggestions.is_empty() {
        anyhow!(
            "no datasets matched --schema {selected_label}. \
             Verify the file contains data matching the selected schema."
        )
    } else {
        anyhow!(
            "no datasets matched --schema {selected_label}. \
             Did you mean --schema {}?",
            suggestions.join(" or --schema ")
        )
    }
}

fn detect_one_d(datasets: &[Dataset]) -> Result<(Vec<Hdf5DataConfig>, Vec<ChannelConfig>)> {
    let time_dataset = datasets
        .iter()
        .find(|d| is_time_dataset_name(&d.name()))
        .map(|d| d.name())
        .ok_or_else(|| {
            anyhow!("no time dataset found — expected one of {TIME_NAMES:?} (case-insensitive)")
        })?;

    let mut data_configs = Vec::new();
    let mut channel_configs = Vec::new();

    for ds in datasets {
        let name = ds.name();
        if name == time_dataset {
            continue;
        }
        if ds.ndim() != 1 {
            continue;
        }
        let dtype = ds
            .dtype()
            .map_err(|e| anyhow!("failed to read dtype for {name}: {e}"))?
            .to_descriptor()
            .map_err(|e| anyhow!("failed to describe dtype for {name}: {e}"))?;
        let Some(channel_type) = hdf5_to_sift_data_type(&dtype) else {
            return Err(anyhow!(
                "unsupported HDF5 type for dataset {name}: {dtype:?}. \
                 Supported types: {SUPPORTED_TYPES_BLURB}."
            ));
        };

        let channel_config = ChannelConfig {
            name: name.trim_start_matches('/').to_string(),
            data_type: channel_type as i32,
            ..Default::default()
        };

        data_configs.push(Hdf5DataConfig {
            time_dataset: time_dataset.clone(),
            time_index: 0,
            value_dataset: name.clone(),
            value_index: 0,
            channel_config: Some(channel_config.clone()),
            time_field: None,
            value_field: None,
        });
        channel_configs.push(channel_config);
    }

    Ok((data_configs, channel_configs))
}

fn detect_two_d(
    datasets: &[Dataset],
    time_index: u64,
) -> Result<(Vec<Hdf5DataConfig>, Vec<ChannelConfig>)> {
    let mut data_configs = Vec::new();
    let mut channel_configs = Vec::new();

    for ds in datasets {
        let name = ds.name();
        if ds.ndim() != 2 {
            continue;
        }
        let shape = ds.shape();
        let n_cols = shape.get(1).copied().unwrap_or(0) as u64;
        if time_index >= n_cols {
            return Err(anyhow!(
                "--time-index {time_index} out of range for dataset {name} with {n_cols} columns"
            ));
        }

        let dtype = ds
            .dtype()
            .map_err(|e| anyhow!("failed to read dtype for {name}: {e}"))?
            .to_descriptor()
            .map_err(|e| anyhow!("failed to describe dtype for {name}: {e}"))?;
        let Some(channel_type) = hdf5_to_sift_data_type(&dtype) else {
            return Err(anyhow!(
                "unsupported HDF5 type for dataset {name}: {dtype:?}. \
                 Supported types: {SUPPORTED_TYPES_BLURB}."
            ));
        };

        for col in 0..n_cols {
            if col == time_index {
                continue;
            }
            let channel_name = format!("{}.{col}", name.trim_start_matches('/'));
            let channel_config = ChannelConfig {
                name: channel_name,
                data_type: channel_type as i32,
                ..Default::default()
            };

            data_configs.push(Hdf5DataConfig {
                time_dataset: name.clone(),
                time_index,
                value_dataset: name.clone(),
                value_index: col,
                channel_config: Some(channel_config.clone()),
                time_field: None,
                value_field: None,
            });
            channel_configs.push(channel_config);
        }
    }

    Ok((data_configs, channel_configs))
}

fn detect_compound(
    datasets: &[Dataset],
    time_index: u64,
    time_field: Option<&str>,
) -> Result<(Vec<Hdf5DataConfig>, Vec<ChannelConfig>)> {
    let mut data_configs = Vec::new();
    let mut channel_configs = Vec::new();

    for ds in datasets {
        let name = ds.name();
        let dtype = ds
            .dtype()
            .map_err(|e| anyhow!("failed to read dtype for {name}: {e}"))?
            .to_descriptor()
            .map_err(|e| anyhow!("failed to describe dtype for {name}: {e}"))?;
        let TypeDescriptor::Compound(compound) = dtype else {
            continue;
        };

        let resolved_time_field = match time_field {
            Some(name) => compound
                .fields
                .iter()
                .find(|f| f.name == name)
                .with_context(|| {
                    format!("--time-field '{name}' not found in dataset {}", ds.name())
                })?,
            None => {
                let idx = time_index as usize;
                compound.fields.get(idx).with_context(|| {
                    format!(
                        "--time-index {time_index} out of range for dataset {} with {} fields",
                        ds.name(),
                        compound.fields.len()
                    )
                })?
            }
        };

        for field in &compound.fields {
            if field.name == resolved_time_field.name {
                continue;
            }
            let Some(channel_type) = hdf5_to_sift_data_type(&field.ty) else {
                return Err(anyhow!(
                    "unsupported HDF5 type for field {}.{}: {:?}. \
                     Supported types: {SUPPORTED_TYPES_BLURB}.",
                    name,
                    field.name,
                    field.ty
                ));
            };
            let channel_name = format!("{}.{}", name.trim_start_matches('/'), field.name);
            let channel_config = ChannelConfig {
                name: channel_name,
                data_type: channel_type as i32,
                ..Default::default()
            };

            data_configs.push(Hdf5DataConfig {
                time_dataset: name.clone(),
                time_index: 0,
                value_dataset: name.clone(),
                value_index: 0,
                channel_config: Some(channel_config.clone()),
                time_field: Some(resolved_time_field.name.clone()),
                value_field: Some(field.name.clone()),
            });
            channel_configs.push(channel_config);
        }
    }

    Ok((data_configs, channel_configs))
}
