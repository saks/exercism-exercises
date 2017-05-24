#![feature(test)]

extern crate test;
extern crate rna_transcription as dna;

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[bench]
    fn bench_to_rna(b: &mut Bencher) {
        b.iter(|| dna::DeoxyribonucleicAcid::new("ACGTGGTCTTAA").to_rna());
    }

    #[bench]
    fn bench_to_rna2(b: &mut Bencher) {
        b.iter(|| dna::DeoxyribonucleicAcid::new("ACGTGGTCTTAA").to_rna2());
    }

    #[bench]
    fn bench_to_rna3(b: &mut Bencher) {
        b.iter(|| dna::DeoxyribonucleicAcid::new("ACGTGGTCTTAA").to_rna3());
    }
}
