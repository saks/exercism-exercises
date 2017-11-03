const HYPEN: &str = "-";
const SPACE: &str = " ";
const WORDS: &[&str] = &["",
                         "one",
                         "two",
                         "three",
                         "four",
                         "five",
                         "six",
                         "seven",
                         "eight",
                         "nine",
                         "ten",
                         "eleven",
                         "twelve",
                         "therteen",
                         "fourteen",
                         "fifteen",
                         "sixteen",
                         "seventeen",
                         "eighteen",
                         "nineteen"];

const TENS: &[&str] = &["", "ten", "twenty", "thirty", "forty", "fifty",
                        "sixty", "seventy", "eighty", "ninety"];
const TEN_POWERS: &[&str] = &["",
                              "ten",
                              "hundred",
                              "thousand",
                              "",
                              "",
                              "million",
                              "",
                              "",
                              "billion",
                              "",
                              "",
                              "trillion",
                              "",
                              "",
                              "quadrillion",
                              "",
                              "",
                              "quintillion"];

pub fn encode(n: i64) -> String {
    if 0 == n {
        String::from("zero")
    } else if n < 0 {
        String::from("won't compile")
    } else {
        Speller::new().spell(n as u64).concat()
    }
}

#[derive(Default)]
pub struct Speller {
    result: Vec<&'static str>,
}

impl Speller {
    pub fn new() -> Speller {
        Speller { result: Vec::new() }
    }

    pub fn from(result: Vec<&'static str>) -> Speller {
        Speller { result: result }
    }

    fn spell_big_num(&mut self, number_of_tens: u64, power: &u32) {
        let sub_result = Speller::new().spell(number_of_tens);
        for word in sub_result {
            self.result.push(word);
        }
        self.add_space();
        self.spell_tens_power(*power);
        self.add_space();
    }

    fn slice_big_nums(&mut self, n: &mut u64) {
        for power in &[18, 15, 12, 9, 6, 3, 2] {
            let ten: u64 = 10u64.pow(*power);
            let number_of_tens = *n / ten;
            if number_of_tens > 0 {
                self.spell_big_num(number_of_tens, power);
                *n %= ten;
            }
        }
    }

    pub fn spell(mut self, mut n: u64) -> Vec<&'static str> {
        self.slice_big_nums(&mut n);

        match n {
            1...19 => self.spell_word(n),
            20...99 => {
                let reminder = n % 10;
                self.spell_tens(n);

                if reminder > 0 {
                    self.add_hypen();
                    self.spell_word(reminder);
                }
            }
            _ => {}
        }

        // drop trailing spece
        if self.result.last().is_some() &&
           self.result.last().unwrap() == &SPACE {
            self.result.pop();
        }

        self.result
    }

    fn spell_word(&mut self, n: u64) {
        self.result.push(WORDS[n as usize]);
    }

    fn spell_tens(&mut self, n: u64) {
        self.result.push(TENS[(n / 10) as usize]);
    }

    fn spell_tens_power(&mut self, power: u32) {
        self.result.push(TEN_POWERS[power as usize]);
    }

    fn add_hypen(&mut self) {
        self.result.push(HYPEN);
    }

    fn add_space(&mut self) {
        self.result.push(SPACE);
    }
}
