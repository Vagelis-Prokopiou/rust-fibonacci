pub fn fibonacci(n: &i64) -> i64 {
    if *n == 1 || *n == 2 {
        return 1;
    }

    return fibonacci(&(*n - 1)) + fibonacci(&(*n - 2));
}

fn main() {
    println!("{}", fibonacci(&5));
}

#[cfg(test)]
mod tests {
    use super::fibonacci;

    #[test]
    fn wordks() {
        assert_eq!(fibonacci(&3), 2);
        assert_eq!(fibonacci(&5), 5);
        assert_eq!(fibonacci(&23), 28657);
        assert_eq!(fibonacci(&29), 514229);
    }
}