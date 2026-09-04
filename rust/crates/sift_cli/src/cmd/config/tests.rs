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
    use super::super::{AppUriState, app_uri_state, apply_profile_updates};
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

[slash]
rest_uri = "https://api.siftstack.com"
app_uri = " / "

[invalid]
rest_uri = "https://api.siftstack.com"
app_uri = 42
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
        assert_eq!(
            app_uri_state(&config, Some("slash")).unwrap(),
            AppUriState::MissingKnown("https://app.siftstack.com".to_string())
        );
        assert_eq!(
            app_uri_state(&config, Some("invalid")).unwrap(),
            AppUriState::Invalid
        );
    }

    #[test]
    fn known_rest_uri_sets_a_missing_app_uri() {
        for (rest_uri, app_uri) in [
            ("https://api.siftstack.com", "https://app.siftstack.com"),
            ("https://gov.api.siftstack.com", "https://gov.siftstack.com"),
        ] {
            let mut config = Table::new();
            apply_profile_updates(
                &mut config,
                None,
                None,
                Some(rest_uri.to_string()),
                None,
                None,
            )
            .unwrap();

            assert_eq!(
                config.get("app_uri"),
                Some(&Value::String(app_uri.to_string())),
                "rest_uri: {rest_uri}"
            );
        }
    }

    #[test]
    fn unknown_rest_uri_does_not_set_app_uri() {
        for rest_uri in [
            "https://api.example.net",
            "https://api.development.siftstack.com",
            "https://api.staging.internal",
            "https://api.sift.test",
        ] {
            let mut config = Table::new();
            apply_profile_updates(
                &mut config,
                None,
                None,
                Some(rest_uri.to_string()),
                None,
                None,
            )
            .unwrap();

            assert!(!config.contains_key("app_uri"), "rest_uri: {rest_uri}");
        }
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
}

mod api_key_key {
    use super::super::apply_profile_updates;
    use crate::cmd::{API_KEY_KEY, API_KEY_KEY_LEGACY};
    use toml::Table;

    fn config(input: &str) -> Table {
        input.parse().unwrap()
    }

    fn update_key(config: &mut Table, profile: Option<&str>, key: &str) {
        apply_profile_updates(
            config,
            profile.map(String::from),
            None,
            None,
            Some(key.to_string()),
            None,
        )
        .unwrap();
    }

    #[test]
    fn writes_the_canonical_key() {
        let mut config = config("");
        update_key(&mut config, None, "fresh");
        assert_eq!(config[API_KEY_KEY].as_str(), Some("fresh"));
        assert!(!config.contains_key(API_KEY_KEY_LEGACY));
    }

    #[test]
    fn migrates_a_legacy_key_instead_of_leaving_both() {
        let mut config = config("apikey = \"old\"\n");
        update_key(&mut config, None, "new");
        assert_eq!(config[API_KEY_KEY].as_str(), Some("new"));
        assert!(
            !config.contains_key(API_KEY_KEY_LEGACY),
            "the older key must go, so that no reader sees two spellings"
        );
    }

    #[test]
    fn migrates_within_a_named_profile_only() {
        let mut config = config("apikey = \"top\"\n\n[mission]\napikey = \"old\"\n");
        update_key(&mut config, Some("mission"), "new");

        let mission = config["mission"].as_table().unwrap();
        assert_eq!(mission[API_KEY_KEY].as_str(), Some("new"));
        assert!(!mission.contains_key(API_KEY_KEY_LEGACY));

        // The default profile does not change, and keeps the older spelling.
        assert_eq!(config[API_KEY_KEY_LEGACY].as_str(), Some("top"));
    }
}
