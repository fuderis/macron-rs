#![allow(dead_code)]
use macron_impl_from::From;

#[test]
fn test_struct_from() {
    #[derive(From, Debug, PartialEq)]
    #[from(&str, expr = value.to_string().into())]
    pub struct Email(String);

    assert_eq!(
        Email::from("user@example.com".to_string()),
        "user@example.com".into()
    );
}

#[test]
fn test_enum_from() {
    #[derive(From, Debug, PartialEq)]
    pub enum Value {
        Integer(i32),

        #[from(skip)]
        Integer2(i32),

        Float(f64),

        #[from((), expr = Self::Null)]
        Null,

        #[from(&str, with = String::from)]
        String(String),
    }

    assert_eq!(Value::Integer(42), 42.into());
    assert_eq!(Value::Float(3.14), 3.14.into());
    assert_eq!(Value::Null, ().into());
    assert_eq!(Value::String("test".to_string()), "test".into());
}
