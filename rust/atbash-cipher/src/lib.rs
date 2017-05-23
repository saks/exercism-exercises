fn rotate_char_code(code: u8) -> u8 {
    122 - code + 97
}

pub fn encode(input: &str) -> String {
    let mut result = String::with_capacity(input.len());
    let mut count: usize = 0;

    for ch in input.chars() {
        let mut code = ch as u8;

        // 'A' -> 'a'
        if code >= 65 && code <= 90 {
            code += 32;
        }

        let encoded_code = match code {
            48...57 => code,
            97...122 => rotate_char_code(code),
            _ => continue,
        };

        result.push(encoded_code as char);

        if count == 4 {
            count = 0;
            result.push(' ');
        } else {
            count += 1;
        }
    }

    if result.len() > 5 && 0 == count {
        result.pop();
    }

    result
}

pub fn decode(input: &str) -> String {
    let mut result = String::with_capacity(input.len());

    for ch in input.chars() {
        match ch as u8 {
            code @ 97...122 => result.push(rotate_char_code(code) as char),
            48...57 => result.push(ch),
            _ => continue,
        }
    }

    result
}
