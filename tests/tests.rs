use std::collections::HashMap;

extern crate fibonacci;

#[test]
fn test_fibonacci_with_hashmap_memoization() {
    let mut hash_map: HashMap<i64, i64> = HashMap::new();

    assert_eq!(fibonacci::fibonacci_with_hashmap_memoization(3, &mut hash_map), 2);
    assert_eq!(fibonacci::fibonacci_with_hashmap_memoization(5, &mut hash_map), 5);
    assert_eq!(fibonacci::fibonacci_with_hashmap_memoization(23, &mut hash_map), 28657);
    assert_eq!(fibonacci::fibonacci_with_hashmap_memoization(29, &mut hash_map), 514229);
}

#[test]
fn test_fibonacci_with_array_memoization() {
    let mut array: [i64; 47] = [0; 47];

    assert_eq!(fibonacci::fibonacci_with_array_memoization(3, &mut array), 2);
    assert_eq!(fibonacci::fibonacci_with_array_memoization(5, &mut array), 5);
    assert_eq!(fibonacci::fibonacci_with_array_memoization(23, &mut array), 28657);
    assert_eq!(fibonacci::fibonacci_with_array_memoization(29, &mut array), 514229);
}

#[test]
fn test_fibonacci_with_array_memoization_iterative() {
    assert_eq!(fibonacci::fibonacci_with_array_memoization_iterative(2), 1);
    assert_eq!(fibonacci::fibonacci_with_array_memoization_iterative(3), 2);
    assert_eq!(fibonacci::fibonacci_with_array_memoization_iterative(5), 5);
    assert_eq!(fibonacci::fibonacci_with_array_memoization_iterative(23), 28657);
    assert_eq!(fibonacci::fibonacci_with_array_memoization_iterative(29), 514229);
    assert_eq!(fibonacci::fibonacci_with_array_memoization_iterative(60), 1548008755920);
}
