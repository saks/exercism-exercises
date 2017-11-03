extern crate crypto_square;
use crypto_square::encrypt;

fn test(input: &str, output: &str) {
    assert_eq!(&encrypt(input), output);
}

pub fn main() {
    // let s = "She got her education, then got rich programming: 👩‍🎓🎓👩‍💻💰";
    // for ch in s.chars() {
    //     println!("{:?}: {:?}", ch, ch as u16);
    // }


    // test("She got her education, then got rich programming: 👩‍🎓🎓👩‍💻💰",
    //      "setnhm hrigpm eeoori gdnton outrgg tchir haeca");
    test("congratulate", "crl oaa ntt gue")
    // test("If man was meant to stay on the ground, god would have given us roots.",
    //      "imtgdvs fearwer mayoogo anouuio ntnnlvt wttddes aohghn sseoau")
    // test("abet", "ae bt");
    // test("a bet", "ae bt");
    // test("     a  b     e      t             ", "ae bt");
    // let s = "If man was meant to stay on the ground, god would have given us roots.";
    // println!("{}", encrypt(s));
    // test("If man was meant to stay on the ground, god would have given us roots.",
    // "imtgdvs fearwer mayoogo anouuio ntnnlvt wttddes aohghn sseoau")
    // let example = "lime anda coco anut";
    // assert_eq!(example, &encrypt(&encrypt(example)));
    // println!("{:?}", &encrypt("lime anda coco anut"));
    // println!("{:?}", &encrypt(&encrypt("lime anda coco anut")));
    // let normalized: String = "lime anda coco anut"
    //     .to_lowercase()
    //     .chars()
    //     .filter(|ch| (*ch as u8) > 96 && (*ch as u8) < 123)
    //     .collect();
    //
    // let len = normalized.len() as f32;
    // let r = len.sqrt() as usize;
    // let c = (len as usize) / r;
    //
    // let mut result: Vec<&str> = Vec::new();
    //
    // println!("len: {}, c: {}, r: {}", len, c, r);
    //
    // for i in 0..c {
    //     result.push(&normalized[i * c..(i + 1) * c]);
    //     // println!("{}", &normalized[i * c..(i + 1) * c]);
    // }
    //
    // let mut encrypted = String::new();
    //
    // for i in 0..c {
    //     for j in 0..r {
    //         // println!("{}", &result[j][i..i + 1]);
    //         encrypted.push_str(&result[j][i..i + 1]);
    //     }
    //     if i < c - 1 {
    //         encrypted.push(' ');
    //     }
    // }
    // println!("{:?}", encrypted);
}
