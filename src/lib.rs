#![doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/README.md"))]
pub mod extra;
pub mod prelude;

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
    binary_heap, btree_map, btree_set, hash_map, hash_set, linked_list, parse_map, vec_deque,
};

#[cfg(any(feature = "derive", feature = "full"))]
pub use macron_impl_display::Display;
#[cfg(any(feature = "derive", feature = "full"))]
pub use macron_impl_error::Error;
#[cfg(any(feature = "derive", feature = "full"))]
pub use macron_impl_from::From;
#[cfg(any(feature = "derive", feature = "full"))]
pub use macron_impl_into::Into;
