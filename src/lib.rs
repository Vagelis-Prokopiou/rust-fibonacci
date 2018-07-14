use std::collections::HashMap;

pub fn fibonacci_with_hashmap_memoization(n: i32, hash_map: &mut HashMap<i32, i32>) -> i32 {
    assert!(n > 0, true);

    if hash_map.contains_key(&n) {
        let b: &i32 = hash_map.get(&n).unwrap();
        return *b;
    }

    if n == 1 || n == 2 {
        return 1;
    }


    let result: i32 = fibonacci_with_hashmap_memoization(n - 1, hash_map) + fibonacci_with_hashmap_memoization(n - 2, hash_map);

    hash_map.insert(n, result);

    return result;
}

pub fn fibonacci_with_array_memoization(n: i32, array: &mut [i32]) -> i32 {
    assert!(n > 0, true);
    let index: usize = n as usize;
    let array_value: i32 = array[index];

    if array_value > 0 {
        return array_value;
    }

    if n == 1 || n == 2 {
        return 1;
    }

    let result: i32 = fibonacci_with_array_memoization(n - 1, array) + fibonacci_with_array_memoization(n - 2, array);

    array[index] = result;

    return result;
}