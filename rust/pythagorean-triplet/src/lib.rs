pub fn find() -> Option<i32> {
    for a in 1..999i32 {
        for b in a..999i32 - a {
            let c = 1000 - a - b;
            if a.pow(2) + b.pow(2) == c.pow(2) {
                return Some(a * b * c);
            }
        }
    }

    None
}
