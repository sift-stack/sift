use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{Read, Seek},
    path::Path,
};

use anyhow::{Context as AnyhowContext, Result, anyhow};
use sift_rs::common::r#type::v1::ChannelDataType;

use crate::cmd::import::mcap::ros2_schema::{LeafField, expand_message_fields, parse_schema_defs};

const MCAP_MAGIC: [u8; 8] = [0x89, b'M', b'C', b'A', b'P', 0x30, b'\r', b'\n'];

/// Chunk compressions Sift can read.
const SUPPORTED_COMPRESSIONS: [&str; 3] = ["", "zstd", "lz4"];

/// The only message and schema encodings the importer decodes.
const MESSAGE_ENCODING: &str = "cdr";
const SCHEMA_ENCODING: &str = "ros2msg";

/// One Sift channel the import creates.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DetectedChannel {
    pub topic: String,
    pub field_path: String,
    /// Default Sift channel name, `<topic>.<field path>`.
    pub name: String,
    pub data_type: ChannelDataType,
    /// Variable-cardinality field, so `complex_types_import_mode` decides what
    /// channels it becomes.
    pub complex: bool,
}

#[derive(Debug, Default)]
pub struct Detection {
    pub channels: Vec<DetectedChannel>,
    /// What the scan could not read or had to skip.
    pub warnings: Vec<String>,
}

#[derive(Debug, Clone)]
struct SchemaInfo {
    name: String,
    encoding: String,
    data: Vec<u8>,
}

#[derive(Debug, Clone)]
struct ChannelInfo {
    id: u16,
    topic: String,
    message_encoding: String,
    /// 0 means the channel has no schema.
    schema_id: u16,
}

#[derive(Debug, Default)]
struct Scan {
    schemas: HashMap<u16, SchemaInfo>,
    /// In first-seen order, deduplicated by channel id.
    channels: Vec<ChannelInfo>,
    seen_channel_ids: HashSet<u16>,
    warnings: Vec<String>,
}

impl Scan {
    fn add_channel(&mut self, channel: ChannelInfo) {
        if self.seen_channel_ids.insert(channel.id) {
            self.channels.push(channel);
        }
    }

    fn warn(&mut self, message: String) {
        if !self.warnings.contains(&message) {
            self.warnings.push(message);
        }
    }
}

fn unsupported_compression(compression: &str) -> anyhow::Error {
    anyhow!(
        "unsupported chunk compression '{compression}'; supported compressions are \
         none, zstd, and lz4"
    )
}

/// Detects the Sift channels an MCAP file imports as, reading its schema and
/// channel records without decoding message payloads.
pub fn detect_config(path: &Path) -> Result<Detection> {
    let mut file = File::open(path).context("failed to open mcap file for preview")?;

    let mut magic = [0u8; MCAP_MAGIC.len()];
    file.read_exact(&mut magic)
        .ok()
        .filter(|_| magic == MCAP_MAGIC)
        .with_context(|| format!("'{}' is not an MCAP file (bad magic bytes)", path.display()))?;

    let file_size = file
        .metadata()
        .context("failed to read mcap file size")?
        .len();

    let mut scan = Scan::default();
    collect_schemas_and_channels(&mut file, file_size, &mut scan)?;
    let topics = detect_topics(&mut scan);
    let channels = build_channels(topics, &mut scan);

    Ok(Detection {
        channels,
        warnings: scan.warnings,
    })
}

/// Reads the file's schema and channel records from the summary section,
/// falling back to a scan of the data section when the summary is missing or
/// does not fully describe the file's channels.
fn collect_schemas_and_channels(file: &mut File, file_size: u64, scan: &mut Scan) -> Result<()> {
    let summary = match summary_section(file, file_size) {
        Ok(summary) => summary,
        Err(e) => {
            scan.warn(format!(
                "the summary section could not be read, so the file's data section was \
                 scanned instead: {e}"
            ));
            None
        }
    };

    let mut scan_data_section = true;

    if let Some(summary) = summary {
        for (id, schema) in &summary.schemas {
            scan.schemas.insert(
                *id,
                SchemaInfo {
                    name: schema.name.clone(),
                    encoding: schema.encoding.clone(),
                    data: schema.data.to_vec(),
                },
            );
        }

        let mut channels: Vec<_> = summary.channels.values().collect();
        channels.sort_by_key(|c| c.id);
        for channel in channels {
            scan.add_channel(ChannelInfo {
                id: channel.id,
                topic: channel.topic.clone(),
                message_encoding: channel.message_encoding.clone(),
                schema_id: channel.schema.as_ref().map_or(0, |s| s.id),
            });
        }

        // An unknown compression would be read as uncompressed garbage.
        for chunk_index in &summary.chunk_indexes {
            if !SUPPORTED_COMPRESSIONS.contains(&chunk_index.compression.as_str()) {
                return Err(unsupported_compression(&chunk_index.compression));
            }
        }

        let attachment_count = summary
            .stats
            .as_ref()
            .map_or(summary.attachment_indexes.len() as u32, |s| {
                s.attachment_count
            });
        if attachment_count > 0 {
            scan.warn(format!(
                "the file has {attachment_count} attachment(s); attachments are not imported"
            ));
        }

        // Repeating schema and channel records in the summary is optional, so
        // what it holds may be partial. Statistics covering at least one
        // message means the summary is trustworthy; without one, read the
        // data section.
        let have_messages = summary.stats.as_ref().is_some_and(|s| s.message_count > 0);
        scan_data_section = scan.channels.is_empty() || !have_messages;
    }

    if scan_data_section {
        scan_records(file, scan)?;
    }
    Ok(())
}

