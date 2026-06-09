/// Creates a collection [VecDeque](std::collections::VecDeque)
#[macro_export]
macro_rules! vec_deque {
    () => { ::std::collections::VecDeque::new() };
    
    ($($v:expr),* $(,)?) => {{
        let mut set = ::std::collections::VecDeque::new();
        $(set.push_back($v);)*
        set
    }};
}
