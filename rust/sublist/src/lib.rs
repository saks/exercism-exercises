extern crate core;

#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Unequal,
    Sublist,
    Superlist,
}

fn is_sublist<T: PartialEq>(a: &[T], b: &[T]) -> bool {
    b.windows(a.len()).any(|w| w == a)
}

pub fn sublist<T: PartialEq>(a: &[T], b: &[T]) -> Comparison {
    let a_len = a.len();
    let b_len = b.len();

    match () {
        _ if a_len == 0 && b_len > 0 => Comparison::Sublist,
        _ if a_len > 0 && b_len == 0 => Comparison::Superlist,
        _ if a_len == b_len && a == b => Comparison::Equal,
        _ if is_sublist(a, b) => Comparison::Sublist,
        _ if is_sublist(b, a) => Comparison::Superlist,
        _ => Comparison::Unequal,
    }
}
