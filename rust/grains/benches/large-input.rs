#![feature(test)]

extern crate test;
extern crate grains;

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[bench]
    fn bench_add_two(b: &mut Bencher) {
        b.iter(|| grains::total())
    }
}
