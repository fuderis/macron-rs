#![doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/README.md"))]
pub mod extra;

#[cfg(feature = "path")]
pub use macron_path::path;

#[cfg(feature = "string")]
pub use macron_regex::re;
#[cfg(feature = "string")]
pub use macron_str::str;

#[cfg(feature = "input")]
pub use macron_input::input;
#[cfg(feature = "input")]
pub use macron_inputln::inputln;

#[cfg(feature = "collections")]
pub use macron_collections::{
    binary_heap, btree_map, btree_set, hash_map, hash_map as map, hash_set, hash_set as set,
    linked_list, parse_map, vec_deque,
};

#[cfg(feature = "derive")]
pub use macron_impl_display::Display;
#[cfg(feature = "derive")]
pub use macron_impl_error::Error;
#[cfg(feature = "derive")]
pub use macron_impl_from::From;
#[cfg(feature = "derive")]
pub use macron_impl_into::Into;

#[cfg(feature = "async-recursion")]
pub use async_recursion::async_recursion;
