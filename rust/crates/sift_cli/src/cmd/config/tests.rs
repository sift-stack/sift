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

mod profile_edits {
    use super::super::{
        Change, apply_profile_updates, mask_key, named_profiles, profile_changes, resolve_profile,
    };
    use toml::Table;

    fn config(input: &str) -> Table {
        input.parse().unwrap()
    }

    fn change(field: &'static str, old: &str, new: &str) -> Change {
        Change {
            field,
            old: old.to_string(),
            new: new.to_string(),
        }
    }

    #[test]
    fn masks_all_but_the_last_four_characters() {
        assert_eq!(mask_key("sift_abcdefgh1234"), "****1234");
        assert_eq!(mask_key("short"), "****");
        assert_eq!(mask_key("12345678"), "****");
    }

    #[test]
    fn lists_named_profiles_sorted() {
        let config = config(
            r#"
rest_uri = "https://api.siftstack.com"

[staging]
rest_uri = "https://api.staging.example.net"

[prod]
rest_uri = "https://api.example.net"
"#,
        );
        assert_eq!(named_profiles(&config), ["prod", "staging"]);
        assert!(named_profiles(&Table::new()).is_empty());
    }

    #[test]
    fn reports_only_changed_fields_with_the_key_masked() {
        let before = config(
            r#"
rest_uri = "https://api.example.net"
apikey = "sift_old_key_a91f"
app_uri = "https://sift.example.net"
"#,
        );
        let mut after = before.clone();
        apply_profile_updates(
            &mut after,
            None,
            Some("grpc.example.net:443".to_string()),
            Some("https://api.example.net".to_string()),
            Some("sift_new_key_7c02".to_string()),
            None,
        )
        .unwrap();

        assert_eq!(
            profile_changes(&before, &after, None),
            [
                change("grpc_uri", "(unset)", "grpc.example.net:443"),
                change("api_key", "****a91f", "****7c02"),
            ]
        );
    }

    #[test]
    fn a_new_named_profile_starts_unset() {
        let before = config("rest_uri = \"https://api.siftstack.com\"\n");
        let mut after = before.clone();
        apply_profile_updates(
            &mut after,
            Some("staging".to_string()),
            None,
            Some("https://api.staging.example.net".to_string()),
            None,
            None,
        )
        .unwrap();

        assert_eq!(
            profile_changes(&before, &after, Some("staging")),
            [change(
                "rest_uri",
                "(unset)",
                "https://api.staging.example.net"
            )]
        );
        assert!(profile_changes(&before, &after, None).is_empty());
    }

    #[test]
    fn default_profile_name_means_the_top_level_profile() {
        let top_level = config("rest_uri = \"https://api.siftstack.com\"\n");
        assert_eq!(
            resolve_profile(&top_level, Some("default".to_string())),
            None
        );
        assert_eq!(
            resolve_profile(&top_level, Some("staging".to_string())).as_deref(),
            Some("staging")
        );

        let table = config("[default]\nrest_uri = \"https://api.siftstack.com\"\n");
        assert_eq!(
            resolve_profile(&table, Some("default".to_string())).as_deref(),
            Some("default")
        );
    }
}
