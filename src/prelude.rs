pub use crate::*;

#[cfg(any(feature = "collections", feature = "full"))]
pub use crate::{hash_map as map, hash_set as set};

/// The dynamic error type
pub type DynError = Box<dyn std::error::Error + Send + Sync + 'static>;
/// The short result alias
pub type Result<T> = std::result::Result<T, DynError>;
/// The std result alias
pub use std::result::Result as StdResult;
