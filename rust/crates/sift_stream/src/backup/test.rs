use crate::backup::sanitize_name;

#[test]
fn test_sanitize_name_with_illegal_chars() {
    let illegal_chars = vec![
        ':', '/', '\\', '*', '?', '"', '<', '>', '|', '.', ' ', '\t', '\n', '\r',
    ];
    for char in illegal_chars {
        assert_eq!(sanitize_name(&format!("test{}test", char)), "test_test");
    }
}

#[test]
fn test_sanitize_name_with_legal_chars() {
    assert_eq!(sanitize_name("test"), "test");
    assert_eq!(sanitize_name("test_test"), "test_test");
    assert_eq!(sanitize_name("test-test"), "test-test");
}
