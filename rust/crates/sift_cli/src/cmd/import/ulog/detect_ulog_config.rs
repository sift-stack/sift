use std::{
    collections::{BTreeSet, HashMap, HashSet},
    path::Path,
};

use anyhow::{Context as AnyhowContext, Result, anyhow, bail};
use sift_rs::common::r#type::v1::{ChannelConfig, ChannelDataType};

/// ULog magic plus the 0x01 0x12 0x35 version prefix.
const ULOG_MAGIC: [u8; 7] = [0x55, 0x4c, 0x6f, 0x67, 0x01, 0x12, 0x35];
const ULOG_HEADER_SIZE: usize = 16;
/// Header plus sync bytes used to recover record framing.
const SYNC_MESSAGE: [u8; 11] = [
    0x08, 0x00, b'S', 0x2f, 0x73, 0x13, 0x20, 0x25, 0x0c, 0xbb, 0x12,
];
const KNOWN_MESSAGE_TYPES: &[u8] = b"BFIMPQARDLCSO";
const LOG_MESSAGES_CHANNEL: &str = "log_messages";
/// ULog forbids cyclic definitions; cap recursion to avoid stack overflow.
const MAX_NESTING_DEPTH: usize = 32;

#[derive(Debug)]
pub struct Field {
    pub type_name: String,
    /// Element count for `type[N]` fields; 0 for scalars.
    pub array_size: usize,
    pub name: String,
}

#[derive(Debug)]
pub struct ScanResult {
    pub formats: HashMap<String, Vec<Field>>,
    pub subscriptions: Vec<(String, u8)>,
    pub has_untagged_logs: bool,
    pub log_tags: BTreeSet<u16>,
}

pub fn detect_config(path: &Path) -> Result<Vec<ChannelConfig>> {
    let data = std::fs::read(path).context("failed to read ulog file for preview")?;
    let scan = scan_ulog(&data)?;
    let channels = detect_ulog_channels(&scan)?;
    Ok(channels
        .into_iter()
        .map(|(name, data_type)| ChannelConfig {
            name,
            data_type: data_type.into(),
            ..Default::default()
        })
        .collect())
}

/// Scan without decoding data records. Two passes mirror the server importer:
/// definitions first, then subscriptions and logged strings across the file.
pub fn scan_ulog(data: &[u8]) -> Result<ScanResult> {
    if data.len() < ULOG_HEADER_SIZE {
        bail!("not a ULog file (invalid size)");
    }
    if data[..ULOG_MAGIC.len()] != ULOG_MAGIC {
        bail!("not a ULog file (bad magic bytes)");
    }

    let mut result = ScanResult {
        formats: collect_formats(data),
        subscriptions: Vec::new(),
        has_untagged_logs: false,
        log_tags: BTreeSet::new(),
    };
    collect_records(data, &mut result);
    Ok(result)
}

/// Collect `F` definitions until the definitions section ends.
/// Unknown records are skipped by size; truncated tails stop the scan.
fn collect_formats(data: &[u8]) -> HashMap<String, Vec<Field>> {
    let mut formats = HashMap::new();
    let mut pos = ULOG_HEADER_SIZE;

    while pos + 3 <= data.len() {
        let msg_size = u16::from_le_bytes([data[pos], data[pos + 1]]) as usize;
        let msg_type = data[pos + 2];
        if pos + 3 + msg_size > data.len() {
            break; // truncated final record
        }

        match msg_type {
            b'A' | b'L' | b'C' => break,
            b'F' => {
                if let Some((name, fields)) = parse_format(&data[pos + 3..pos + 3 + msg_size]) {
                    formats.insert(name, fields);
                }
            }
            _ => {}
        }
        pos += 3 + msg_size;
    }
    formats
}

/// Collect subscriptions and log-string channels, resyncing after lost
/// framing when possible.
fn collect_records(data: &[u8], result: &mut ScanResult) {
    let mut pos = ULOG_HEADER_SIZE;

    while pos + 3 <= data.len() {
        let msg_size = u16::from_le_bytes([data[pos], data[pos + 1]]) as usize;
        let msg_type = data[pos + 2];

        if !KNOWN_MESSAGE_TYPES.contains(&msg_type) {
            // Resync at the next sync marker.
            let Some(idx) = find_subslice(&data[pos..], &SYNC_MESSAGE) else {
                break;
            };
            pos += idx;
            continue;
        }

        if pos + 3 + msg_size > data.len() {
            break; // truncated final record
        }
        let payload = &data[pos + 3..pos + 3 + msg_size];

        match msg_type {
            // Add subscription: multi_id[1], msg_id[2], message_name.
            b'A' if msg_size >= 4 => {
                let name = String::from_utf8_lossy(&payload[3..]).into_owned();
                result.subscriptions.push((name, payload[0]));
            }
            b'L' => result.has_untagged_logs = true,
            // Tagged log string: log_level[1], tag[2], ...
            b'C' if msg_size >= 3 => {
                result
                    .log_tags
                    .insert(u16::from_le_bytes([payload[1], payload[2]]));
            }
            _ => {}
        }
        pos += 3 + msg_size;
    }
}

