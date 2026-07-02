use super::url_clause;

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
