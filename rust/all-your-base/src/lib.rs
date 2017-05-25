pub fn convert(number: &[u32], from_base: u32, to_base: u32) -> Result<Vec<u32>, ()> {
    // validate input and output base
    if from_base < 2 || to_base < 2 {
        return Err(());
    }

    // validate input number
    let max_d = from_base - 1;
    for d in number {
        if d > &max_d {
            return Err(());
        }
    }

    if number.is_empty() {
        return Ok(vec![]);
    }

    Ok(num_to_base(num_from(number, from_base), to_base))
}

fn num_from(number: &[u32], base: u32) -> u32 {
    number.iter().fold(0, |acc, value| acc * base + value)
}

fn num_to_base(mut num: u32, base: u32) -> Vec<u32> {
    let mut result = Vec::new();

    while num > 0 {
        let rem = num % base;
        num /= base;
        result.push(rem);
    }

    result.reverse();
    result
}
