use super::ColumnName;

#[test]
fn column_name_required_fields_only() {
    let out = ColumnName::builder("temp", "c1").build();
    assert_eq!(out.to_string(), "temp {channel_id=\"c1\"}");
}

#[test]
fn column_name_canonical_field_order() {
    let out = ColumnName::builder("temp", "c1")
        .bit_field_element(Some("fault_a"))
        .run(Some("r42"))
        .units(Some("C"))
        .build();
    assert_eq!(
        out.to_string(),
        "temp {channel_id=\"c1\", bit_field_element=\"fault_a\", run=\"r42\", units=\"C\"}"
    );
}

#[test]
fn column_name_omits_none_optional_fields() {
    let out = ColumnName::builder("temp", "c1")
        .run(None)
        .units(None)
        .build();
    assert_eq!(out.to_string(), "temp {channel_id=\"c1\"}");
}

#[test]
fn column_name_omits_empty_optional_fields() {
    let out = ColumnName::builder("temp", "c1")
        .run(Some(""))
        .units(Some(""))
        .build();
    assert_eq!(out.to_string(), "temp {channel_id=\"c1\"}");
}

#[test]
fn column_name_keeps_present_optionals() {
    let out = ColumnName::builder("temp", "c1")
        .run(Some("r42"))
        .units(Some("C"))
        .build();
    assert_eq!(
        out.to_string(),
        "temp {channel_id=\"c1\", run=\"r42\", units=\"C\"}"
    );
}

#[test]
fn column_name_accessors_round_trip_builder_inputs() {
    let name = ColumnName::builder("temp", "c1")
        .bit_field_element(Some("fault_a"))
        .run(Some("r42"))
        .units(Some("C"))
        .build();
    assert_eq!(name.name(), "temp");
    assert_eq!(name.channel_id(), "c1");
    assert_eq!(name.bit_field_element(), Some("fault_a"));
    assert_eq!(name.run(), Some("r42"));
    assert_eq!(name.units(), Some("C"));
}

#[test]
fn column_name_into_string_emits_display() {
    let name = ColumnName::builder("temp", "c1").build();
    let s: String = name.into();
    assert_eq!(s, "temp {channel_id=\"c1\"}");
}

#[test]
fn column_name_try_from_required_only_round_trips() {
    let original = ColumnName::builder("temp", "c1").build();
    let parsed = ColumnName::try_from(original.to_string().as_str()).expect("should parse");
    assert_eq!(parsed, original);
}

#[test]
fn column_name_try_from_all_fields_round_trips() {
    let original = ColumnName::builder("temp", "c1")
        .bit_field_element(Some("fault_a"))
        .run(Some("r42"))
        .units(Some("C"))
        .build();
    let parsed = ColumnName::try_from(original.to_string().as_str()).expect("should parse");
    assert_eq!(parsed, original);
}

#[test]
fn column_name_try_from_empty_string_errors() {
    let err = ColumnName::try_from("").expect_err("empty input should error");
    assert!(err.to_string().contains("missing name"));
}

#[test]
fn column_name_try_from_missing_attr_block_errors() {
    let err = ColumnName::try_from("temp").expect_err("bare name (no attr block) should error");
    assert!(
        err.to_string().contains("missing attribute block"),
        "unexpected error: {err}"
    );
}

#[test]
fn column_name_try_from_missing_channel_id_errors() {
    let err =
        ColumnName::try_from("temp {run=\"r42\"}").expect_err("missing channel_id should error");
    assert!(
        err.to_string().contains("missing required `channel_id`"),
        "unexpected error: {err}"
    );
}

#[test]
fn column_name_try_from_unknown_key_errors() {
    let err = ColumnName::try_from("temp {channel_id=\"c1\", flavor=\"strawberry\"}")
        .expect_err("unknown key should error");
    assert!(
        err.to_string().contains("unknown attribute key `flavor`"),
        "unexpected error: {err}"
    );
}

#[test]
fn column_name_try_from_malformed_attr_errors() {
    let err = ColumnName::try_from("temp {channel_id=\"c1\", nokeyvalue}")
        .expect_err("attr without `=` should error");
    assert!(
        err.to_string().contains("missing `=`"),
        "unexpected error: {err}"
    );
}
