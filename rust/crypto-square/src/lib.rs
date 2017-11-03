fn normalize(s: &str) -> String {
    s.to_lowercase()
        .chars()
        .filter(|ch| (*ch as u16) > 96 && (*ch as u16) < 123)
        .collect()
}

pub fn encrypt(input: &str) -> String {
    let normalized = normalize(input);
    let len = normalized.len();

    if 0 == len {
        return String::new();
    }

    let r = (len as f32).sqrt() as usize;
    let c = match len - r.pow(2) {
        0 => r,
        _ => r + 1,
    };

    let mut result: Vec<&str> = Vec::new();

    for i in 0..c {
        let start = i * c;
        if start > len {
            break;
        }

        let mut end = (i + 1) * c;
        if end > len {
            end = len;
        };
        let end = end; // drop mutability
        result.push(&normalized[start..end]);
    }

    let mut encrypted = String::new();

    for ci in 0..c {
        for row in &result {
            if ci >= row.len() {
                break;
            }
            encrypted.push_str(&row[ci..ci + 1]);
        }
        if ci < c - 1 {
            encrypted.push(' ');
        }
    }

    encrypted
}
