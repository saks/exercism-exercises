pub fn square(s: u32) -> u64 {
    assert!(s > 0 && s < 65, "Square must be between 1 and 64");
    0b01 << (s - 1) // i.e. 2.pow(s - 1)
}

pub fn total() -> u64 {
    (1..65).map(square).sum()
}
