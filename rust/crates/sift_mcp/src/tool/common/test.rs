use sift_rs::metadata::v1::{MetadataKeyType, metadata_value::Value as MetadataValueInner};

use super::{MetadataEntry, MetadataScalar};
use sift_rs::metadata::v1::MetadataValue;

fn convert(scalar: MetadataScalar) -> MetadataValue {
    MetadataEntry {
        name: "k".into(),
        value: scalar,
    }
    .into()
}

#[test]
fn string_scalar_maps_to_string_key_and_value() {
    let mv = convert(MetadataScalar::String("hello".into()));
    assert_eq!(mv.key.unwrap().r#type, MetadataKeyType::String as i32);
    assert!(matches!(mv.value, Some(MetadataValueInner::StringValue(s)) if s == "hello"));
}

#[test]
fn number_scalar_maps_to_number_key_and_value() {
    let mv = convert(MetadataScalar::Number(3.5));
    assert_eq!(mv.key.unwrap().r#type, MetadataKeyType::Number as i32);
    assert!(matches!(mv.value, Some(MetadataValueInner::NumberValue(n)) if n == 3.5));
}

#[test]
fn boolean_scalar_maps_to_boolean_key_and_value() {
    let mv = convert(MetadataScalar::Boolean(true));
    assert_eq!(mv.key.unwrap().r#type, MetadataKeyType::Boolean as i32);
    assert!(matches!(mv.value, Some(MetadataValueInner::BooleanValue(b)) if b));
}

#[test]
fn untagged_scalar_deserializes_by_json_literal_type() {
    // The untagged enum must pick the variant from the JSON literal type.
    assert!(matches!(
        serde_json::from_str::<MetadataScalar>("\"text\"").unwrap(),
        MetadataScalar::String(_)
    ));
    assert!(matches!(
        serde_json::from_str::<MetadataScalar>("42").unwrap(),
        MetadataScalar::Number(_)
    ));
    assert!(matches!(
        serde_json::from_str::<MetadataScalar>("true").unwrap(),
        MetadataScalar::Boolean(_)
    ));
}
