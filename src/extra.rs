/// Wraps an expression in a `Box::new()`.
/// Moves the value from the stack to the heap.
#[macro_export]
macro_rules! boxed {
    ($e:expr) => {
        ::std::boxed::Box::new($e)
    };
}

/// Wraps an expression in a `Box::pin()`.
/// Pins the value on the heap, required for self-referential structs and some futures.
#[macro_export]
macro_rules! pinned {
    ($e:expr) => {
        ::std::boxed::Box::pin($e)
    };
}

/// Wraps an expression in an `Arc::new()`.
/// Provides thread-safe shared ownership via Atomic Reference Counting.
#[macro_export]
macro_rules! arc {
    ($e:expr) => {
        ::std::sync::Arc::new($e)
    };
}

/// Wraps an expression in a `tokio::sync::Mutex::new()`.
/// Async-aware mutex that doesn't block the entire thread.
#[macro_export]
macro_rules! mutex {
    ($e:expr) => {
        ::tokio::sync::Mutex::new($e)
    };
}

/// Wraps an expression in an `Arc<tokio::sync::Mutex<T>>`.
/// The go-to pattern for shared mutable state in async Axum/Tokio apps.
#[macro_export]
macro_rules! arc_mutex {
    ($e:expr) => {
        ::std::sync::Arc::new(::tokio::sync::Mutex::new($e))
    };
}

/// Wraps an expression in a `std::sync::Mutex::new()`.
/// Standard blocking mutex for non-async logic or very short critical sections.
#[macro_export]
macro_rules! std_mutex {
    ($e:expr) => {
        ::std::sync::Mutex::new($e)
    };
}

/// Wraps an expression in an `Arc<std::sync::Mutex<T>>`.
/// Thread-safe shared state using standard blocking primitives.
#[macro_export]
macro_rules! std_arc_mutex {
    ($e:expr) => {
        ::std::sync::Arc::new(::std::sync::Mutex::new($e))
    };
}
