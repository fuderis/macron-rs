extern crate macron_str;  use macron_str::str;

#[test]
fn test_empty_str() {
    let s = str!();

    assert_eq!(s, String::new());
}

#[test]
fn test_str() {
    let s = str!("Hello, World!");

    assert_eq!(s, "Hello, World!");
}

#[test]
fn test_str_int() {
    let s = str!(10);

    assert_eq!(s, "10");
}

#[test]
fn test_str_reff() {
    let reff = 10.2;
    let s = str!(reff);

    assert_eq!(s, "10.2");
}

#[test]
fn test_format_str_with_args() {
    let s = str!("Hello, {}!", "World");

    assert_eq!(s, "Hello, World!");
}

#[test]
fn test_format_str_with_named_args() {
    let s = str!("Hello, {name}!", name = "World");

    assert_eq!(s, "Hello, World!");
}

#[test]
fn test_format_str_with_outer_args() {
    let name = "World";
    let s = str!("Hello, {name}!");

    assert_eq!(s, "Hello, World!");
}
