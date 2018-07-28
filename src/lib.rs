use std::collections::HashMap;

pub fn fibonacci_with_hashmap_memoization(n: u64, cache: &mut HashMap<u64, u64>) -> u64 {
    assert!(n > 0, true);

    if cache.contains_key(&n) {
        let result: &u64 = cache.get(&n).unwrap();
        return *result;
    }

    if n == 1 || n == 2 {
        return 1;
    }

    let result: u64 = fibonacci_with_hashmap_memoization(n - 1, cache) + fibonacci_with_hashmap_memoization(n - 2, cache);

    cache.insert(n, result);

    return result;
}

pub fn fibonacci_with_array_memoization(n: u64, cache: &mut [u64]) -> u64 {
    assert!(n > 0, true);
    let index: usize = n as usize;
    let array_value: u64 = cache[index];

    if array_value > 0 {
        return array_value;
    }

    if n == 1 || n == 2 {
        return 1;
    }

    let result: u64 = fibonacci_with_array_memoization(n - 1, cache) + fibonacci_with_array_memoization(n - 2, cache);

    cache[index] = result;

    return result;
}

pub fn fibonacci_iterative(n: u64) -> u64 {
    assert!(n > 0, true);
    if n == 1 || n == 2 { return 1; }

    let mut current: u64 = 0;
    let mut previous: u64 = 1;
    let mut previous2: u64 = 1;

    for _i in 3..n + 1 {
        current = previous + previous2;
        previous2 = previous;
        previous = current;
    }

    return current;
}
