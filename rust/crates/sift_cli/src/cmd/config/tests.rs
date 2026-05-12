mod test_is_update_empty {
    use super::super::is_update_empty;
    use crate::cli::ConfigUpdateArgs;

    fn args(
        grpc_uri: Option<&str>,
        rest_uri: Option<&str>,
        api_key: Option<&str>,
    ) -> ConfigUpdateArgs {
        ConfigUpdateArgs {
            interactive: false,
            grpc_uri: grpc_uri.map(String::from),
            rest_uri: rest_uri.map(String::from),
            api_key: api_key.map(String::from),
        }
    }

    #[test]
    fn no_flags_is_empty() {
        assert!(is_update_empty(&args(None, None, None)));
    }

    #[test]
    fn all_empty_strings_is_empty() {
        assert!(is_update_empty(&args(Some(""), Some(""), Some(""))));
    }

    #[test]
    fn any_single_flag_is_not_empty() {
        assert!(!is_update_empty(&args(Some("g"), None, None)));
        assert!(!is_update_empty(&args(None, Some("r"), None)));
        assert!(!is_update_empty(&args(None, None, Some("k"))));
    }

    #[test]
    fn all_flags_set_is_not_empty() {
        assert!(!is_update_empty(&args(Some("g"), Some("r"), Some("k"))));
    }
}
