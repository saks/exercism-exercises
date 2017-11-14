use std::cmp::Ordering::*;

pub fn find<L, E>(input: L, n: E) -> Option<usize>
    where L: AsRef<[E]>,
          E: Ord
{
    let mut current = input.as_ref();
    let mut base = 0;

    loop {
        let (head, tail) = current.split_at(current.len() >> 1);

        if tail.is_empty() {
            return None;
        }

        match tail[0].cmp(&n) {
            Equal => return Some(base + head.len()),
            Less => {
                base += head.len() + 1;
                current = &tail[1..];
            }
            Greater => current = head,
        }
    }
}
