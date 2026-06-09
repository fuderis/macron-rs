#![doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/README.md"))]

#[cfg(feature = "regex")]
pub use regex::{self, Match, Regex};

/// Creates a new instance of [Regex](https://docs.rs/regex/latest/regex/struct.Regex.html)
#[macro_export]
macro_rules! re {
    ($($tokens:tt)*) => {{
        #[cfg(feature = "regex")] {
            $crate::regex::Regex::new(&::std::format!($($tokens)*)).unwrap()
        }
        #[cfg(not(feature = "regex"))] {
            ::regex::Regex::new(&::std::format!($($tokens)*)).unwrap()
        }
    }};

    ($expr:expr) => {{
        #[cfg(feature = "regex")] {
            $crate::regex::Regex::new(&::std::format!($expr)).unwrap()
        }
        #[cfg(not(feature = "regex"))] {
            ::regex::Regex::new(&::std::format!($expr)).unwrap()
        }
    }};
}
