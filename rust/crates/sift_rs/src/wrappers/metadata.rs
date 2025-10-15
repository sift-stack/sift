use crate::metadata::v1::{MetadataKey, MetadataKeyType};

pub use crate::metadata::v1::MetadataValue;
pub use crate::metadata::v1::metadata_value::Value as MetadataEnumValue;

impl From<f64> for MetadataEnumValue {
    fn from(value: f64) -> MetadataEnumValue {
        MetadataEnumValue::NumberValue(value)
    }
}

impl From<bool> for MetadataEnumValue {
    fn from(value: bool) -> MetadataEnumValue {
        MetadataEnumValue::BooleanValue(value)
    }
}

impl From<String> for MetadataEnumValue {
    fn from(value: String) -> MetadataEnumValue {
        MetadataEnumValue::StringValue(value)
    }
}

impl From<&str> for MetadataEnumValue {
    fn from(value: &str) -> MetadataEnumValue {
        MetadataEnumValue::StringValue(value.to_string())
    }
}

impl<T: Into<MetadataEnumValue>> From<(String, T)> for MetadataValue {
    fn from((name, value): (String, T)) -> MetadataValue {
        MetadataValue::from((name.as_str(), value))
    }
}

impl<T: Into<MetadataEnumValue>> From<(&str, T)> for MetadataValue {
    fn from((name, value): (&str, T)) -> MetadataValue {
        let enum_value: MetadataEnumValue = value.into();
        let key_type = match enum_value {
            MetadataEnumValue::NumberValue(_) => MetadataKeyType::Number,
            MetadataEnumValue::BooleanValue(_) => MetadataKeyType::Boolean,
            MetadataEnumValue::StringValue(_) => MetadataKeyType::String,
        };

        let key = MetadataKey {
            name: name.to_string(),
            r#type: key_type.into(),
            archived_date: None,
            is_archived: false,
        };

        MetadataValue {
            key: Some(key),
            value: Some(enum_value),
            archived_date: None,
            is_archived: false,
        }
    }
}

/// A macro for easily creating an array of metadata to be provided to Sift.
/// Returns a Vec<[MetadataValue]>
///
///  # Example
/// ```
/// # #[macro_use] extern crate sift_rs;
/// # use sift_rs::metadata::v1::MetadataValue;
/// # fn main() {
/// let metadata: Vec<MetadataValue> = metadata![
///        ("test_number", 5.0),
///        ("is_simulation", true),
///        ("location", "SiftHQ"),
/// ];
/// # }
/// ```
#[macro_export]
macro_rules! metadata {
    ( $( ($k:expr, $v:expr) ),* $(,)? ) => {
        vec![ $( ($k, $v).into() ),* ]
    }
}
