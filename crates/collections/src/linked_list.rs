/// Creates a collection [LinkedList](std::collections::LinkedList)
#[macro_export]
macro_rules! linked_list {
    () => { ::std::collections::LinkedList::new() };
    
    ($($v:expr),* $(,)?) => {{
        let mut set = ::std::collections::LinkedList::new();
        $(set.push_back($v);)*
        set
    }};
}
