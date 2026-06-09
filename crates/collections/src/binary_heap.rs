/// Creates a collection [BinaryHeap](std::collections::BinaryHeap)
#[macro_export]
macro_rules! binary_heap {
    () => { ::std::collections::BinaryHeap::new() };
    
    ($($value:expr),* $(,)?) => {{
        let mut heap = ::std::collections::BinaryHeap::new();
        $(heap.push($value);)*
        heap
    }};
}
