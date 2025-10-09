use anyhow::{Context, Result, format_err};
use sift_rs::common::r#type::v1::{ChannelBitFieldElement, ChannelEnumType};

pub fn try_parse_enum_config(val: &str) -> Result<Vec<ChannelEnumType>> {
    let values = val.split(",").collect::<Vec<&str>>();

    if values.is_empty() {
        return Err(format_err!("blank --enum-config argument not allowed"));
    }

    if values.len() % 2 != 0 {
        return Err(format_err!(
            "expected --enum-config value to have pair(s) but got {val}"
        ));
    }

    let mut result = Vec::new();
    for conf in values.chunks(2) {
        let key = conf[0].parse::<u32>()
            .with_context(|| format!("expected first value in comma-separated list for enum config to be a number for '{val}'"))?;
        let name = conf[1].to_string();

        result.push(ChannelEnumType { key, name, ..Default::default() })
    }
    Ok(result)
}

pub fn try_parse_bit_field_config(val: &str) -> Result<Vec<ChannelBitFieldElement>> {
    let values = val.split(",").collect::<Vec<&str>>();

    if values.is_empty() {
        return Err(format_err!("blank --bit-field-config argument not allowed"));
    }
    if values.len() % 3 != 0 {
        return Err(format_err!(
            "expected --bit-field-config to contain triplet(s) but got {val}"
        ));
    }

    let mut result = Vec::new();
    for conf in values.chunks(3) {
        let index = conf[0].parse::<i32>()
            .with_context(|| format!("expected first value in comma-separated list for bit-field config to be a number for '{val}'"))?;
        let name = conf[1].to_string();
        let bit_count = conf[2].parse::<u32>()
            .with_context(|| format!("expected third value in comma-separated list for bit-field config to be a number for '{val}'"))?;

        result.push(ChannelBitFieldElement {
            name,
            index,
            bit_count,
        });
    }
    Ok(result)
}
