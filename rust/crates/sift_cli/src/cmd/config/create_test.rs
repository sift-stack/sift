use indoc::formatdoc;
use sift_connect::grpc::config::SIFT_CONFIG_NAME;
use std::{
    fs::File,
    io::{Sink, Write},
};
use tempdir::TempDir;
use toml::{Table, Value};

#[test]
fn test_config_create_no_existing_config_default_profile() {
    let tmp_dir = TempDir::new("test_config_create_no_existing_config_default_profile")
        .expect("failed to create temporary directory");
    let config_path = tmp_dir.path().join(SIFT_CONFIG_NAME);

    let mut out = Vec::new();

    let uri = "https://example.com";
    let apikey = "123123123";

    super::create::run(
        &mut out,
        Some(&config_path),
        None,
        uri.into(),
        apikey.into(),
        false,
    )
    .expect("config::create");

    let (content, _) =
        super::read_config_to_string(Some(&config_path)).expect("failed to read config");

    let toml_config = content
        .parse::<Table>()
        .expect("failed to parse newly created config");

    assert_eq!(
        Some(Value::String(uri.to_string())),
        toml_config.get("uri").cloned()
    );
    assert_eq!(
        Some(Value::String(apikey.to_string())),
        toml_config.get("apikey").cloned()
    );

    let actual = String::from_utf8(strip_ansi_escapes::strip(out)).expect("expected UTF-8");
    let expected = format!(
        "Successfully updated default profile in {}\n",
        config_path.display()
    );
    assert_eq!(expected, actual);
}

#[test]
fn test_config_create_no_existing_config_named_profile() {
    let tmp_dir = TempDir::new("test_config_create_no_existing_config_named_profile")
        .expect("failed to create temporary directory");
    let config_path = tmp_dir.path().join(SIFT_CONFIG_NAME);

    let mut out = Vec::new();

    let uri = "https://example.com";
    let apikey = "123123123";
    let profile = "named_profile";

    super::create::run(
        &mut out,
        Some(&config_path),
        Some(profile.into()),
        uri.into(),
        apikey.into(),
        false,
    )
    .expect("config::create");

    let (content, _) =
        super::read_config_to_string(Some(&config_path)).expect("failed to read config");

    let toml_config = content
        .parse::<Table>()
        .expect("failed to parse newly created config");

    let Some(Value::Table(named_profile)) = toml_config.get(profile).cloned() else {
        panic!("expected toml sub-table");
    };

    assert_eq!(
        Some(Value::String(uri.to_string())),
        named_profile.get("uri").cloned()
    );
    assert_eq!(
        Some(Value::String(apikey.to_string())),
        named_profile.get("apikey").cloned()
    );

    let actual = String::from_utf8(strip_ansi_escapes::strip(out)).expect("expected UTF-8");
    let expected = format!(
        "Successfully updated {profile} profile in {}\n",
        config_path.display()
    );
    assert_eq!(expected, actual);
}

