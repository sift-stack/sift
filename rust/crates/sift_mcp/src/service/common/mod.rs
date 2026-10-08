use std::{
    collections::{HashMap, HashSet},
    fmt,
};

use anyhow::{Context, bail};

#[cfg(test)]
mod test;

/// Maximum page size and record limit for list calls.
pub const PAGE_SIZE: u32 = 200;
/// Record limit applied when a caller omits `limit`. Matches the starting limit
/// the `list_*` tool descriptions advise, so a caller that ignores that advice
/// cannot trigger an unbounded query.
pub const DEFAULT_LIMIT: u32 = 50;
pub const BIT_FIELD_METADATA_KEY: &str = "bit_field_elements";
pub const ENUM_METADATA_KEY: &str = "enum_config";
pub const TS_COLUMN_NAME: &str = "timestamp_unix_nanos";
/// Original channel label, before it was sanitized into a SQL identifier.
pub const CHANNEL_NAME_METADATA_KEY: &str = "channel_name";
pub const CHANNEL_ID_METADATA_KEY: &str = "channel_id";
pub const RUN_METADATA_KEY: &str = "run";
pub const UNITS_METADATA_KEY: &str = "units";
pub const BIT_FIELD_ELEMENT_METADATA_KEY: &str = "bit_field_element";

const NANOS_PER_SEC: i64 = 1_000_000_000;

/// A page of list results, plus whether the service stopped short of the full
/// match set.
///
/// Every list call is capped at `limit`, and the upstream response that told us
/// more exist is dropped on the floor once we truncate. Without this flag a
/// caller cannot tell a complete result from a truncated one, so "how many are
/// there" is unanswerable — and an agent reading a capped page tends to report
/// its size as the total.
#[derive(Debug)]
pub struct Page<T> {
    pub items: Vec<T>,
    pub has_more: bool,
}

/// Maximum channel names spelled out in a message before the tail is summarized.
const MAX_NAMED_CHANNELS: usize = 20;

/// Formats channel names for an error or a warning. A selection can hold up to
/// [`PAGE_SIZE`] channels, so an uncapped list would bury the guidance that
/// follows it under a wall of names.
pub fn name_list(names: &[String]) -> String {
    if names.len() <= MAX_NAMED_CHANNELS {
        return names.join(", ");
    }

    format!(
        "{}, and {} more",
        names[..MAX_NAMED_CHANNELS].join(", "),
        names.len() - MAX_NAMED_CHANNELS,
    )
}

/// Returns page size and record limit. `limit` is clamped to `1..=PAGE_SIZE`;
/// omitting it falls back to [`DEFAULT_LIMIT`]. No input yields an unbounded
/// record limit.
pub fn paging(limit: Option<u32>) -> (u32, usize) {
    let limit = limit.unwrap_or(DEFAULT_LIMIT).clamp(1, PAGE_SIZE);
    (limit, limit as usize)
}

/// Escapes a value for interpolation into a double-quoted CEL string literal.
pub fn cel_escape(s: &str) -> String {
    s.replace('\\', "\\\\").replace('"', "\\\"")
}

pub fn unix_nanos_to_secs_and_subsec_nanos(nanos: i64) -> (i64, i32) {
    let secs = nanos.div_euclid(NANOS_PER_SEC);
    let subsec_nanos = nanos.rem_euclid(NANOS_PER_SEC) as i32;
    (secs, subsec_nanos)
}

pub fn secs_and_subsec_nanos_to_unix_nanos(sec: i64, subsec_nanos: i32) -> i64 {
    sec * NANOS_PER_SEC + i64::from(subsec_nanos)
}

/// How a [`ColumnName`] is written onto a Parquet field.
///
/// `Sql` is the default. The field name is a plain SQL identifier and
/// `channel_id`, `run`, and `units` live in field metadata. `Legacy` keeps the
/// historical column-name string for callers that parse it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ColumnNameStyle {
    #[default]
    Sql,
    Legacy,
}

