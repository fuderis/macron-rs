/// Creates a collection [BTreeSet](std::collections::BTreeSet)
#[macro_export]
macro_rules! btree_set {
    () => { ::std::collections::BTreeSet::new() };
    
    ($($v:expr),* $(,)?) => {{
        let mut set = ::std::collections::BTreeSet::new();
        $(set.insert($v);)*
        set
    }};
}
