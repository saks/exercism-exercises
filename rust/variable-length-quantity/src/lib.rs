const FIRST_7_BYTES: u32 = 0x7F;
const FIRST_7_BYTES_U8: u8 = 0x7F;
const EIGHTH_BYTE: u8 = 0x80;
const HIGH_7: u32 = 0b1111_1110_0000_0000_0000_0000_0000_0000;

#[inline]
fn can_lshift_7(n: u32) -> bool {
    (HIGH_7 & n) == 0
}

fn n_to_bytes(n: &u32) -> Vec<u8> {
    let mut n = *n;
    let mut result = vec![];

    result.push((n & FIRST_7_BYTES) as u8);
    n >>= 7;

    loop {
        if n == 0 {
            break;
        }
        result.push(EIGHTH_BYTE | ((n & FIRST_7_BYTES) as u8));
        n >>= 7;
    }

    result.reverse();
    result
}


/// Convert a list of numbers to a stream of bytes encoded with variable length encoding.
pub fn to_bytes(values: &[u32]) -> Vec<u8> {
    values.iter().flat_map(|n| n_to_bytes(n)).collect()
}

/// Given a stream of bytes, extract all numbers which are encoded in there.
pub fn from_bytes(bytes: &[u8]) -> Result<Vec<u32>, &'static str> {
    let mut result = vec![];
    let mut current_num: u32 = 0;
    let mut expecting_one_more = false;

    for b in bytes {
        current_num |= u32::from(*b & FIRST_7_BYTES_U8);

        if (b & EIGHTH_BYTE) == EIGHTH_BYTE {
            expecting_one_more = true;
            if !can_lshift_7(current_num) {
                return Err("Overflow!");
            };
            current_num <<= 7;
        } else {
            expecting_one_more = false;
            result.push(current_num);
            current_num = 0;
        }
    }

    if expecting_one_more {
        return Err("incomplete byte sequence!");
    }

    Ok(result)
}
