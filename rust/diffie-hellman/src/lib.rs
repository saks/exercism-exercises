extern crate rand;
use rand::distributions::{IndependentSample, Range};

pub fn private_key(p: u64) -> u64 {
    let primes = primes_up_to(p);
    let between = Range::new(0, primes.len() - 1);
    let mut rng = rand::thread_rng();

    primes[between.ind_sample(&mut rng)]
}

pub fn public_key(p: u64, g: u64, a: u64) -> u64 {
    (g.pow(a as u32)) % p
}

pub fn secret(p: u64, b_pub: u64, a: u64) -> u64 {
    b_pub.pow(a as u32) % p
}

pub fn primes_up_to(n: u64) -> Vec<u64> {
    match n {
        0 | 1 => return vec![],
        2 => return vec![2],
        _ => {}
    }

    let mut candidates = vec![0; n as usize + 1];
    candidates[0] = 1;
    candidates[1] = 1;

    for p in 2..n {
        let mut mult = 2;
        let mut excluded = mult * p;

        while excluded < n + 1 {
            candidates[excluded as usize] = 1;
            mult += 1;
            excluded = mult * p;
        }
    }

    candidates
        .iter()
        .enumerate()
        .filter_map(|(i, n)| if n == &0 { Some(i as u64) } else { None })
        .collect()
}
