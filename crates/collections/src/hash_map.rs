/// Creates a collection [HashMap](std::collections::HashMap)
#[macro_export]
macro_rules! hash_map {
    () => { ::std::collections::HashMap::new() };
    ($($tokens:tt)*) => { ::std::collections::HashMap::from( $crate::macron_map::parse_map!($($tokens)*) ) };
}
