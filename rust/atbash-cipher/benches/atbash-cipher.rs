#![feature(test)]

extern crate test;
extern crate atbash_cipher as cipher;

static SENTENCE: &str = r#"
Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor
incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis
nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat.
Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu
fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in
culpa qui officia deserunt mollit anim id est laborum.
"#;

static ENCODED_SENTENCE: &str = r#"
olivn rkhfn wloli hrgzn vgxlm hvxgv gfizw rkrhx rmtvo rghvw wlvrf hnlwg vnkli
 rmxrw rwfmg fgozy livvg wloli vnztm zzorj fzfgv mrnzw nrmrn evmrz njfrh mlhgi
 fwvcv ixrgz grlmf ooznx lozyl irhmr hrfgz orjfr kvcvz xlnnl wlxlm hvjfz gwfrh
 zfgvr ifivw lolir mivki vsvmw virgr melof kgzgv evorg vhhvx roofn wloli vvfuf
 trzgm foozk zirzg fivcx vkgvf ihrmg lxxzv xzgxf krwzg zgmlm kilrw vmghf mgrmx
 fokzj frluu rxrzw vhvif mgnlo orgzm rnrwv hgozy lifn
"#;

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[bench]
    fn bench_encode(b: &mut Bencher) {
        b.iter(|| cipher::encode(SENTENCE));
    }

    // #[bench]
    // fn bench_encode2(b: &mut Bencher) {
    //     b.iter(|| cipher::encode2(SENTENCE));
    // }

    #[bench]
    fn bench_decode(b: &mut Bencher) {
        b.iter(|| cipher::decode(ENCODED_SENTENCE));
    }

    // #[bench]
    // fn bench_decode2(b: &mut Bencher) {
    //     b.iter(|| cipher::decode2(ENCODED_SENTENCE));
    // }
}