/// Return importable `(channel_key, data_type)` pairs in file order.
/// Topic channels use `<message>_<multi_id>.<field>`; logs use
/// `log_messages[_<tag>]`.
pub fn detect_ulog_channels(scan: &ScanResult) -> Result<Vec<(String, ChannelDataType)>> {
    let mut seen = HashSet::new();
    let mut channels = Vec::new();

    for (message_name, multi_id) in &scan.subscriptions {
        // Malformed definitions can leave a subscription without a format.
        let Some(fields) = scan.formats.get(message_name) else {
            continue;
        };
        // No top-level timestamp means no usable time axis.
        if !fields.iter().any(|f| f.name == "timestamp") {
            continue;
        }
        for (key, data_type) in expand_message_fields(&scan.formats, message_name)? {
            // `timestamp` is the time axis; `_padding` fields are alignment bytes.
            if key == "timestamp" || is_padding(&key) {
                continue;
            }
            let channel = format!("{message_name}_{multi_id}.{key}");
            if seen.insert(channel.clone()) {
                channels.push((channel, data_type));
            }
        }
    }

    if scan.has_untagged_logs && seen.insert(LOG_MESSAGES_CHANNEL.to_string()) {
        channels.push((LOG_MESSAGES_CHANNEL.to_string(), ChannelDataType::String));
    }
    for tag in &scan.log_tags {
        let channel = format!("{LOG_MESSAGES_CHANNEL}_{tag}");
        if seen.insert(channel.clone()) {
            channels.push((channel, ChannelDataType::String));
        }
    }
    Ok(channels)
}

/// Flatten a message format into `(field_key, data_type)` leaf entries.
/// Arrays expand to `field[i]`, nested messages use dotted paths, and both
/// `char` and `char[N]` collapse to a single string field.
pub fn expand_message_fields(
    formats: &HashMap<String, Vec<Field>>,
    message_name: &str,
) -> Result<Vec<(String, ChannelDataType)>> {
    let mut flattened = Vec::new();
    walk_fields(formats, message_name, "", 0, &mut flattened)?;
    Ok(flattened)
}

fn walk_fields(
    formats: &HashMap<String, Vec<Field>>,
    type_name: &str,
    prefix: &str,
    depth: usize,
    out: &mut Vec<(String, ChannelDataType)>,
) -> Result<()> {
    if depth > MAX_NESTING_DEPTH {
        bail!("message formats nest deeper than {MAX_NESTING_DEPTH} levels (cyclic format?)");
    }
    let fields = formats
        .get(type_name)
        .ok_or_else(|| anyhow!("format references unknown message type: {type_name}"))?;

    for field in fields {
        if field.type_name == "char" {
            out.push((format!("{prefix}{}", field.name), ChannelDataType::String));
        } else if let Some(data_type) = ulog_to_sift_data_type(&field.type_name) {
            if field.array_size > 0 {
                out.extend(
                    (0..field.array_size)
                        .map(|i| (format!("{prefix}{}[{i}]", field.name), data_type)),
                );
            } else {
                out.push((format!("{prefix}{}", field.name), data_type));
            }
        } else if field.array_size > 0 {
            for i in 0..field.array_size {
                let nested_prefix = format!("{prefix}{}[{i}].", field.name);
                walk_fields(formats, &field.type_name, &nested_prefix, depth + 1, out)?;
            }
        } else {
            let nested_prefix = format!("{prefix}{}.", field.name);
            walk_fields(formats, &field.type_name, &nested_prefix, depth + 1, out)?;
        }
    }
    Ok(())
}

/// Map a ULog C scalar type to a Sift channel type. Smaller ints widen to
/// 32-bit, and char fields import as strings.
pub fn ulog_to_sift_data_type(c_type: &str) -> Option<ChannelDataType> {
    match c_type {
        "int8_t" | "int16_t" | "int32_t" => Some(ChannelDataType::Int32),
        "int64_t" => Some(ChannelDataType::Int64),
        "uint8_t" | "uint16_t" | "uint32_t" => Some(ChannelDataType::Uint32),
        "uint64_t" => Some(ChannelDataType::Uint64),
        "float" => Some(ChannelDataType::Float),
        "double" => Some(ChannelDataType::Double),
        "bool" => Some(ChannelDataType::Bool),
        "char" => Some(ChannelDataType::String),
        _ => None,
    }
}

/// Parse an `F` payload: `message_name:type field;type field;...`.
/// Matches the server importer: malformed payloads are skipped, invalid UTF-8
/// bytes are dropped, only the first field segment is read, extra field tokens
/// are ignored, and negative array sizes become scalars.
fn parse_format(payload: &[u8]) -> Option<(String, Vec<Field>)> {
    let text = String::from_utf8_lossy(payload).replace('\u{fffd}', "");
    let mut parts = text.split(':');
    let name = parts.next()?;
    let field_list = parts.next()?;

    let mut fields = Vec::new();
    for entry in field_list.split(';').filter(|entry| !entry.is_empty()) {
        let mut tokens = entry.split(' ');
        let type_str = tokens.next()?;
        let field_name = tokens.next()?;
        let (type_name, array_size) = match type_str.split_once('[') {
            Some((base, rest)) => {
                let count: i64 = rest.split(']').next()?.parse().ok()?;
                (base, usize::try_from(count).unwrap_or(0))
            }
            None => (type_str, 0),
        };
        fields.push(Field {
            type_name: type_name.to_string(),
            array_size,
            name: field_name.to_string(),
        });
    }
    Some((name.to_string(), fields))
}

fn is_padding(channel_key: &str) -> bool {
    channel_key
        .split('.')
        .any(|segment| segment.starts_with("_padding"))
}

fn find_subslice(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    haystack
        .windows(needle.len())
        .position(|window| window == needle)
}