/// Drives the summary reader over `file`, seeking rather than reading the whole
/// file so large recordings stay cheap to preview.
fn summary_section(file: &mut File, file_size: u64) -> Result<Option<mcap::Summary>> {
    use mcap::sans_io::{SummaryReadEvent, SummaryReader, SummaryReaderOptions};

    file.rewind().context("failed to seek in mcap file")?;
    let mut reader =
        SummaryReader::new_with_options(SummaryReaderOptions::default().with_file_size(file_size));

    while let Some(event) = reader.next_event() {
        match event? {
            SummaryReadEvent::ReadRequest(n) => {
                let read = file.read(reader.insert(n))?;
                reader.notify_read(read);
            }
            SummaryReadEvent::SeekRequest(to) => {
                let pos = file.seek(to)?;
                reader.notify_seeked(pos);
            }
        }
    }
    Ok(reader.finish())
}

/// Scans the file's records from the start, decompressing chunks so schema and
/// channel records written inside them are seen.
fn scan_records(file: &mut File, scan: &mut Scan) -> Result<()> {
    use mcap::{
        records::Record,
        sans_io::{LinearReadEvent, LinearReader, LinearReaderOptions},
    };

    file.rewind().context("failed to seek in mcap file")?;
    // Tolerate a missing end magic so a truncated recording still yields what
    // it managed to read.
    let mut reader = LinearReader::new_with_options(
        LinearReaderOptions::default()
            .with_skip_end_magic(true)
            .with_validate_chunk_crcs(true),
    );

    loop {
        let Some(event) = reader.next_event() else {
            return Ok(());
        };
        let event = match event {
            Err(mcap::McapError::UnsupportedCompression(compression)) => {
                return Err(unsupported_compression(&compression));
            }
            Err(e) => {
                scan.warn(format!(
                    "stopped reading at an unparseable record; the detected channels may \
                     be incomplete: {e}"
                ));
                return Ok(());
            }
            Ok(event) => event,
        };

        match event {
            LinearReadEvent::ReadRequest(n) => {
                let read = file
                    .read(reader.insert(n))
                    .context("failed to read mcap file")?;
                reader.notify_read(read);
            }
            LinearReadEvent::Record { data, opcode } => {
                let record = match mcap::parse_record(opcode, data) {
                    Ok(record) => record,
                    Err(e) => {
                        scan.warn(format!(
                            "stopped reading at an unparseable record; the detected \
                             channels may be incomplete: {e}"
                        ));
                        return Ok(());
                    }
                };
                match record {
                    Record::Schema { header, data } => {
                        scan.schemas.insert(
                            header.id,
                            SchemaInfo {
                                name: header.name,
                                encoding: header.encoding,
                                data: data.to_vec(),
                            },
                        );
                    }
                    Record::Channel(channel) => scan.add_channel(ChannelInfo {
                        id: channel.id,
                        topic: channel.topic,
                        message_encoding: channel.message_encoding,
                        schema_id: channel.schema_id,
                    }),
                    _ => (),
                }
            }
        }
    }
}

/// A supported topic and the leaves it imports.
struct TopicInfo {
    topic: String,
    leaves: Vec<LeafField>,
}

