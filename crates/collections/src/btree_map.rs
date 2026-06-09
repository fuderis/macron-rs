/// Creates a collection [BTreeMap](std::collections::BTreeMap)
#[macro_export]
macro_rules! btree_map {
    () => { ::std::collections::BTreeMap::new() };
    ($($tt:tt)*) => { ::std::collections::BTreeMap::from( $crate::macron_map::parse_map!( $($tt)* ) ) };
}
