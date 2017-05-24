pub fn primes_up_to(n: usize) -> Vec<usize> {
    match n {
        0 | 1 => return vec![],
        2 => return vec![2],
        _ => {}
    }

    let mut buf: Vec<usize> = Vec::with_capacity(n);
    for _ in 0..n + 1 {
        buf.push(0);
    }
    buf[0] = 1;
    buf[1] = 1;

    for p in 2..n {
        let mut mult: usize = 2;
        let mut excluded = mult * p;

        while excluded < n + 1 {
            buf[excluded] = 1;
            mult += 1;
            excluded = mult * p;
        }
    }

    let mut result: Vec<usize> = Vec::new();
    for (i, n) in buf.iter().enumerate() {
        if n == &0 {
            result.push(i);
        }
    }

    result
}
