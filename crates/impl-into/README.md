[![github]](https://github.com/fuderis/macron-rs/tree/main/crates/impl-into)&ensp;
[![crates-io]](https://crates.io/crates/macron-impl-into)&ensp;
[![docs-rs]](https://docs.rs/macron-impl-into)

[github]: https://img.shields.io/badge/github-8da0cb?style=for-the-badge&labelColor=555555&logo=github
[crates-io]: https://img.shields.io/badge/crates.io-fc8d62?style=for-the-badge&labelColor=555555&logo=rust
[docs-rs]: https://img.shields.io/badge/docs.rs-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs

# Impl Into Macro

The implementation of trait [Into](std::convert::Into).

> See more macros: [docs](https://docs.rs/macron), [repository](https://github.com/fuderis/macron-rs).

## Examples:

### Struct
```rust
#[derive(Into, Debug, PartialEq)]
pub struct Email(String);

let email = Email("user@example.com".to_string());
let email_str: String = email.into();

assert_eq!("user@example.com", &email_str);
```

### Enum
```rust
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
```

## License & Feedback:

> This library distributed under the [MIT](https://github.com/fuderis/macron-rs/blob/main/LICENSE.md) license.

You can contact me via [GitHub](https://github.com/fuderis) or send a message to my [E-Mail](mailto:synapdrake@ya.ru).
This library is actively evolving, and your suggestions and feedback are always welcome!
