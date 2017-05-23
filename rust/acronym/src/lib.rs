pub fn abbreviate(input: &str) -> String {
    let mut result = String::new();

    if let Some(first) = input.chars().nth(0) {
        result.push(first);
    }

    for i in 0..input.len() - 1 {
        let curr = input.chars().nth(i).unwrap();
        let peek = input.chars().nth(i + 1).unwrap();

        if curr == ' ' || curr == '-' || (curr.is_lowercase() && peek.is_uppercase()) {
            result.push(peek);
        }
    }

    result.to_uppercase()
}
