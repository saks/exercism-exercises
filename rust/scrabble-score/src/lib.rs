fn get_score(ch: char) -> usize {
    match ch {
        'a' | 'e' | 'i' | 'o' | 'u' | 'l' | 'n' | 'r' | 's' | 't' | 'A' | 'E' | 'I' | 'O' |
        'U' | 'L' | 'N' | 'R' | 'S' | 'T' => 1,
        'd' | 'g' | 'D' | 'G' => 2,
        'b' | 'c' | 'm' | 'p' | 'B' | 'C' | 'M' | 'P' => 3,
        'f' | 'h' | 'v' | 'w' | 'y' | 'F' | 'H' | 'V' | 'W' | 'Y' => 4,
        'k' | 'K' => 5,
        'j' | 'x' | 'J' | 'X' => 8,
        'q' | 'z' | 'Q' | 'Z' => 10,
        _ => 0,
    }
}


pub fn score(word: &str) -> usize {
    word.chars().map(get_score).sum()
}
