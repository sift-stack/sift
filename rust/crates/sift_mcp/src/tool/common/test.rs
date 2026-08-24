use super::{ListParams, list_body, project_fields, url_clause};
use serde_json::{Value, json};

/// `filter` is deliberately required on list tools: the caller must choose a
/// scope explicitly, with the empty string as the documented way to request
/// everything. Loosening this to `Option` would let unfiltered queries fire
/// by accident; keep the schema rejection.
#[test]
fn list_params_reject_omitted_filter() {
    let result = serde_json::from_value::<ListParams>(serde_json::json!({ "limit": 5 }));
    assert!(
        result.is_err(),
        "omitted filter must fail schema validation"
    );
}

#[test]
fn list_params_accept_empty_filter() {
    let params: ListParams =
        serde_json::from_value(serde_json::json!({ "filter": "" })).expect("empty filter is valid");
    assert_eq!(params.filter, "");
    assert_eq!(params.limit, None);
}

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
        list_body("channels", items.clone(), None, false),
        json!({ "channels": items, "count": 1, "has_more": false })
    );
}

#[test]
fn list_body_treats_an_empty_field_list_as_no_projection() {
    let items = vec![json!({ "channelId": "c1", "name": "throttle" })];

    assert_eq!(
        list_body("channels", items.clone(), Some(vec![]), false),
        json!({ "channels": items, "count": 1, "has_more": false })
    );
}

#[test]
fn list_body_surfaces_unmatched_fields_alongside_the_items() {
    let items = vec![json!({ "name": "throttle" })];

    assert_eq!(
        list_body(
            "channels",
            items,
            Some(vec!["name".into(), "nope".into()]),
            false
        ),
        json!({
            "channels": [{ "name": "throttle" }],
            "count": 1,
            "has_more": false,
            "unmatched_fields": ["nope"],
        })
    );
}

#[test]
fn list_body_counts_the_items_it_returns() {
    // The caller should never have to count the array itself.
    let items: Vec<Value> = (0..120)
        .map(|i| json!({ "name": format!("c{i}") }))
        .collect();

    let body = list_body("channels", items, None, false);

    assert_eq!(body["count"], json!(120));
    assert_eq!(body["channels"].as_array().unwrap().len(), 120);
}

#[test]
fn list_body_counts_what_survived_projection() {
    // Projection drops nothing here, but the count must describe the returned
    // rows rather than anything upstream of them.
    let items = vec![json!({ "name": "a" }), json!({ "name": "b" })];

    let body = list_body("channels", items, Some(vec!["name".into()]), false);

    assert_eq!(body["count"], json!(2));
}

#[test]
fn list_body_reports_a_capped_page() {
    // `count` alone reads as a total. Pairing it with `has_more` is what tells a
    // caller the page was cut short rather than leaving them to guess from
    // whether count happens to equal the limit they passed.
    let items = vec![json!({ "name": "a" }), json!({ "name": "b" })];

    let body = list_body("channels", items, None, true);

    assert_eq!(body["count"], json!(2));
    assert_eq!(body["has_more"], json!(true));
}

#[test]
fn project_fields_reports_nothing_unmatched_for_an_empty_page() {
    // A filter that matched no rows is not a misspelled field name. Reporting
    // every requested field as unmatched here would send the caller off
    // respelling names that were fine.
    let (items, unmatched) = project_fields(Vec::new(), &["name".into(), "channel_id".into()]);

    assert!(items.is_empty());
    assert!(
        unmatched.is_empty(),
        "an empty page says nothing about field names: {unmatched:?}"
    );
}

#[test]
fn project_fields_still_reports_a_typo_on_a_non_empty_page() {
    // The guard above must not swallow the signal the field exists for.
    let items = vec![json!({ "name": "throttle" })];

    let (_, unmatched) = project_fields(items, &["name".into(), "nmae".into()]);

    assert_eq!(unmatched, vec!["nmae".to_string()]);
}

#[test]
fn normalized_names_ignore_case_and_both_separators() {
    let items = vec![json!({ "assetId": "a1" })];

    for spelling in ["asset_id", "asset-id", "AssetId", "ASSET_ID"] {
        let (projected, unmatched) = project_fields(items.clone(), &[spelling.into()]);
        assert_eq!(
            projected,
            vec![json!({ "assetId": "a1" })],
            "{spelling} should address the same key"
        );
        assert!(unmatched.is_empty(), "{spelling} should match");
    }
}

#[test]
fn list_body_omits_unmatched_fields_when_the_page_is_empty() {
    let body = list_body("channels", Vec::new(), Some(vec!["name".into()]), false);

    assert_eq!(body["count"], json!(0));
    assert!(
        body.get("unmatched_fields").is_none(),
        "an empty page must not look like a spelling problem: {body}"
    );
}
