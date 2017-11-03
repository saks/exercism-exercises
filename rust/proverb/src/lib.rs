
pub fn build_proverb(list: Vec<&str>) -> String {
    let mut result = String::new();

    if list.is_empty() {
        return result;
    }

    for (i, word) in list.iter().enumerate() {
        if i + 1 < list.len() {
            result.push_str(&format!("For want of a {} the {} was lost.\n", word, list[i + 1]));
        }
    }

    if list.len() < 3 {
        result.push_str(&format!("And all for the want of a {}.", list[0]));
    } else {
        result.push_str("And all for the want of a horseshoe nail.");
    }

    result
}
