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
