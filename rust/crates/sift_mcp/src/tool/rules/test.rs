use rmcp::model::ErrorCode;

use super::{parse_rule_definition, rule_identifier};

#[test]
fn rule_identifier_accepts_rule_id_only() {
    let (rule_id, client_key) =
        rule_identifier(Some("rule-1".to_string()), None).expect("should accept rule_id");
    assert_eq!(rule_id, "rule-1");
    assert_eq!(client_key, "");
}

#[test]
fn rule_identifier_accepts_client_key_only() {
    let (rule_id, client_key) =
        rule_identifier(None, Some("ck-1".to_string())).expect("should accept client_key");
    assert_eq!(rule_id, "");
    assert_eq!(client_key, "ck-1");
}

#[test]
fn rule_identifier_rejects_both() {
    let err = rule_identifier(Some("rule-1".to_string()), Some("ck-1".to_string()))
        .expect_err("should reject both");
    assert_eq!(err.code, ErrorCode::INVALID_PARAMS);
}

#[test]
fn rule_identifier_rejects_neither() {
    let err = rule_identifier(None, None).expect_err("should reject neither");
    assert_eq!(err.code, ErrorCode::INVALID_PARAMS);
}

#[test]
fn parse_rule_definition_parses_valid_json() {
    let json = r#"{ "name": "overtemp", "description": "engine over temperature" }"#;
    let update = parse_rule_definition(json).expect("should parse");
    assert_eq!(update.name, "overtemp");
    assert_eq!(update.description, "engine over temperature");
}

#[test]
fn parse_rule_definition_rejects_invalid_json() {
    let err = parse_rule_definition("not json").expect_err("should reject");
    assert_eq!(err.code, ErrorCode::INVALID_PARAMS);
}
