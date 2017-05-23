use std::char;
use std::str::FromStr;

pub fn encode(text: &str) -> String {
    let mut res = String::new();
    let mut rep = 1u32;
    let mut last_char = 0 as char;

    for ch in text.chars() {
        if last_char == ch {
            rep += 1;
            continue;
        } else {
            if rep > 1 {
                res.push_str(rep.to_string().as_str());
                res.push(last_char);
                rep = 1;
            } else if last_char != 0 as char {
                res.push(last_char);
            }

            last_char = ch;
        };
    }

    if rep > 1 {
        res.push_str(rep.to_string().as_str());
    };

    if last_char != 0 as char {
        res.push(last_char);
    };

    res
}

fn unzip(n_buf: &mut String, res: &mut String, ch: char) {
    let n = i32::from_str(n_buf.as_str()).unwrap();
    for _ in 0..n {
        res.push(ch)
    }
    n_buf.clear();
}

pub fn decode(text: &str) -> String {
    let mut res = String::with_capacity(text.len());
    let mut n_buf = String::new();

    for ch in text.chars() {
        if ch.is_numeric() {
            n_buf.push(ch);
        } else if n_buf.is_empty() {
            res.push(ch);
        } else {
            unzip(&mut n_buf, &mut res, ch);
        }
    }

    res
}
