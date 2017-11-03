pub fn main() {
    for _ in 0..100 {
        'outer: for a in 1..999i32 {
            for b in a..999i32 - a {
                let c = 1000i32 - a - b;
                if a.pow(2) + b.pow(2) == c.pow(2) {
                    break 'outer;
                }
            }
        }
    }
}
