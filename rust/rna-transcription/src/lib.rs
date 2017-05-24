#[derive(Debug, PartialEq)]
pub struct RibonucleicAcid {
    data: String,
}

#[derive(Debug, PartialEq)]
pub struct DeoxyribonucleicAcid {
    data: String,
}

fn dna_to_rna_c(c: char) -> char {
    match c {
        'G' => 'C',
        'C' => 'G',
        'T' => 'A',
        'A' => 'U',
        _ => c,
    }
}

fn dna_to_rna_b(b: &u8) -> u8 {
    match *b {
        71 => 67,
        67 => 71,
        84 => 65,
        65 => 85,
        _ => *b,
    }
}

impl RibonucleicAcid {
    pub fn new(data: &str) -> Self {
        Self { data: data.to_owned() }
    }
}

impl DeoxyribonucleicAcid {
    pub fn new(data: &str) -> Self {
        Self { data: data.to_owned() }
    }

    pub fn to_rna(self) -> RibonucleicAcid {
        let mut bytes = self.data.into_bytes();

        for b in &mut bytes {
            *b = dna_to_rna_b(b);
        }

        RibonucleicAcid { data: String::from_utf8(bytes).unwrap() }
    }

    pub fn to_rna3(&self) -> RibonucleicAcid {
        let mut new_data = String::with_capacity(self.data.len());

        for c in self.data.chars() {
            new_data.push(dna_to_rna_c(c));
        }

        RibonucleicAcid { data: new_data }
    }

    pub fn to_rna2(&self) -> RibonucleicAcid {
        RibonucleicAcid { data: self.data.chars().map(dna_to_rna_c).collect() }
    }
}
