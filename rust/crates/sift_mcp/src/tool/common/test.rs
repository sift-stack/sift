use super::{list_body, project_fields, url_clause};
use serde_json::{Value, json};

#[test]
fn url_clause_present_renders_view_link() {
    assert_eq!(
        url_clause(Some("https://app.siftstack.com/rules/r-1")),
        " View it in Sift: https://app.siftstack.com/rules/r-1"
    );
}

#[test]
fn url_clause_none_is_empty() {
    assert_eq!(url_clause(None), "");
}

#[test]
fn project_fields_keeps_only_requested_keys() {
    let items = vec![json!({
        "channelId": "c1",
        "name": "throttle",
        "dataType": "CHANNEL_DATA_TYPE_DOUBLE",
    })];

    let (projected, unmatched) = project_fields(items, &["name".into(), "channelId".into()]);

    assert_eq!(
        projected,
        vec![json!({ "name": "throttle", "channelId": "c1" })]
    );
    assert!(unmatched.is_empty());
    // Requested keys keep their original spelling; dataType is dropped.
    let keys: Vec<&String> = projected[0].as_object().unwrap().keys().collect();
    assert_eq!(keys, vec!["channelId", "name"]);
}

#[test]
fn project_fields_matches_across_naming_conventions() {
    let items = vec![json!({ "assetId": "a1", "createdByUserId": "u1" })];

    let (projected, unmatched) =
        project_fields(items, &["asset_id".into(), "CREATED-BY-USER-ID".into()]);

    assert_eq!(
        projected,
        vec![json!({ "assetId": "a1", "createdByUserId": "u1" })]
    );
    assert!(unmatched.is_empty());
}

#[test]
fn project_fields_reports_a_name_that_matched_nothing() {
    let items = vec![json!({ "name": "throttle" })];

    let (projected, unmatched) = project_fields(items, &["name".into(), "nmae".into()]);

    assert_eq!(projected, vec![json!({ "name": "throttle" })]);
    assert_eq!(unmatched, vec!["nmae".to_string()]);
}

#[test]
fn project_fields_does_not_report_a_key_missing_from_only_some_items() {
    // Proto3 omits fields at their default value, so absence on one item says
    // nothing about the name being wrong.
    let items = vec![
        json!({ "name": "throttle", "description": "main throttle" }),
        json!({ "name": "chamber" }),
    ];

    let (projected, unmatched) = project_fields(items, &["name".into(), "description".into()]);

    assert_eq!(
        projected,
        vec![
            json!({ "name": "throttle", "description": "main throttle" }),
            json!({ "name": "chamber" }),
        ]
    );
    assert!(unmatched.is_empty());
}

#[test]
fn project_fields_passes_through_non_objects() {
    let items = vec![Value::String("bare".into()), json!(7)];

    let (projected, _) = project_fields(items.clone(), &["name".into()]);

    assert_eq!(projected, items);
}

#[test]
fn list_body_without_fields_returns_every_key() {
    let items = vec![json!({ "channelId": "c1", "name": "throttle" })];

    assert_eq!(
        list_body("channels", items.clone(), None),
        json!({ "channels": items })
    );
}

#[test]
fn list_body_treats_an_empty_field_list_as_no_projection() {
    let items = vec![json!({ "channelId": "c1", "name": "throttle" })];

    assert_eq!(
        list_body("channels", items.clone(), Some(vec![])),
        json!({ "channels": items })
    );
}

#[test]
fn list_body_surfaces_unmatched_fields_alongside_the_items() {
    let items = vec![json!({ "name": "throttle" })];

    assert_eq!(
        list_body("channels", items, Some(vec!["name".into(), "nope".into()])),
        json!({
            "channels": [{ "name": "throttle" }],
            "unmatched_fields": ["nope"],
        })
    );
}
