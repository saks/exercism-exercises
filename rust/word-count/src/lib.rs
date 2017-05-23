use std::collections::HashMap;
use std::str;

pub fn word_count(input: &str) -> HashMap<String, u32> {
    let mut result = HashMap::new();

    let words = input
        .split(|ch: char| !(ch.is_alphabetic() || ch.is_numeric()))
        .filter(|word| !word.is_empty())
        .map(|word| word.to_lowercase());

    for word in words {
        *result.entry(word).or_insert(0) += 1;
    }

    result
}