/// Derives the supported topics and their importable leaves, warning about
/// every topic it had to skip.
fn detect_topics(scan: &mut Scan) -> Vec<TopicInfo> {
    // Preserve first-seen topic order.
    let mut topic_order: Vec<String> = Vec::new();
    let mut channels_by_topic: HashMap<String, Vec<ChannelInfo>> = HashMap::new();
    for channel in &scan.channels {
        channels_by_topic
            .entry(channel.topic.clone())
            .or_insert_with(|| {
                topic_order.push(channel.topic.clone());
                Vec::new()
            })
            .push(channel.clone());
    }

    // Sift channel names compare case-insensitively, so distinct topics
    // colliding only by case conflict; the first wins.
    let mut kept_by_lower: HashMap<String, String> = HashMap::new();
    let mut collisions: Vec<(String, String)> = Vec::new();
    for topic in &topic_order {
        let first = kept_by_lower
            .entry(topic.to_lowercase())
            .or_insert_with(|| topic.clone());
        if first != topic {
            collisions.push((topic.clone(), first.clone()));
        }
    }
    for (topic, first) in collisions {
        scan.warn(format!(
            "topic '{topic}' collides with topic '{first}' by case only; kept the first. \
             Pass --parse-error-policy ignore-error to import it and skip the rest, \
             otherwise the import fails"
        ));
    }

    let mut topics = Vec::new();
    let mut unsupported: Vec<(String, String)> = Vec::new();

    for topic in &topic_order {
        if kept_by_lower.get(&topic.to_lowercase()) != Some(topic) {
            continue;
        }
        let topic_channels = &channels_by_topic[topic];

        // Channels sharing a topic merge only when their schemas and message
        // encodings agree.
        let encodings: HashSet<&str> = topic_channels
            .iter()
            .map(|c| c.message_encoding.as_str())
            .collect();
        let signatures: HashSet<Option<(&str, &str, &[u8])>> = topic_channels
            .iter()
            .map(|c| {
                scan.schemas
                    .get(&c.schema_id)
                    .map(|s| (s.name.as_str(), s.encoding.as_str(), s.data.as_slice()))
            })
            .collect();
        if encodings.len() > 1 || signatures.len() > 1 {
            unsupported.push((
                topic.clone(),
                "it has multiple channels with mismatched schemas or message encodings".into(),
            ));
            continue;
        }

        let channel = &topic_channels[0];
        let Some(schema) = scan.schemas.get(&channel.schema_id) else {
            unsupported.push((topic.clone(), "it has no schema".into()));
            continue;
        };
        if channel.message_encoding != MESSAGE_ENCODING {
            unsupported.push((
                topic.clone(),
                format!(
                    "its message encoding is '{}' (only {MESSAGE_ENCODING} is supported)",
                    channel.message_encoding
                ),
            ));
            continue;
        }
        if schema.encoding != SCHEMA_ENCODING {
            unsupported.push((
                topic.clone(),
                format!(
                    "its schema encoding is '{}' (only {SCHEMA_ENCODING} is supported)",
                    schema.encoding
                ),
            ));
            continue;
        }

        let schema_text = match std::str::from_utf8(&schema.data) {
            Ok(text) => text,
            Err(e) => {
                unsupported.push((
                    topic.clone(),
                    format!("its schema is not valid UTF-8 ({e})"),
                ));
                continue;
            }
        };

        match parse_schema_defs(&schema.name, schema_text)
            .and_then(|(root, msgdefs)| expand_message_fields(&root, &msgdefs))
        {
            Ok(leaves) => topics.push(TopicInfo {
                topic: topic.clone(),
                leaves,
            }),
            Err(e) => unsupported.push((topic.clone(), e.to_string())),
        }
    }

    if !unsupported.is_empty() {
        unsupported.sort();
        let details = unsupported
            .iter()
            .map(|(topic, reason)| format!("'{topic}': {reason}"))
            .collect::<Vec<_>>()
            .join("; ");
        scan.warn(format!("skipped unsupported topics: {details}"));
    }
    topics
}

/// Returns one channel per leaf field, named `<topic>.<field path>`.
fn build_channels(topics: Vec<TopicInfo>, scan: &mut Scan) -> Vec<DetectedChannel> {
    let mut channels = Vec::new();
    // Sift channel names are unique per asset and compare case-insensitively.
    let mut taken_names: HashMap<String, (String, String)> = HashMap::new();

    for topic in topics {
        for leaf in topic.leaves {
            let name = format!("{}.{}", topic.topic, leaf.field_path);
            if let Some((first_topic, first_path)) = taken_names.get(&name.to_lowercase()) {
                scan.warn(format!(
                    "two channels are both named '{name}': topic '{}' field '{}' and topic \
                     '{first_topic}' field '{first_path}'; listed the first only",
                    topic.topic, leaf.field_path
                ));
                continue;
            }
            taken_names.insert(
                name.to_lowercase(),
                (topic.topic.clone(), leaf.field_path.clone()),
            );
            channels.push(DetectedChannel {
                topic: topic.topic.clone(),
                field_path: leaf.field_path,
                name,
                data_type: leaf.data_type,
                complex: leaf.complex,
            });
        }
    }
    channels
}
