pub fn primes_up_to(n: usize) -> Vec<usize> {
    match n {
        0 | 1 => return vec![],
        2 => return vec![2],
        _ => {}
    }

    let mut candidates = vec![0; n + 1];
    candidates[0] = 1;
    candidates[1] = 1;

    for p in 2..n {
        let mut mult = 2;
        let mut excluded = mult * p;

        while excluded < n + 1 {
            candidates[excluded] = 1;
            mult += 1;
            excluded = mult * p;
        }
    }

    candidates
        .iter()
        .enumerate()
        .filter_map(|(i, n)| if n == &0 { Some(i) } else { None })
        .collect()
}
