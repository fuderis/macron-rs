#![doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/README.md"))]

#[cfg(feature = "regex")]
pub use regex::{self, Match, Regex};

/// Creates a new instance of [Regex](https://docs.rs/regex/latest/regex/struct.Regex.html)
#[cfg(feature = "regex")]
#[macro_export]
macro_rules! re {
    ($($tokens:tt)*) => {{
        $crate::regex::Regex::new(&::std::format!($($tokens)*)).unwrap()
    }};

    ($expr:expr) => {{
        $crate::regex::Regex::new(&::std::format!($expr)).unwrap()
    }};
}

/// Creates a new instance of [Regex](https://docs.rs/regex/latest/regex/struct.Regex.html)
#[cfg(not(feature = "regex"))]
#[macro_export]
macro_rules! re {
    ($($tokens:tt)*) => {{
        ::regex::Regex::new(&::std::format!($($tokens)*)).unwrap()
    }};

    ($expr:expr) => {{
        ::regex::Regex::new(&::std::format!($expr)).unwrap()
    }};
}
