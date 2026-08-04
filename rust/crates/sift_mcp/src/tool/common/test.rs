use super::{ListParams, url_clause};

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
