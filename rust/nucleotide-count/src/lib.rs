use std::collections::HashMap;

fn is_valid(nucleotides: &str) -> bool {
    !nucleotides
         .chars()
         .any(|n| (n != 'A') && (n != 'C') && (n != 'G') && (n != 'T'))
}

pub fn count(nucleotide: char, nucleotides: &str) -> Result<usize, ()> {
    if !is_valid(nucleotides) {
        return Err(());
    }

    match nucleotide {
        'A' | 'C' | 'G' | 'T' => Ok(nucleotides.chars().filter(|&n| n == nucleotide).count()),
        _ => Err(()),
    }
}

pub fn nucleotide_counts(nucleotides: &str) -> Result<HashMap<char, usize>, ()> {
    if !is_valid(nucleotides) {
        return Err(());
    }

    let mut res: HashMap<char, usize> = [('A', 0), ('C', 0), ('G', 0), ('T', 0)]
        .iter()
        .cloned()
        .collect();

    for nucleotide in nucleotides.chars() {
        let n = res.get_mut(&nucleotide).unwrap();
        *n += 1;
    }

    Ok(res)
}
