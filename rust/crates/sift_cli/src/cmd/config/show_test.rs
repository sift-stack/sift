use super::show;
use indoc::indoc;
use sift_connect::grpc::config::SIFT_CONFIG_NAME;
use std::{fs::File, io::Write};
use tempdir::TempDir;

#[test]
fn test_config_show() {
    let tmp_dir =
        TempDir::new("test_config_show_run").expect("failed to create temporary directory");
    let config_path = tmp_dir.path().join(SIFT_CONFIG_NAME);
    let mut sift_config = File::create(&config_path).expect("failed to create test Sift config");

    let content = indoc! {"
        uri = \"https://example.com\"
        apikey = \"some-api-key\"

        [development]
        uri = \"https://example.com\"
        apikey = \"other-api-key\"
    "};

    sift_config
        .write_all(content.as_bytes())
        .expect("failed to write contents to config");

    let mut out = Vec::new();

    show::run(&mut out, Some(&config_path)).expect("show::run");

    let actual = String::from_utf8(strip_ansi_escapes::strip(out)).expect("expected UTF-8");
    let expect = format!("{content}\n{}\n", config_path.display());
    assert_eq!(expect, actual, "output doesn't match expected");
}
