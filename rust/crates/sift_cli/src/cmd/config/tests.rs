mod test_is_update_empty {
    use super::super::is_update_empty;
    use crate::cli::ConfigUpdateArgs;

    fn args(
        grpc_uri: Option<&str>,
        rest_uri: Option<&str>,
        api_key: Option<&str>,
        app_uri: Option<&str>,
    ) -> ConfigUpdateArgs {
        ConfigUpdateArgs {
            interactive: false,
            grpc_uri: grpc_uri.map(String::from),
            rest_uri: rest_uri.map(String::from),
            api_key: api_key.map(String::from),
            app_uri: app_uri.map(String::from),
        }
    }

    #[test]
    fn no_flags_is_empty() {
        assert!(is_update_empty(&args(None, None, None, None)));
    }

    #[test]
    fn all_empty_strings_is_empty() {
        assert!(is_update_empty(&args(
            Some(""),
            Some(""),
            Some(""),
            Some("")
        )));
    }

    #[test]
    fn any_single_flag_is_not_empty() {
        assert!(!is_update_empty(&args(Some("g"), None, None, None)));
        assert!(!is_update_empty(&args(None, Some("r"), None, None)));
        assert!(!is_update_empty(&args(None, None, Some("k"), None)));
        assert!(!is_update_empty(&args(None, None, None, Some("a"))));
    }

    #[test]
    fn all_flags_set_is_not_empty() {
        assert!(!is_update_empty(&args(
            Some("g"),
            Some("r"),
            Some("k"),
            Some("a")
        )));
    }
}

mod app_uri {
    use super::super::{
        AppUriState, app_uri_state, apply_profile_updates, set_missing_app_uri_value,
    };
    use toml::{Table, Value};

    fn config(input: &str) -> Table {
        input.parse().unwrap()
    }

    #[test]
    fn reports_configured_and_missing_app_uris() {
        let config = config(
            r#"
rest_uri = "https://api.siftstack.com"
app_uri = "https://app.siftstack.com"

[gov]
rest_uri = "https://gov.api.siftstack.com"

[custom]
rest_uri = "https://api.example.net"
"#,
        );

        assert_eq!(
            app_uri_state(&config, None).unwrap(),
            AppUriState::Configured("https://app.siftstack.com".to_string())
        );
        assert_eq!(
            app_uri_state(&config, Some("gov")).unwrap(),
            AppUriState::MissingKnown("https://gov.siftstack.com".to_string())
        );
        assert_eq!(
            app_uri_state(&config, Some("custom")).unwrap(),
            AppUriState::MissingUnknown(Some("https://api.example.net".to_string()))
        );
    }

    #[test]
    fn known_rest_uri_sets_a_missing_app_uri() {
        let mut config = Table::new();
        apply_profile_updates(
            &mut config,
            None,
            None,
            Some("https://api.siftstack.com".to_string()),
            None,
            None,
        )
        .unwrap();

        assert_eq!(
            config.get("app_uri"),
            Some(&Value::String("https://app.siftstack.com".to_string()))
        );
    }

    #[test]
    fn unknown_rest_uri_does_not_set_app_uri() {
        let mut config = Table::new();
        apply_profile_updates(
            &mut config,
            None,
            None,
            Some("https://api.example.net".to_string()),
            None,
            None,
        )
        .unwrap();

        assert!(!config.contains_key("app_uri"));
    }

    #[test]
    fn inferred_app_uri_does_not_replace_an_existing_value() {
        let mut config = config(
            r#"
rest_uri = "https://api.example.net"
app_uri = "https://sift.example.net"
"#,
        );
        apply_profile_updates(
            &mut config,
            None,
            None,
            Some("https://api.siftstack.com".to_string()),
            None,
            None,
        )
        .unwrap();

        assert_eq!(
            config.get("app_uri"),
            Some(&Value::String("https://sift.example.net".to_string()))
        );
    }

    #[test]
    fn explicit_app_uri_is_written_to_a_named_profile() {
        let mut config = Table::new();
        apply_profile_updates(
            &mut config,
            Some("customer".to_string()),
            None,
            None,
            None,
            Some("https://sift.example.net".to_string()),
        )
        .unwrap();

        assert_eq!(
            config["customer"]["app_uri"].as_str(),
            Some("https://sift.example.net")
        );
    }

    #[test]
    fn doctor_fix_sets_only_a_missing_app_uri() {
        let mut config = config(
            r#"
[production]
rest_uri = "https://api.siftstack.com"

[custom]
rest_uri = "https://api.example.net"
app_uri = "https://sift.example.net"
"#,
        );

        assert!(
            set_missing_app_uri_value(&mut config, Some("production"), "https://app.siftstack.com")
                .unwrap()
        );
        assert!(
            !set_missing_app_uri_value(&mut config, Some("custom"), "https://wrong.example.net")
                .unwrap()
        );
        assert_eq!(
            config["production"]["app_uri"].as_str(),
            Some("https://app.siftstack.com")
        );
        assert_eq!(
            config["custom"]["app_uri"].as_str(),
            Some("https://sift.example.net")
        );
    }
}
