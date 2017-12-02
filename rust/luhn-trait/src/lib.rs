use std::fmt::Display;

pub trait Luhn {
    fn valid_luhn(&self) -> bool;
}

impl<T: Display> Luhn for T {
    fn valid_luhn(&self) -> bool {
        is_valid(self.to_string())
    }
}

fn is_valid(data: String) -> bool {
    let mut sum: u32 = 0;
    let mut is_even = true;
    let mut len: usize = 0;

    for ch in data.chars().rev() {
        let code = ch as u8;
        match code {
            48...57 => {}
            32 => continue, // space char
            _ => return false,
        }

        let num = u32::from(code) - 48;
        len += 1;

        match num {
            _ if is_even => sum += num,
            0...4 if !is_even => sum += 2 * num,
            5...9 if !is_even => sum += 2 * num - 9,
            _ => panic!("unexpected number"),
        }

        is_even = !is_even;
    }

    len > 1 && (0 == sum % 10)
}
