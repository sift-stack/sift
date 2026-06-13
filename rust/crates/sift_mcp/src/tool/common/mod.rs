use rmcp::schemars::{self, JsonSchema};
use serde::Deserialize;
use sift_rs::metadata::v1::{
    MetadataKey, MetadataKeyType, MetadataValue, metadata_value::Value as MetadataValueInner,
};

/// A scalar metadata value. Untagged so the JSON literal type (string / number /
/// boolean) selects the variant directly.
#[derive(Debug, Deserialize, JsonSchema)]
#[serde(untagged)]
pub enum MetadataScalar {
    String(String),
    Number(f64),
    Boolean(bool),
}

/// A single `{ name, value }` metadata entry as accepted by tool parameters.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct MetadataEntry {
    pub name: String,
    pub value: MetadataScalar,
}

impl From<MetadataEntry> for MetadataValue {
    fn from(entry: MetadataEntry) -> Self {
        let (key_type, value) = match entry.value {
            MetadataScalar::String(s) => {
                (MetadataKeyType::String, MetadataValueInner::StringValue(s))
            }
            MetadataScalar::Number(n) => {
                (MetadataKeyType::Number, MetadataValueInner::NumberValue(n))
            }
            MetadataScalar::Boolean(b) => (
                MetadataKeyType::Boolean,
                MetadataValueInner::BooleanValue(b),
            ),
        };
        MetadataValue {
            key: Some(MetadataKey {
                name: entry.name,
                r#type: key_type.into(),
                ..Default::default()
            }),
            value: Some(value),
            ..Default::default()
        }
    }
}

/// Escape a raw string for safe embedding inside a double-quoted CEL literal.
pub fn cel_escape(s: &str) -> String {
    s.replace('\\', "\\\\").replace('"', "\\\"")
}
