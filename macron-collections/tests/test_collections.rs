use macron_collections::*;
use std::collections::*;

#[test]
fn test_vec_deque() {
    let mut deque: VecDeque<_> = vec_deque![1, 2, 3];

    assert_eq!(deque.len(), 3);
    assert_eq!(deque.pop_front(), Some(1));
    assert_eq!(deque.pop_back(), Some(3));
}

#[test]
fn test_hash_map() {
    let key = "one";
    let val = 1;

    let map: HashMap<_, _> = hash_map! {
        key => val,
        "two": 2,
        "three" => 3,
        "four": 4,
    };

    assert_eq!(map.get("one"), Some(&1));
    assert_eq!(map.get("two"), Some(&2));
    assert_eq!(map.get("three"), Some(&3));
    assert_eq!(map.get("four"), Some(&4));
}

#[test]
fn test_hash_set() {
    let set: HashSet<_> = hash_set![1, 2, 3];

    assert!(set.contains(&1));
    assert!(set.contains(&2));
    assert!(set.contains(&3));
}

#[test]
fn test_btree_map() {
    let key = "one";
    let val = 1;

    let map: BTreeMap<_, _> = btree_map! {
        key => val,
        "two": 2,
        "three" => 3,
        "four": 4,
    };

    assert_eq!(map.get("one"), Some(&1));
    assert_eq!(map.get("two"), Some(&2));
    assert_eq!(map.get("three"), Some(&3));
    assert_eq!(map.get("four"), Some(&4));
}

#[test]
fn test_btree_set() {
    let set: BTreeSet<_> = btree_set![4, 5, 6];

    assert!(set.contains(&4));
    assert!(set.contains(&5));
    assert!(set.contains(&6));
}

#[test]
fn test_binary_heap() {
    let mut heap = binary_heap![3, 1, 2];

    assert_eq!(heap.pop(), Some(3)); // max-heap
    assert_eq!(heap.pop(), Some(2));
    assert_eq!(heap.pop(), Some(1));
}

#[test]
fn test_linked_list() {
    let mut list = linked_list![10, 20, 30];

    assert_eq!(list.len(), 3);
    assert_eq!(list.pop_front(), Some(10));
    assert_eq!(list.pop_back(), Some(30));
}
