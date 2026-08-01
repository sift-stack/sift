use super::*;

const APP_URI: &str = "https://app.siftstack.com";

fn service() -> UrlService {
    UrlService::new(Some(APP_URI.to_string()))
}

#[test]
fn full_url_with_all_params() {
    let url = service()
        .build_explore_url(ExploreUrlRequest {
            assets: Some(vec![String::from("Engine-7")]),
            runs: Some(vec![String::from("2025-thrust-test")]),
            channels: Some(vec![String::from("temperature"), String::from("pressure")]),
            panel_type: Some(String::from("scatter-plot")),
            start_time_unix_nanos: Some(0),
            end_time_unix_nanos: Some(1_700_000_000_000_000_000),
            explore_host: None,
        })
        .unwrap();
    assert_eq!(
        url,
        "https://app.siftstack.com/explore?method=single\
         &assets=Engine-7\
         &runs=2025-thrust-test\
         &channels=temperature,pressure\
         &panelType=scatter-plot\
         &startTime=1970-01-01T00:00:00.000Z\
         &endTime=2023-11-14T22:13:20.000Z"
    );
}

#[test]
fn axis_prefix_colon_is_preserved() {
    let url = service()
        .build_explore_url(ExploreUrlRequest {
            channels: Some(vec![
                String::from("L1:temperature"),
                String::from("L2:pressure"),
            ]),
            ..Default::default()
        })
        .unwrap();
    assert!(
        url.contains("&channels=L1:temperature,L2:pressure"),
        "got {url}"
    );
}

#[test]
fn comma_inside_single_value_is_encoded() {
    let url = service()
        .build_explore_url(ExploreUrlRequest {
            channels: Some(vec![String::from("weird,name")]),
            ..Default::default()
        })
        .unwrap();
    assert!(url.contains("&channels=weird%2Cname"), "got {url}");
}

#[test]
fn unknown_panel_type_is_rejected() {
    let err = service()
        .build_explore_url(ExploreUrlRequest {
            assets: Some(vec![String::from("a")]),
            panel_type: Some(String::from("bogus")),
            ..Default::default()
        })
        .unwrap_err();
    assert_eq!(err.code.0, -32602);
    assert!(err.message.contains("bogus"), "got `{}`", err.message);
}

#[test]
fn empty_request_is_rejected() {
    let err = service()
        .build_explore_url(ExploreUrlRequest::default())
        .unwrap_err();
    assert_eq!(err.code.0, -32602);
}

#[test]
fn empty_vecs_are_treated_as_missing() {
    let err = service()
        .build_explore_url(ExploreUrlRequest {
            assets: Some(vec![]),
            runs: Some(vec![]),
            channels: Some(vec![]),
            ..Default::default()
        })
        .unwrap_err();
    assert_eq!(err.code.0, -32602);
}

#[test]
fn configured_app_uri_trims_a_trailing_slash() {
    let svc = UrlService::new(Some(String::from("https://sift.example.net/")));
    let url = svc
        .build_explore_url(ExploreUrlRequest {
            assets: Some(vec![String::from("a")]),
            ..Default::default()
        })
        .unwrap();
    assert!(
        url.starts_with("https://sift.example.net/explore?"),
        "got {url}"
    );
}

#[test]
fn slash_only_app_uri_is_treated_as_missing() {
    let svc = UrlService::new(Some(String::from(" / ")));
    let err = svc.build_report_url("rep-123").unwrap_err();
    assert_eq!(err.code.0, -32602);
}

#[test]
fn missing_app_uri_without_explore_host_errors() {
    let svc = UrlService::new(None);
    let err = svc
        .build_explore_url(ExploreUrlRequest {
            assets: Some(vec![String::from("a")]),
            ..Default::default()
        })
        .unwrap_err();
    assert_eq!(err.code.0, -32602);
    assert!(
        err.message.contains("app_uri") && err.message.contains("explore_host"),
        "expected profile and request guidance, got `{}`",
        err.message
    );
}

#[test]
fn slash_only_explore_host_uses_configured_app_uri() {
    let url = service()
        .build_explore_url(ExploreUrlRequest {
            assets: Some(vec![String::from("a")]),
            explore_host: Some(String::from(" / ")),
            ..Default::default()
        })
        .unwrap();
    assert!(
        url.starts_with("https://app.siftstack.com/explore?"),
        "got {url}"
    );
}

#[test]
fn explore_host_takes_priority_over_configured_app_uri() {
    let url = service()
        .build_explore_url(ExploreUrlRequest {
            assets: Some(vec![String::from("a")]),
            explore_host: Some(String::from("https://override.example.net/")),
            ..Default::default()
        })
        .unwrap();
    assert!(
        url.starts_with("https://override.example.net/explore?"),
        "got {url}"
    );
}

#[test]
fn report_url_uses_configured_app_uri() {
    let url = service().build_report_url("rep-123").unwrap();
    assert_eq!(url, "https://app.siftstack.com/reports/rep-123");
}

#[test]
fn report_url_errors_without_app_uri() {
    let svc = UrlService::new(None);
    let err = svc.build_report_url("rep-123").unwrap_err();
    assert_eq!(err.code.0, -32602);
}

#[test]
fn rule_url_uses_configured_app_uri() {
    let url = service().build_rule_url("rule-123").unwrap();
    assert_eq!(url, "https://app.siftstack.com/rules/rule-123");
}

#[test]
fn rule_url_errors_without_app_uri() {
    let svc = UrlService::new(None);
    let err = svc.build_rule_url("rule-123").unwrap_err();
    assert_eq!(err.code.0, -32602);
}

#[test]
fn annotation_asset_run_urls_use_singular_path_segments() {
    let svc = service();
    assert_eq!(
        svc.build_annotation_url("ann-1").unwrap(),
        "https://app.siftstack.com/annotation/ann-1"
    );
    assert_eq!(
        svc.build_asset_url("asset-1").unwrap(),
        "https://app.siftstack.com/asset/asset-1"
    );
    assert_eq!(
        svc.build_run_url("run-1").unwrap(),
        "https://app.siftstack.com/run/run-1"
    );
}

#[test]
fn annotation_asset_run_urls_error_without_app_uri() {
    let svc = UrlService::new(None);
    assert_eq!(
        svc.build_annotation_url("ann-1").unwrap_err().code.0,
        -32602
    );
    assert_eq!(svc.build_asset_url("asset-1").unwrap_err().code.0, -32602);
    assert_eq!(svc.build_run_url("run-1").unwrap_err().code.0, -32602);
}
