use std::net::SocketAddr;

use crate::cmd::doc::doc_url;

fn addr(s: &str) -> SocketAddr {
    s.parse().expect("valid socket address")
}

#[test]
fn unspecified_bind_prints_clickable_localhost_url() {
    assert_eq!(doc_url(addr("0.0.0.0:3000")), "http://localhost:3000");
}

#[test]
fn explicit_host_is_preserved() {
    assert_eq!(
        doc_url(addr("192.168.1.10:8080")),
        "http://192.168.1.10:8080"
    );
}

#[test]
fn explicit_ipv6_host_is_bracketed() {
    assert_eq!(doc_url(addr("[::1]:8080")), "http://[::1]:8080");
}
