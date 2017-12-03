pub fn number(text: &str) -> Option<String> {
    let mut result = String::with_capacity(10);

    for ch in text.chars() {
        if (ch as u8) < 58 && (ch as u8) > 47 {
            result.push(ch);
        }
    }

    if result.len() == 11 && result.starts_with('1') {
        result = String::from(&result[1..]);
    }

    if result.len() != 10 || result.starts_with('0') ||
        result.starts_with('1') || result[3..].starts_with('0') ||
        result[3..].starts_with('1')
    {
        return None;
    }

    Some(result)
}
