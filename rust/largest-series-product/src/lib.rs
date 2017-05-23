fn product_all(input: &str) -> Result<u32, ()> {
    let mut res = 1;

    for ch in input.chars() {
        if let Some(n) = ch.to_digit(10) {
            res *= n;
        } else {
            return Err(());
        }
    }

    Ok(res)
}

pub fn lsp(input: &str, length: usize) -> Result<u32, ()> {
    if length > input.len() {
        return Err(());
    }

    if 0 == length {
        return Ok(1);
    }

    let input_len = input.len();

    let mut res = 0;

    for i in 0..input_len - length + 1 {
        let slice = &input[i..i + length];
        let prod = product_all(slice)?;

        if res < prod {
            res = prod;
        }
    }

    Ok(res)
}
