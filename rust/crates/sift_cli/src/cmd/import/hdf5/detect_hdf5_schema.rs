use std::collections::HashMap;
use std::path::Path;

use anyhow::{Context as AnyhowContext, Result, anyhow};
use hdf5::types::{FloatSize, IntSize, TypeDescriptor, VarLenAscii, VarLenUnicode};
use hdf5::{Dataset, File, Group};
use sift_rs::{
    common::r#type::v1::{ChannelConfig, ChannelDataType, ChannelEnumType},
    data_imports::v2::Hdf5DataConfig,
};

use crate::cli::hdf5::Hdf5Schema;

const TIME_NAMES: &[&str] = &["time", "timestamp", "timestamps", "ts"];
const VALUE_NAMES: &[&str] = &["value", "values"];

pub fn basename(path: &str) -> &str {
    path.rsplit('/').next().unwrap_or(path)
}

pub fn parent_path(path: &str) -> &str {
    match path.rfind('/') {
        Some(0) => "/",
        Some(idx) => &path[..idx],
        None => "/",
    }
}

pub fn is_time_dataset_name(name: &str) -> bool {
    let leaf = basename(name).to_ascii_lowercase();
    TIME_NAMES.iter().any(|n| *n == leaf)
}

fn is_value_leaf(name: &str) -> bool {
    let leaf = basename(name).to_ascii_lowercase();
    VALUE_NAMES.iter().any(|n| *n == leaf)
}

fn collect_datasets_recursive(group: &Group) -> Result<Vec<Dataset>> {
    let mut datasets = group
        .datasets()
        .map_err(|e| anyhow!("failed to enumerate datasets in {}: {e}", group.name()))?;
    let subgroups = group
        .groups()
        .map_err(|e| anyhow!("failed to enumerate groups in {}: {e}", group.name()))?;
    for sub in &subgroups {
        datasets.extend(collect_datasets_recursive(sub)?);
    }
    Ok(datasets)
}

fn get_string_attr(ds: &Dataset, name: &str) -> Option<String> {
    let attr = ds.attr(name).ok()?;
    if let Ok(s) = attr.read_scalar::<VarLenUnicode>() {
        return Some(s.to_string());
    }
    if let Ok(s) = attr.read_scalar::<VarLenAscii>() {
        return Some(s.to_string());
    }
    None
}

pub const SUPPORTED_TYPES_BLURB: &str =
    "bool, int8/16/32/64, uint8/16/32/64, float32, float64, string, enum";

pub fn hdf5_to_sift_data_type(ty: &TypeDescriptor) -> Option<ChannelDataType> {
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
        TypeDescriptor::VarLenUnicode
        | TypeDescriptor::VarLenAscii
        | TypeDescriptor::FixedAscii(_)
        | TypeDescriptor::FixedUnicode(_) => Some(ChannelDataType::String),
        TypeDescriptor::Enum(_) => Some(ChannelDataType::Enum),
        _ => None,
    }
}

pub fn enum_types_for(ty: &TypeDescriptor) -> Vec<ChannelEnumType> {
    let TypeDescriptor::Enum(e) = ty else {
        return Vec::new();
    };
    e.members
        .iter()
        .map(|m| ChannelEnumType {
            name: m.name.clone(),
            key: m.value as u32,
            is_signed: e.signed,
        })
        .collect()
}

pub fn detect_config(
    path: &Path,
    schema: Hdf5Schema,
    time_index: u64,
    time_field: Option<&str>,
) -> Result<(Vec<Hdf5DataConfig>, Vec<ChannelConfig>)> {
    let file = File::open(path).map_err(|e| anyhow!("failed to open hdf5 file: {e}"))?;
    let datasets = collect_datasets_recursive(&file)?;

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
    let mut group_time: HashMap<String, String> = HashMap::new();
    for ds in datasets {
        let name = ds.name();
        if !is_time_dataset_name(&name) || ds.ndim() != 1 {
            continue;
        }
        group_time.entry(parent_path(&name).to_owned()).or_insert(name);
    }

    if group_time.is_empty() {
        return Err(anyhow!(
            "no time dataset found — expected one of {TIME_NAMES:?} (case-insensitive) \
             at the root or within any group"
        ));
    }

    let mut data_configs = Vec::new();
    let mut channel_configs = Vec::new();

    for ds in datasets {
        let name = ds.name();
        if is_time_dataset_name(&name) || ds.ndim() != 1 {
            continue;
        }
        let Some(time_dataset) = nearest_time_dataset(&group_time, &name) else {
            continue;
        };

        let dtype = match ds.dtype().and_then(|t| t.to_descriptor()) {
            Ok(d) => d,
            Err(e) => {
                eprintln!(
                    "skipping {name}: cannot describe HDF5 dtype ({e}). \
                     Supported types: {SUPPORTED_TYPES_BLURB}."
                );
                continue;
            }
        };
        let Some(channel_type) = hdf5_to_sift_data_type(&dtype) else {
            eprintln!(
                "skipping {name}: unsupported HDF5 type {dtype:?}. \
                 Supported types: {SUPPORTED_TYPES_BLURB}."
            );
            continue;
        };

        let units = get_string_attr(ds, "units").unwrap_or_default();
        let description = get_string_attr(ds, "long_name")
            .or_else(|| get_string_attr(ds, "description"))
            .unwrap_or_default();

        let channel_name = one_d_channel_name(&name);

        let channel_config = ChannelConfig {
            name: channel_name,
            data_type: channel_type as i32,
            units,
            description,
            enum_types: enum_types_for(&dtype),
            ..Default::default()
        };

        data_configs.push(Hdf5DataConfig {
            time_dataset,
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

fn nearest_time_dataset(group_time: &HashMap<String, String>, value_path: &str) -> Option<String> {
    let mut current = parent_path(value_path);
    loop {
        if let Some(t) = group_time.get(current) {
            return Some(t.clone());
        }
        if current == "/" {
            return None;
        }
        current = parent_path(current);
    }
}

fn one_d_channel_name(value_path: &str) -> String {
    if is_value_leaf(value_path) {
        let parent = parent_path(value_path);
        if parent != "/" {
            return parent.trim_start_matches('/').to_string();
        }
    }
    value_path.trim_start_matches('/').to_string()
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
                enum_types: enum_types_for(&dtype),
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
                enum_types: enum_types_for(&field.ty),
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