#[test]
fn test_config_create_update_default_profile() {
    let tmp_dir = TempDir::new("test_config_create_update_default_profile")
        .expect("failed to create temporary directory");
    let config_path = tmp_dir.path().join(SIFT_CONFIG_NAME);
    let mut sift_config = File::create(&config_path).expect("failed to create test Sift config");

    let dev_uri = "https://dev-example.com";
    let dev_key = "other-api-key";
    let named_profile = "development";

    let example_config = formatdoc! {"
        uri = \"https://example.com\"
        apikey = \"some-api-key\"

        [{named_profile}]
        uri = \"{dev_uri}\"
        apikey = \"{dev_key}\"
    "};

    sift_config
        .write_all(example_config.as_bytes())
        .expect("failed to write contents to config");

    let mut sink = Sink::default();

    let new_uri = "https://new-example.com";
    let new_apikey = "new-123123123";

    let res = super::create::run(
        &mut sink,
        Some(&config_path),
        None,
        new_uri.into(),
        new_apikey.into(),
        false, // force
    );

    assert!(
        res.is_err(),
        "should not be able to override default without force"
    );

    let mut out = Vec::new();

    let res = super::create::run(
        &mut out,
        Some(&config_path),
        None,
        new_uri.into(),
        new_apikey.into(),
        true, // force
    );

    assert!(res.is_ok(), "update should have succeeded with force");

    let (content, _) =
        super::read_config_to_string(Some(&config_path)).expect("failed to read config");

    let toml_config = content
        .parse::<Table>()
        .expect("failed to parse newly created config");

    assert_eq!(
        Some(Value::String(new_uri.to_string())),
        toml_config.get("uri").cloned()
    );
    assert_eq!(
        Some(Value::String(new_apikey.to_string())),
        toml_config.get("apikey").cloned()
    );

    let actual = String::from_utf8(strip_ansi_escapes::strip(out)).expect("expected UTF-8");
    let expected = format!(
        "Successfully updated default profile in {}\n",
        config_path.display()
    );
    assert_eq!(expected, actual);

    let Some(Value::Table(named_profile)) = toml_config.get(named_profile).cloned() else {
        panic!("expected toml sub-table");
    };

    assert_eq!(
        Some(Value::String(dev_uri.to_string())),
        named_profile.get("uri").cloned(),
        "named profile shouldn't change"
    );
    assert_eq!(
        Some(Value::String(dev_key.to_string())),
        named_profile.get("apikey").cloned(),
        "named profile shouldn't change"
    );
}

#[test]
fn test_config_create_update_named_profile() {
    let tmp_dir = TempDir::new("test_config_create_update_named_profile")
        .expect("failed to create temporary directory");
    let config_path = tmp_dir.path().join(SIFT_CONFIG_NAME);
    let mut sift_config = File::create(&config_path).expect("failed to create test Sift config");

    let uri = "https://default-example.com";
    let key = "default-api-key";
    let named_profile = "development";

    let example_config = formatdoc! {"
        uri = \"{uri}\"
        apikey = \"{key}\"

        [{named_profile}]
        uri = \"https://example.com\"
        apikey = \"some-api-key\"
    "};

    sift_config
        .write_all(example_config.as_bytes())
        .expect("failed to write contents to config");

    let mut sink = Sink::default();

    let new_uri = "https://new-example.com";
    let new_apikey = "new-123123123";

    let res = super::create::run(
        &mut sink,
        Some(&config_path),
        Some(named_profile.into()),
        new_uri.into(),
        new_apikey.into(),
        false, // force
    );

    assert!(
        res.is_err(),
        "should not be able to override default without force"
    );

    let mut out = Vec::new();

    let res = super::create::run(
        &mut out,
        Some(&config_path),
        Some(named_profile.into()),
        new_uri.into(),
        new_apikey.into(),
        true, // force
    );

    assert!(res.is_ok(), "update should have succeeded with force");

    let (content, _) =
        super::read_config_to_string(Some(&config_path)).expect("failed to read config");

    let toml_config = content
        .parse::<Table>()
        .expect("failed to parse newly created config");

    assert_eq!(
        Some(Value::String(uri.to_string())),
        toml_config.get("uri").cloned(),
        "default profile shouldn't change"
    );
    assert_eq!(
        Some(Value::String(key.to_string())),
        toml_config.get("apikey").cloned(),
        "default profile shouldn't change"
    );

    let actual = String::from_utf8(strip_ansi_escapes::strip(out)).expect("expected UTF-8");
    let expected = format!(
        "Successfully updated {named_profile} profile in {}\n",
        config_path.display()
    );
    assert_eq!(expected, actual);

    let Some(Value::Table(named_profile)) = toml_config.get(named_profile).cloned() else {
        panic!("expected toml sub-table");
    };

    assert_eq!(
        Some(Value::String(new_uri.to_string())),
        named_profile.get("uri").cloned(),
        "named profile shouldn't change"
    );
    assert_eq!(
        Some(Value::String(new_apikey.to_string())),
        named_profile.get("apikey").cloned(),
        "named profile shouldn't change"
    );
}
