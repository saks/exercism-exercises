/// Determines whether the supplied string is a valid ISBN number
pub fn is_valid_isbn(isbn: &str) -> bool {
    let input: String = isbn.chars()
        .filter(|ch| ch.is_digit(10) || &'X' == ch)
        .collect();

    if input.len() > 10 {
        return false;
    }

    let sum = input
        .chars()
        .enumerate()
        .fold(0, |sum, (i, ch)| {
            let n = match ch {
                '0'...'9' => ch.to_digit(10).unwrap(),
                _ if ch == 'X' && i == 9 => 10,
                _ => 0,
            };

            sum + (n * (10 - i as u32))
        });

    0 == sum % 11
}
