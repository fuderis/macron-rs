#![cfg(feature = "regex")]

extern crate macron_regex;
use macron_regex::re;

#[test]
fn test_regex() {
    let re = re!(r"^Hello, \w+!$");
    assert!(re.is_match("Hello, World!"));
}

#[test]
fn test_regex_format() {
    let re = re!(r"^Hello, {}!$", "World");
    assert!(re.is_match("Hello, World!"));
}
