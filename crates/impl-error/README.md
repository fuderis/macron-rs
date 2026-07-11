[![github]](https://github.com/fuderis/macron-rs/tree/main/crates/impl-error)&ensp;
[![crates-io]](https://crates.io/crates/macron-impl-error)&ensp;
[![docs-rs]](https://docs.rs/macron-impl-error)

[github]: https://img.shields.io/badge/github-8da0cb?style=for-the-badge&labelColor=555555&logo=github
[crates-io]: https://img.shields.io/badge/crates.io-fc8d62?style=for-the-badge&labelColor=555555&logo=rust
[docs-rs]: https://img.shields.io/badge/docs.rs-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs

# Impl Error Macro

## Introduction

The implementation of trait [Error](std::error::Error).

> See more macros: [docs](https://docs.rs/macron), [repository](https://github.com/fuderis/macron-rs).

## Examples

### Struct
```rust
#[derive(Debug, Error)]
struct Error {
    source: Option<Box<dyn std::error::Error>>,
}

impl ::std::fmt::Display for Error {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "it's my custom error")
    }
}

let err = Error { source: None };
assert_eq!(format!("{err}"), "it's my custom error");
```

### Enum
```rust
#[derive(Debug, Error)]
enum Error {
    WithAnySource {
        source: Option<Box<dyn std::error::Error>>,
        msg: String,
    },

    WithCertainSource(#[source] Box<SourceError>),

    WithoutSource,

    WithoutSource2(String, String),
}

impl ::std::fmt::Display for Error {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match &self {
            Self::WithAnySource { source, msg } => write!(
                f,
                "the error with a message: '{msg}' and a source: '{src}'",
                src = if let Some(src) = source.as_deref() {
                    src.to_string()
                } else {
                    "no source".to_owned()
                }
            ),
            Self::WithCertainSource(src) => write!(f, "the error with the source: '{src}'"),
            Self::WithoutSource => write!(f, "the error without a source"),
            Self::WithoutSource2(_, _) => write!(f, "the error without source"),
        }
    }
}

#[derive(Debug, Error)]
enum SourceError {
    Error,
}

impl ::std::fmt::Display for SourceError {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match &self {
            Self::Error => write!(f, "the source of error"),
        }
    }
}

let err = Error::WithAnySource {
    source: Some(Box::new(SourceError::Error)),
    msg: "the message of error".to_owned(),
};
let err2 = Error::WithCertainSource(SourceError::Error.into());
let err3 = Error::WithoutSource;

assert_eq!(
    format!("{err}"),
    "the error with a message: 'the message of error' and a source: 'the source of error'"
);
assert_eq!(
    format!("{err2}"),
    "the error with the source: 'the source of error'"
);
assert_eq!(format!("{err3}"), "the error without a source");
```

## License & Feedback

> This library is distributed under the [MIT](https://github.com/fuderis/macron-rs/blob/main/LICENSE.md) license.

You can contact me via [GitHub](https://github.com/fuderis) or send a message to my [E-Mail](mailto:synapdrake@ya.ru).
This library is actively evolving, and your suggestions and feedback are always welcome!
