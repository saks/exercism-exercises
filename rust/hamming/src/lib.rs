pub fn hamming_distance(str1: &str, str2: &str) -> Result<usize, ()> {
    if str1.len() != str2.len() {
        return Err(());
    }

    Ok(str1.chars()
           .zip(str2.chars())
           .filter(|&(a, b)| a != b)
           .count())
}
