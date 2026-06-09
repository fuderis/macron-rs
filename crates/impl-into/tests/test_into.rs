#![allow(dead_code)]
use macron_impl_into::Into;

#[test]
fn test_struct_into() {
    #[derive(Into, Debug, PartialEq)]
    pub struct Email(String);

    let email = Email("user@example.com".to_string());
    let email_str: String = email.into();

    assert_eq!("user@example.com", &email_str);
}

// ------------------------------------------------

#[test]
fn test_enum_into() {
    #[derive(Into)]
    pub enum Number {
        Integer(i32),

        #[into(i32, expr = value as i32)]
        Float(f64),

        #[into(i32, expr = 0)]
        Null,

        #[into(i32, with = Self::from_str)]
        Stringify(&'static str),
    }

    impl Number {
        fn from_str(s: &str) -> i32 {
            s.parse::<i32>().unwrap_or(0)
        }
    }

    assert_eq!(42, Number::Integer(42).into());
    assert_eq!(3, Number::Float(3.14).into());
    assert_eq!(0, Number::Null.into());
    assert_eq!(15, Number::Stringify("15").into());
}
