fn rotate_char_code(code: u8, n: u8) -> u8 {
    let pos = match code {
        65...90 => code - 65,
        97...122 => code - 97,
        _ => return code,
    };

    match pos + n {
        0...25 => code + n,
        _ => code + n - 26,
    }
}

pub fn rotate(input: &str, n: u8) -> String {
    let mut result = String::with_capacity(input.len());

    for ch in input.chars() {
        result.push(rotate_char_code(ch as u8, n) as char)
    }

    result
}
