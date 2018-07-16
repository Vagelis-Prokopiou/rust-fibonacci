use std::collections::HashMap;

extern crate fibonacci;

fn main() {
    let mut hash_map: HashMap<i32, i32> = HashMap::new();
    let mut array: [i32; 47] = [0; 47];

//    println!("{}", fibonacci::fibonacci_with_hashmap_memoization(46, &mut hash_map));
//    println!("{}", fibonacci::fibonacci_with_array_memoization(46, &mut array));
    println!("{}", fibonacci::fibonacci_with_array_memoization_iterative(3));
}