impl ColumnNameStyle {
    /// Parses the `get_data` `column_names` parameter. Omitted means [`Sql`](Self::Sql).
    pub fn parse(value: Option<&str>) -> Result<Self, anyhow::Error> {
        match value.map(str::trim).filter(|s| !s.is_empty()) {
            None | Some("sql") => Ok(Self::Sql),
            Some("legacy") => Ok(Self::Legacy),
            Some(other) => {
                bail!("column_names must be `sql` or `legacy`, got `{other}`")
            }
        }
    }
}

/// A fully-qualified channel column. `Display` emits the legacy Parquet column
/// name: `<name> {channel_id="...", bit_field_element="...", run="...", units="..."}`.
/// Empty optional fields are omitted.
///
/// The name written into a file is [`ColumnName::field_names`], which is a plain
/// SQL identifier unless the caller asked for [`ColumnNameStyle::Legacy`].
/// Construct via [`ColumnName::builder`] or [`ColumnName::from_field_parts`].
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

    /// Preferred Parquet field name for [`ColumnNameStyle::Sql`], before
    /// de-duplication. A plain identifier: `[A-Za-z_][A-Za-z0-9_]*`, and not a
    /// reserved SQL word.
    pub fn sql_identifier(&self) -> String {
        let mut label = sanitize_sql_identifier(&self.name);
        if let Some(element) = &self.bit_field_element {
            label = format!("{label}_{}", sanitize_sql_identifier(element));
        }
        if is_reserved_sql_word(&label) {
            label.insert_str(0, "col_");
        }
        label
    }

    /// Identity stored in Parquet field metadata so a SQL-safe field name does
    /// not have to carry it. `run`, `units`, and `bit_field_element` are omitted
    /// when empty.
    pub fn identity_metadata(&self) -> HashMap<String, String> {
        let mut metadata = HashMap::from([
            (CHANNEL_NAME_METADATA_KEY.to_string(), self.name.clone()),
            (CHANNEL_ID_METADATA_KEY.to_string(), self.channel_id.clone()),
        ]);
        if let Some(element) = &self.bit_field_element {
            metadata.insert(BIT_FIELD_ELEMENT_METADATA_KEY.to_string(), element.clone());
        }
        if let Some(run) = &self.run {
            metadata.insert(RUN_METADATA_KEY.to_string(), run.clone());
        }
        if let Some(units) = &self.units {
            metadata.insert(UNITS_METADATA_KEY.to_string(), units.clone());
        }
        metadata
    }

    /// Field names for `columns`, in the same order.
    ///
    /// [`ColumnNameStyle::Sql`] uses [`sql_identifier`](Self::sql_identifier).
    /// When several columns share that identifier, each one gains `_` plus its
    /// channel id so the assignment does not depend on map iteration order.
    /// [`ColumnNameStyle::Legacy`] uses the historical `Display` string.
    pub fn field_names(columns: &[&Self], style: ColumnNameStyle) -> Vec<String> {
        if style == ColumnNameStyle::Legacy {
            return columns.iter().map(|column| column.to_string()).collect();
        }

        let preferred: Vec<String> = columns
            .iter()
            .map(|column| column.sql_identifier())
            .collect();
        let mut counts: HashMap<&str, usize> = HashMap::new();
        for name in &preferred {
            *counts.entry(name.as_str()).or_insert(0) += 1;
        }

        let mut order: Vec<usize> = (0..columns.len()).collect();
        order.sort_by(|&left, &right| {
            preferred[left]
                .cmp(&preferred[right])
                .then(columns[left].channel_id.cmp(&columns[right].channel_id))
                .then(
                    columns[left]
                        .bit_field_element
                        .cmp(&columns[right].bit_field_element),
                )
                .then(columns[left].run.cmp(&columns[right].run))
                .then(columns[left].name.cmp(&columns[right].name))
        });

        let mut used = HashSet::new();
        let mut assigned = vec![String::new(); columns.len()];
        for index in order {
            let column = columns[index];
            let mut candidate = if counts[preferred[index].as_str()] > 1 {
                format!("{}_{}", preferred[index], sql_suffix(&column.channel_id))
            } else {
                preferred[index].clone()
            };
            if used.contains(&candidate) {
                let extra = column
                    .bit_field_element
                    .as_deref()
                    .or(column.run.as_deref())
                    .unwrap_or("x");
                candidate = format!("{candidate}_{}", sql_suffix(extra));
            }
            let base = candidate.clone();
            let mut suffix = 2u32;
            while used.contains(&candidate) {
                candidate = format!("{base}_{suffix}");
                suffix += 1;
            }
            used.insert(candidate.clone());
            assigned[index] = candidate;
        }
        assigned
    }

    /// Rebuilds a column from a Parquet field. Metadata written by
    /// [`ColumnNameStyle::Sql`] wins; otherwise the field name is parsed as the
    /// legacy `Display` form.
    pub fn from_field_parts(
        name: &str,
        metadata: &HashMap<String, String>,
    ) -> Result<Self, anyhow::Error> {
        if let Some(channel_id) = metadata.get(CHANNEL_ID_METADATA_KEY) {
            let channel_name = metadata
                .get(CHANNEL_NAME_METADATA_KEY)
                .map(String::as_str)
                .unwrap_or(name);
            return Ok(Self::builder(channel_name, channel_id)
                .bit_field_element(
                    metadata
                        .get(BIT_FIELD_ELEMENT_METADATA_KEY)
                        .map(String::as_str),
                )
                .run(metadata.get(RUN_METADATA_KEY).map(String::as_str))
                .units(metadata.get(UNITS_METADATA_KEY).map(String::as_str))
                .build());
        }
        Self::try_from(name)
    }
}

