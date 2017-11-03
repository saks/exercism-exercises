fn is_pryme(number: i32) -> bool {
    if number == 2 || number == 3 {
        return true;
    }

    if number % 2 == 0 || number % 3 == 0 {
        return false;
    }

    let mut divisor = 6;

    while divisor * divisor - 2 * divisor + 1 <= number {
        if number % (divisor - 1) == 0 {
            return false;
        }

        if number % (divisor + 1) == 0 {
            return false;
        }

        divisor += 6;
    }

    true
}

pub fn nth(n: i32) -> Result<i32, String> {
    if n < 1 {
        return Err(String::from("Input must be > 1!"));
    }

    let mut current_n = 0;

    for i in 2..std::i32::MAX {
        if is_pryme(i) {
            current_n += 1;
            if current_n == n {
                return Ok(i);
            }
        }
    }

    Err(String::from("Error!"))
}
