use std::collections::BTreeMap;

pub fn transform(input: &BTreeMap<i32, Vec<String>>) -> BTreeMap<String, i32> {
    let mut result = BTreeMap::new();

    for (score, words) in input.iter() {
        for word in words {
            result.insert(word.to_lowercase(), *score);
        }
    }

    result
}
