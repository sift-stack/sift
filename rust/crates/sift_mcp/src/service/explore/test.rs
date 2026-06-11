use super::*;

const REST_URI: &str = "https://api.siftstack.com";

fn service() -> ExploreService {
    ExploreService::new(REST_URI.to_string())
}

#[test]
fn full_url_with_all_params() {
    let url = service()
        .build_url(ExploreUrlRequest {
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
        .build_url(ExploreUrlRequest {
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
        .build_url(ExploreUrlRequest {
            channels: Some(vec![String::from("weird,name")]),
            ..Default::default()
        })
        .unwrap();
    assert!(url.contains("&channels=weird%2Cname"), "got {url}");
}

#[test]
fn unknown_panel_type_is_rejected() {
    let err = service()
        .build_url(ExploreUrlRequest {
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
        .build_url(ExploreUrlRequest::default())
        .unwrap_err();
    assert_eq!(err.code.0, -32602);
}

#[test]
fn empty_vecs_are_treated_as_missing() {
    let err = service()
        .build_url(ExploreUrlRequest {
            assets: Some(vec![]),
            runs: Some(vec![]),
            channels: Some(vec![]),
            ..Default::default()
        })
        .unwrap_err();
    assert_eq!(err.code.0, -32602);
}

#[test]
fn host_derivation_strips_rest_uri_path() {
    let svc = ExploreService::new(String::from("https://api.siftstack.com/v1"));
    let url = svc
        .build_url(ExploreUrlRequest {
            assets: Some(vec![String::from("a")]),
            ..Default::default()
        })
        .unwrap();
    assert!(
        url.starts_with("https://app.siftstack.com/explore?"),
        "got {url}"
    );
}

#[test]
fn unsupported_rest_uri_without_explore_host_errors() {
    let svc = ExploreService::new(String::from("https://my-self-hosted.example"));
    let err = svc
        .build_url(ExploreUrlRequest {
            assets: Some(vec![String::from("a")]),
            ..Default::default()
        })
        .unwrap_err();
    assert_eq!(err.code.0, -32602);
    assert!(
        err.message.contains("explore_host"),
        "expected guidance to point at explore_host, got `{}`",
        err.message
    );
}
