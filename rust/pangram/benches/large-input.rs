#![feature(test)]

extern crate test;
extern crate pangram;
static sentence: &str = "a quick movement of the enemy will jeopardize five gunboats";

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;


    #[bench]
    fn bench_alex(b: &mut Bencher) {
        b.iter(|| pangram::is_pangram_alex(&sentence))
    }

    #[bench]
    fn bench_vadim1(b: &mut Bencher) {
        b.iter(|| pangram::is_pangram_vadim1(&sentence))
    }

    #[bench]
    fn bench_vadim2(b: &mut Bencher) {
        b.iter(|| pangram::is_pangram_vadim2(&sentence))
    }
}
