use std::{
    fs::File,
    io::{self, BufReader, Read},
};

use anyhow::{Context, Result, anyhow};
use flate2::{Compression, write::GzEncoder};
use reqwest::header::{CONTENT_ENCODING, CONTENT_TYPE};
use sift_rs::common::r#type::v1::{ChannelBitFieldElement, ChannelEnumType};

use crate::{cli::time::TimeFormat, cmd::Context as CmdContext, util::api::create_rest_client};

/// Gzip and upload a file to a pre-signed upload URL with the given content type.
/// Reads from the file's current cursor position.
pub async fn upload_gzipped_file(
    ctx: &CmdContext,
    upload_url: &str,
    file: File,
    content_type: &str,
) -> Result<()> {
    let compressed_data = gzip_file(file)?;
    let rest_client = create_rest_client(ctx).context("failed to create rest client")?;

    let res = rest_client
        .post(upload_url)
        .header(CONTENT_ENCODING, "gzip")
        .header(CONTENT_TYPE, content_type)
        .body(compressed_data)
        .send()
        .await
        .context("failed to upload file")?;

    if !res.status().is_success() {
        let status = res.status();
        let text = res
            .text()
            .await
            .unwrap_or_else(|_| "<failed to read body>".into());
        return Err(anyhow!("upload failed with http status {status}: {text}"));
    }
    Ok(())
}

/// Be sure that the file's cursor is rewinded to the start before hand.
pub fn gzip_file(file: File) -> Result<Vec<u8>> {
    let mut reader = BufReader::new(file);
    let mut buffer = Vec::new();
    reader.read_to_end(&mut buffer)?;

    let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
    io::copy(&mut buffer.as_slice(), &mut encoder)?;
    let compressed_data = encoder.finish()?;
    Ok(compressed_data)
}

pub fn validate_time_format(
    time_format: TimeFormat,
    relative_start_time: &Option<String>,
) -> Result<()> {
    match time_format {
        TimeFormat::RelativeNanoseconds
        | TimeFormat::RelativeMicroseconds
        | TimeFormat::RelativeMilliseconds
        | TimeFormat::RelativeSeconds
        | TimeFormat::RelativeMinutes
        | TimeFormat::RelativeHours => {
            if relative_start_time.is_none() {
                return Err(anyhow!(
                    "--relative-start-time is required if time format is relative"
                ));
            }
            Ok(())
        }
        _ => Ok(()),
    }
}

pub fn try_parse_enum_config(val: &str) -> Result<Vec<ChannelEnumType>> {
    let values = val.split("|").collect::<Vec<&str>>();

    if values.is_empty() {
        return Err(anyhow!("blank --enum-config argument not allowed"));
    }

    let mut result = Vec::new();
    for key_value in values {
        let parts = key_value.split(",").collect::<Vec<&str>>();
        if parts.len() != 2 {
            return Err(anyhow!(
                "expected --enum-config argument to contain <key,value> pairs delimited by \"|\""
            ))
            .context(format!("bad argument: {val}"));
        }
        let key = parts[0].parse::<u32>()
            .with_context(|| format!("expected first value in comma-separated list for enum config to be a number for '{val}'"))?;
        let name = parts[1].to_string();

        result.push(ChannelEnumType {
            key,
            name,
            ..Default::default()
        })
    }
    Ok(result)
}

pub fn try_parse_bit_field_config(val: &str) -> Result<Vec<ChannelBitFieldElement>> {
    let values = val.split("|").collect::<Vec<&str>>();

    if values.is_empty() {
        return Err(anyhow!("blank --bit-field-config argument not allowed"));
    }

    let mut result = Vec::new();
    for element in values {
        let parts = element.split(",").collect::<Vec<&str>>();
        if parts.len() != 3 {
            return Err(anyhow!("expected --bit-field-config argument to contain <name,index,length> triplets delimited by \"|\""))
                .context(format!("bad argument: {val}"));
        }
        let name = parts[0].to_string();
        let index = parts[1].parse::<i32>()
            .with_context(|| format!("expected first value in comma-separated list for bit-field config to be a number for '{val}'"))?;
        let bit_count = parts[2].parse::<u32>()
            .with_context(|| format!("expected third value in comma-separated list for bit-field config to be a number for '{val}'"))?;

        result.push(ChannelBitFieldElement {
            name,
            index,
            bit_count,
        });
    }
    Ok(result)
}
