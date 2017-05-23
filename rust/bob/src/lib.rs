pub fn reply(text: &str) -> &'static str {
    let bytes = text.as_bytes();
    let len = bytes.len();

    if 0 == len {
        return "Fine. Be that way!";
    }

    // "?"
    if 63 == bytes[len - 1] {
        return "Sure.";
    }

    let mut is_yelling = true;

    for char_code in bytes {
        if *char_code > 96 && *char_code < 123 {
            is_yelling = false;
            break;
        }
    }

    if is_yelling {
        return "Whoa, chill out!";
    }

    "Whatever."
}
