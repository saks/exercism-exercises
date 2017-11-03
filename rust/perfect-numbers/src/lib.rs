use std::cmp::Ordering;

#[derive(Debug, PartialEq)]
pub enum Classification {
    Perfect,
    Abundant,
    Deficient,
}

pub fn aliquot_sum(num: u64) -> u64 {
    (1..num / 2 + 1).filter(|n| num % n == 0).sum()
}

pub fn classify(n: u64) -> Result<Classification, &'static str> {
    if n < 1 {
        return Err("Number must be positive");
    }

    match aliquot_sum(n).cmp(&n) {
        Ordering::Greater => Ok(Classification::Abundant),
        Ordering::Less => Ok(Classification::Deficient),
        Ordering::Equal => Ok(Classification::Perfect),
    }
}
