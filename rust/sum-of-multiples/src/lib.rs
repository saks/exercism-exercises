pub fn sum_of_multiples(n: i32, mult: &[i32]) -> i32 {
    let is_multiple = |i: &i32| mult.iter().any(|x| 0 == (i % x));

    (1..n).filter(is_multiple).sum()
}
