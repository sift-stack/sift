use std::collections::HashMap;

use super::{
    CHANNEL_ID_METADATA_KEY, CHANNEL_NAME_METADATA_KEY, ColumnName, ColumnNameStyle, DEFAULT_LIMIT,
    PAGE_SIZE, RESERVED_SQL_WORDS, RUN_METADATA_KEY, UNITS_METADATA_KEY, paging,
};

#[test]
fn paging_uses_default_limit_when_unset() {
    assert_eq!(DEFAULT_LIMIT, 50);
    assert_eq!(paging(None), (50, 50));
}

#[test]
fn paging_passes_through_in_range_limit() {
    assert_eq!(paging(Some(125)), (125, 125));
}

#[test]
fn paging_clamps_limit_above_page_size() {
    assert_eq!(PAGE_SIZE, 200);
    assert_eq!(paging(Some(201)), (200, 200));
    assert_eq!(paging(Some(50_000)), (PAGE_SIZE, PAGE_SIZE as usize));
    assert_eq!(paging(Some(u32::MAX)), (PAGE_SIZE, PAGE_SIZE as usize));
}

#[test]
fn paging_clamps_zero_limit_to_one() {
    assert_eq!(paging(Some(0)), (1, 1));
}

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
fn column_name_sql_identifier_keeps_plain_names() {
    let out = ColumnName::builder("temp", "c1").build();
    assert_eq!(out.sql_identifier(), "temp");
    assert_eq!(
        ColumnName::field_names(&[&out], ColumnNameStyle::Sql),
        vec!["temp".to_string()]
    );
}

#[test]
fn column_name_sql_identifier_sanitizes_punctuation() {
    let out = ColumnName::builder("PT-PC", "c1")
        .run(Some("fc1b4ea8-1111"))
        .units(Some("psia"))
        .build();
    assert_eq!(out.sql_identifier(), "PT_PC");
    assert_eq!(
        ColumnName::builder("motor.d.current", "c1")
            .build()
            .sql_identifier(),
        "motor_d_current"
    );
    assert_eq!(
        ColumnName::builder("status", "c1")
            .bit_field_element(Some("fault-a"))
            .build()
            .sql_identifier(),
        "status_fault_a"
    );
    assert_eq!(
        ColumnName::builder("1temp", "c1").build().sql_identifier(),
        "_1temp"
    );
    assert_eq!(
        ColumnName::builder("***", "c1").build().sql_identifier(),
        "_"
    );
}

#[test]
fn column_name_sql_identifier_prefixes_reserved_words() {
    assert_eq!(
        ColumnName::builder("select", "c1").build().sql_identifier(),
        "col_select"
    );
    assert_eq!(
        ColumnName::builder("ORDER", "c1").build().sql_identifier(),
        "col_ORDER"
    );
}

#[test]
fn reserved_sql_words_are_sorted_and_unique() {
    let mut sorted = RESERVED_SQL_WORDS.to_vec();
    sorted.sort_unstable();
    sorted.dedup();
    assert_eq!(RESERVED_SQL_WORDS, sorted.as_slice());
}

#[test]
fn column_name_field_names_suffix_collisions_stably() {
    let left = ColumnName::builder("PT-PC", "c1").build();
    let right = ColumnName::builder("PT_PC", "c2").build();
    assert_eq!(
        ColumnName::field_names(&[&left, &right], ColumnNameStyle::Sql),
        vec!["PT_PC_c1".to_string(), "PT_PC_c2".to_string()]
    );
    assert_eq!(
        ColumnName::field_names(&[&right, &left], ColumnNameStyle::Sql),
        vec!["PT_PC_c2".to_string(), "PT_PC_c1".to_string()]
    );
}

#[test]
fn column_name_legacy_field_names_keep_the_brace_form() {
    let out = ColumnName::builder("temp", "c1").build();
    assert_eq!(
        ColumnName::field_names(&[&out], ColumnNameStyle::Legacy),
        vec![out.to_string()]
    );
}

#[test]
fn column_name_identity_metadata_omits_empty_optionals() {
    let out = ColumnName::builder("PT-PC", "c1")
        .run(Some("r1"))
        .units(Some("psia"))
        .build();
    let metadata = out.identity_metadata();
    assert_eq!(
        metadata.get(CHANNEL_NAME_METADATA_KEY).map(String::as_str),
        Some("PT-PC")
    );
    assert_eq!(
        metadata.get(CHANNEL_ID_METADATA_KEY).map(String::as_str),
        Some("c1")
    );
    assert_eq!(
        metadata.get(RUN_METADATA_KEY).map(String::as_str),
        Some("r1")
    );
    assert_eq!(
        metadata.get(UNITS_METADATA_KEY).map(String::as_str),
        Some("psia")
    );
    assert!(!metadata.contains_key("bit_field_element"));
}

#[test]
fn column_name_from_field_parts_prefers_metadata() {
    let original = ColumnName::builder("PT-PC", "c1")
        .run(Some("r1"))
        .units(Some("psia"))
        .build();
    let parsed = ColumnName::from_field_parts("PT_PC", &original.identity_metadata())
        .expect("metadata should rebuild the column");
    assert_eq!(parsed, original);
}

#[test]
fn column_name_from_field_parts_falls_back_to_legacy_name() {
    let original = ColumnName::builder("temp", "c1").build();
    let parsed = ColumnName::from_field_parts(&original.to_string(), &HashMap::new())
        .expect("legacy column name should parse");
    assert_eq!(parsed, original);
}

#[test]
fn column_name_style_parse_defaults_to_sql() {
    assert_eq!(ColumnNameStyle::parse(None).unwrap(), ColumnNameStyle::Sql);
    assert_eq!(
        ColumnNameStyle::parse(Some(" sql ")).unwrap(),
        ColumnNameStyle::Sql
    );
    assert_eq!(
        ColumnNameStyle::parse(Some("legacy")).unwrap(),
        ColumnNameStyle::Legacy
    );
    assert!(ColumnNameStyle::parse(Some("quoted")).is_err());
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
