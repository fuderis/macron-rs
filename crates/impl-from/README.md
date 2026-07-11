[![github]](https://github.com/fuderis/macron-rs/tree/main/crates/impl-from)&ensp;
[![crates-io]](https://crates.io/crates/macron-impl-from)&ensp;
[![docs-rs]](https://docs.rs/macron-impl-from)

[github]: https://img.shields.io/badge/github-8da0cb?style=for-the-badge&labelColor=555555&logo=github
[crates-io]: https://img.shields.io/badge/crates.io-fc8d62?style=for-the-badge&labelColor=555555&logo=rust
[docs-rs]: https://img.shields.io/badge/docs.rs-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs

# Impl From Macro

The implementation of trait [From](std::convert::From).

> See more macros: [docs](https://docs.rs/macron), [repository](https://github.com/fuderis/macron-rs).

## Examples

### Struct
```rust
#[derive(From, Debug, PartialEq)]
#[from(&str, expr = value.to_string().into())]
pub struct Email(String);

assert_eq!(
    Email::from("user@example.com".to_string()),
    "user@example.com".into()
);
```

### Enum
```rust
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
```

## License & Feedback

> This library is distributed under the [MIT](https://github.com/fuderis/macron-rs/blob/main/LICENSE.md) license.

You can contact me via [GitHub](https://github.com/fuderis) or send a message to my [E-Mail](mailto:synapdrake@ya.ru).
This library is actively evolving, and your suggestions and feedback are always welcome!
