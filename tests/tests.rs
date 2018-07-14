use std::collections::HashMap;

extern crate fibonacci;

#[test]
fn test_fibonacci_with_hashmap_memoization() {
    let mut hash_map: HashMap<i32, i32> = HashMap::new();

    assert_eq!(fibonacci::fibonacci_with_hashmap_memoization(3, &mut hash_map), 2);
    assert_eq!(fibonacci::fibonacci_with_hashmap_memoization(5, &mut hash_map), 5);
    assert_eq!(fibonacci::fibonacci_with_hashmap_memoization(23, &mut hash_map), 28657);
    assert_eq!(fibonacci::fibonacci_with_hashmap_memoization(29, &mut hash_map), 514229);
}

#[test]
fn test_fibonacci_with_array_memoization() {
    let mut array: [i32; 47] = [0; 47];

    assert_eq!(fibonacci::fibonacci_with_array_memoization(3, &mut array), 2);
    assert_eq!(fibonacci::fibonacci_with_array_memoization(5, &mut array), 5);
    assert_eq!(fibonacci::fibonacci_with_array_memoization(23, &mut array), 28657);
    assert_eq!(fibonacci::fibonacci_with_array_memoization(29, &mut array), 514229);
}