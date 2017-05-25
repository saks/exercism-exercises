pub struct Roman(usize);

impl Roman {
    pub fn to_string(self) -> String {
        let huns = ["", "C", "CC", "CCC", "CD", "D", "DC", "DCC", "DCCC", "CM"];
        let tens = ["", "X", "XX", "XXX", "XL", "L", "LX", "LXX", "LXXX", "XC"];
        let ones = ["", "I", "II", "III", "IV", "V", "VI", "VII", "VIII", "IX"];

        let mut result = String::new();
        let Roman(mut n) = self;

        while n >= 1000 {
            result.push('M');
            n -= 1000;
        }

        result.push_str(huns[n / 100]);
        n = n % 100;

        result.push_str(tens[n / 10]);
        n = n % 10;

        result.push_str(ones[n]);

        result
    }
}

impl From<usize> for Roman {
    fn from(n: usize) -> Roman {
        Roman(n)
    }
}
