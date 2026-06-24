//! Integration tests exercising the public API of `decamelize`.

use decamelize::{decamelize, decamelize_with};

#[test]
fn common_identifiers() {
    assert_eq!(decamelize("getHTTPResponseCode"), "get_http_response_code");
    assert_eq!(decamelize("innerHTML"), "inner_html");
    assert_eq!(decamelize("fooBarBaz"), "foo_bar_baz");
    assert_eq!(decamelize("camelCaseURL"), "camel_case_url");
}

#[test]
fn custom_separators() {
    assert_eq!(decamelize_with("getHTTPResponseCode", "-", false), "get-http-response-code");
    assert_eq!(decamelize_with("fooBarBaz", " ", false), "foo bar baz");
    assert_eq!(decamelize_with("fooBarBaz", "/", false), "foo/bar/baz");
}

#[test]
fn preserve_mode() {
    assert_eq!(decamelize_with("getHTTPResponseCode", "_", true), "get_HTTP_response_code");
    assert_eq!(decamelize_with("innerHTML", "_", true), "inner_HTML");
    assert_eq!(decamelize_with("parseDBURL", "_", true), "parse_DBURL");
}

#[test]
fn edge_cases() {
    assert_eq!(decamelize(""), "");
    assert_eq!(decamelize("a"), "a");
    assert_eq!(decamelize("already_snake"), "already_snake");
    assert_eq!(decamelize("UPPERCASE"), "uppercase");
}
