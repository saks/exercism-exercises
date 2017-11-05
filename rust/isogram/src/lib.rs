pub fn check(word: &str) -> bool {
    let mut mask: u32 = 0x0;

    for ch in word.chars() {
        let code = ch as u8;

        let pos = match code {
            97...122 => code - 32 - 65,
            65...91 => code - 65,
            _ => continue,
        };

        if 0 == (mask & (1 << pos)) {
            mask |= 1 << pos;
        } else {
            return false;
        }
    }
    true
}
