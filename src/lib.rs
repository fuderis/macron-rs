#![doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/README.md"))]

/// A dynamic error type
pub type DynError = Box<dyn std::error::Error + Send + Sync + 'static>;
/// A useful result alias
pub type Result<T> = std::result::Result<T, DynError>;

#[cfg(any(feature = "path", feature = "full"))]
pub use macron_path::path;

#[cfg(any(feature = "string", feature = "full"))]
pub use macron_regex::re;
#[cfg(any(feature = "string", feature = "full"))]
pub use macron_str::str;

#[cfg(any(feature = "input", feature = "full"))]
pub use macron_input::input;
#[cfg(any(feature = "input", feature = "full"))]
pub use macron_inputln::inputln;

#[cfg(any(feature = "collections", feature = "full"))]
pub use macron_collections::{
    binary_heap, btree_map, btree_set, hash_map, hash_set, linked_list, map, vec_deque,
};

#[cfg(any(feature = "derive", feature = "full"))]
pub use macron_impl_display::Display;
#[cfg(any(feature = "derive", feature = "full"))]
pub use macron_impl_error::Error;
#[cfg(any(feature = "derive", feature = "full"))]
pub use macron_impl_from::From;
#[cfg(any(feature = "derive", feature = "full"))]
pub use macron_impl_into::Into;
