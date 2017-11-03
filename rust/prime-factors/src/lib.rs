fn find_prime_factor(n: i64) -> Option<i64> {
    for i in 2..n + 1 {
        if n % i == 0 {
            return Some(i);
        }
    }

    None
}

pub fn factors(n: i64) -> Vec<i64> {
    let mut result = Vec::new();
    let mut current = n;

    while let Some(factor) = find_prime_factor(current) {
        result.push(factor);
        current /= factor;
    }

    result
}
