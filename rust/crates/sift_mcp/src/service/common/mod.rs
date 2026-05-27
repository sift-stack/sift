use std::fmt;

use anyhow::{Context, bail};

#[cfg(test)]
mod test;

/// Default and max page size for list calls.
pub const PAGE_SIZE: u32 = 1000;
pub const BIT_FIELD_METADATA_KEY: &str = "bit_field_elements";
pub const ENUM_METADATA_KEY: &str = "enum_config";
pub const TS_COLUMN_NAME: &str = "timestamp_unix_nanos";

const NANOS_PER_SEC: i64 = 1_000_000_000;

/// Returns page size and limit.
pub fn paging(limit: Option<u32>) -> (u32, usize) {
    match limit {
        Some(lim) if lim <= PAGE_SIZE => (lim, lim as usize),
        _ => (PAGE_SIZE, usize::MAX),
    }
}

pub fn unix_nanos_to_secs_and_subsec_nanos(nanos: i64) -> (i64, i32) {
    let secs = nanos.div_euclid(NANOS_PER_SEC);
    let subsec_nanos = nanos.rem_euclid(NANOS_PER_SEC) as i32;
    (secs, subsec_nanos)
}

pub fn secs_and_subsec_nanos_to_unix_nanos(sec: i64, subsec_nanos: i32) -> i64 {
    sec * NANOS_PER_SEC + i64::from(subsec_nanos)
}

/// A fully-qualified Arrow column name with the channel-specific fields the
/// data pipeline cares about. `Display` emits the canonical Parquet column-name
/// string: `<name> {channel_id="...", bit_field_element="...", run="...", units="..."}`.
/// Empty optional fields are omitted. Construct via [`ColumnName::builder`] or
/// `TryFrom<String>` (when re-hydrating a string from a Parquet schema).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ColumnName {
    name: String,
    channel_id: String,
    bit_field_element: Option<String>,
    run: Option<String>,
    units: Option<String>,
}

impl ColumnName {
    pub fn builder<'a>(name: &'a str, channel_id: &'a str) -> ColumnNameBuilder<'a> {
        ColumnNameBuilder {
            name,
            channel_id,
            bit_field_element: None,
            run: None,
            units: None,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn channel_id(&self) -> &str {
        &self.channel_id
    }

    pub fn bit_field_element(&self) -> Option<&str> {
        self.bit_field_element.as_deref()
    }

    pub fn run(&self) -> Option<&str> {
        self.run.as_deref()
    }

    pub fn units(&self) -> Option<&str> {
        self.units.as_deref()
    }
}

impl fmt::Display for ColumnName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {{channel_id=\"{}\"", self.name, self.channel_id)?;
        if let Some(v) = &self.bit_field_element {
            write!(f, ", bit_field_element=\"{v}\"")?;
        }
        if let Some(v) = &self.run {
            write!(f, ", run=\"{v}\"")?;
        }
        if let Some(v) = &self.units {
            write!(f, ", units=\"{v}\"")?;
        }
        write!(f, "}}")
    }
}

impl From<ColumnName> for String {
    fn from(name: ColumnName) -> String {
        name.to_string()
    }
}

impl TryFrom<&str> for ColumnName {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.is_empty() {
            bail!("missing name: input is empty");
        }

        let (name, attrs) = value
            .split_once(" {")
            .with_context(|| format!("missing attribute block in `{value}`"))?;
        let attrs = attrs.trim_end_matches('}');

        let mut channel_id: Option<String> = None;
        let mut bit_field_element: Option<String> = None;
        let mut run: Option<String> = None;
        let mut units: Option<String> = None;

        for segment in attrs.split(',') {
            let segment = segment.trim();
            if segment.is_empty() {
                continue;
            }

            let (key, val) = segment
                .split_once('=')
                .with_context(|| format!("missing `=` in attribute segment `{segment}`"))?;

            let key = key.trim();
            if key.is_empty() {
                bail!("missing key in attribute segment `{segment}`");
            }
            let val = val
                .trim()
                .trim_start_matches('"')
                .trim_end_matches('"')
                .to_string();

            match key {
                "channel_id" => channel_id = Some(val),
                "bit_field_element" => bit_field_element = Some(val),
                "run" => run = Some(val),
                "units" => units = Some(val),
                other => bail!("unknown attribute key `{other}`"),
            }
        }

        let channel_id =
            channel_id.with_context(|| format!("missing required `channel_id` in `{value}`"))?;

        Ok(ColumnName {
            name: name.to_string(),
            channel_id,
            bit_field_element,
            run,
            units,
        })
    }
}

/// Builder for [`ColumnName`]. `name` and `channel_id` are required at
/// construction time; `bit_field_element`, `run`, and `units` are optional and
/// each silently drop empty / `None` inputs to keep [`ColumnName`]'s `Display`
/// from emitting `key=""` pairs.
pub struct ColumnNameBuilder<'a> {
    name: &'a str,
    channel_id: &'a str,
    bit_field_element: Option<String>,
    run: Option<String>,
    units: Option<String>,
}

impl<'a> ColumnNameBuilder<'a> {
    pub fn bit_field_element(mut self, value: Option<&str>) -> Self {
        self.bit_field_element = value.filter(|s| !s.is_empty()).map(str::to_string);
        self
    }

    pub fn run(mut self, value: Option<&str>) -> Self {
        self.run = value.filter(|s| !s.is_empty()).map(str::to_string);
        self
    }

    pub fn units(mut self, value: Option<&str>) -> Self {
        self.units = value.filter(|s| !s.is_empty()).map(str::to_string);
        self
    }

    pub fn build(self) -> ColumnName {
        ColumnName {
            name: self.name.to_string(),
            channel_id: self.channel_id.to_string(),
            bit_field_element: self.bit_field_element,
            run: self.run,
            units: self.units,
        }
    }
}
