#![allow(dead_code)]
use macron_impl_error::Error;

#[test]
fn test_struct_error() {
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
}

#[test]
fn test_enum_error() {
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
}
