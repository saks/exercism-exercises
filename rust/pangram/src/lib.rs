pub fn is_pangram(text: &str) -> bool {
    let lower_text = text.to_lowercase();

    "abcdefghijklmnopqrstuvwxyz"
        .chars()
        .all(|ch: char| lower_text.chars().any(|text_ch: char| text_ch == ch))
}

pub fn is_pangram_vadim1(text: &str) -> bool {
    let mut mask: u32 = 0;
    for ch in text.chars() {
        let mut code = ch as u16; // convert to lower case
        // i.e. A -> a
        code |= 0x20;
        code += 0x100 - 0x61;
        code &= 0xFF;
        // ignore non-aphabet chars
        if code < 26 {
            mask |= 1 << code
        }
    }
    mask == 0x3ffffff
}

pub fn is_pangram_vadim2(text: &str) -> bool {
    let mut mask: u32 = 0;

    for ch in text.chars() {
        let code = ch as u32;
        if code & !0x3F == 0x40 {
            mask |= 1 << (code & 0x1F)
        }
    }

    mask & 0x7ff_fffe == 0x7ff_fffe
}

pub fn is_pangram_alex(text: &str) -> bool {
    let mut mask: u32 = 0;


    for ch in text.chars() {
        let mut code = ch as u8;

        // convert to upper case if this is lower case char,
        // i.e. a -> A
        if code >= 97 && code <= 122 {
            code -= 32;
        }

        // ignore non-aphabet chars
        if code < 65 || code > 90 {
            continue;
        }

        let pos = code - 65;
        mask |= 1 << pos
    }

    mask == 0x3ffffff
}
