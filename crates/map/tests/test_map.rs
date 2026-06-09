extern crate macron_map;  use macron_map::map;

#[test]
fn test_map() {
    let (k, v) = ("one", 1);
    
    let map = map! {
        k => v,
        "two": 2,
        "three" => 3,
        "four": 4,
    };
    
    assert_eq!(map, [("one", 1), ("two", 2), ("three", 3), ("four", 4)])
}
