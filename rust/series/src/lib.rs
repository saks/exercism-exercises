pub fn series(digits: &str, len: usize) -> Vec<String> {
    let digits_len = digits.len();

    if len > digits_len {
        return vec![];
    }

    let n = digits_len - len + 1;
    let mut result = Vec::with_capacity(n);

    for i in 0..n {
        result.push(String::from(&digits[i..i + len]))
    }

    result
}