/// ASCII letters, digits, and `_` pass through. Every other run of characters
/// becomes a single `_`. The result is never empty and never starts with a digit,
/// so it can stand alone as an unquoted SQL identifier.
fn sanitize_sql_identifier(raw: &str) -> String {
    let mut out = sanitize_sql_fragment(raw);
    if out.is_empty() || out.as_bytes()[0].is_ascii_digit() {
        out.insert(0, '_');
    }
    out
}

/// Like [`sanitize_sql_identifier`], but a leading digit is kept. Used for a
/// suffix that is already attached to a valid identifier.
fn sanitize_sql_fragment(raw: &str) -> String {
    let mut out = String::new();
    let mut separator = false;
    for c in raw.chars() {
        if c.is_ascii_alphanumeric() {
            if separator && !out.is_empty() {
                out.push('_');
            }
            separator = false;
            out.push(c);
        } else {
            separator = true;
        }
    }
    out
}

fn sql_suffix(raw: &str) -> String {
    let fragment = sanitize_sql_fragment(raw);
    if fragment.is_empty() {
        "x".to_string()
    } else {
        fragment
    }
}

/// Lowercase reserved words that cannot be a bare identifier in the SQL dialect
/// `sql` uses. Sorted so [`is_reserved_sql_word`] can binary-search.
const RESERVED_SQL_WORDS: &[&str] = &[
    "all",
    "and",
    "as",
    "asc",
    "avg",
    "between",
    "by",
    "case",
    "cast",
    "count",
    "create",
    "cross",
    "delete",
    "desc",
    "distinct",
    "drop",
    "else",
    "end",
    "except",
    "exists",
    "false",
    "filter",
    "first",
    "from",
    "full",
    "group",
    "having",
    "in",
    "inner",
    "insert",
    "intersect",
    "interval",
    "into",
    "is",
    "join",
    "last",
    "left",
    "like",
    "limit",
    "max",
    "min",
    "natural",
    "not",
    "null",
    "offset",
    "on",
    "or",
    "order",
    "outer",
    "over",
    "partition",
    "right",
    "select",
    "set",
    "sum",
    "table",
    "then",
    "timestamp",
    "true",
    "union",
    "update",
    "using",
    "values",
    "when",
    "where",
    "window",
    "with",
];

fn is_reserved_sql_word(ident: &str) -> bool {
    let lower = ident.to_ascii_lowercase();
    RESERVED_SQL_WORDS.binary_search(&lower.as_str()).is_ok()
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
