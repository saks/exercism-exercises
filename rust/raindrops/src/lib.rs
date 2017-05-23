use std::fmt;
use std::vec::Vec;

#[derive(Debug)]
enum DropTypes {
    Pling,
    Plang,
    Plong,
    Plain(i32),
}

#[derive(Debug)]
struct DropSet(Vec<DropTypes>);

impl fmt::Display for DropSet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut result = String::from("");


        for drop in &self.0 {
            result.push_str(&format!("{}", drop));
        }

        write!(f, "{}", result)
    }
}

#[derive(Debug)]
struct Drop(i32);

impl From<Drop> for DropSet {
    fn from(drop: Drop) -> DropSet {
        let n = drop.0;
        let is_pling = 0 == (n % 3);
        let is_plang = 0 == (n % 5);
        let is_plong = 0 == (n % 7);

        let mut data: Vec<DropTypes> = Vec::new();

        if is_pling {
            data.push(DropTypes::Pling);
        }

        if is_plang {
            data.push(DropTypes::Plang);
        }

        if is_plong {
            data.push(DropTypes::Plong);
        }

        if data.is_empty() {
            data.push(DropTypes::Plain(n));
        }

        DropSet(data)
    }
}

impl fmt::Display for DropTypes {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DropTypes::Pling => write!(f, "Pling"),
            DropTypes::Plang => write!(f, "Plang"),
            DropTypes::Plong => write!(f, "Plong"),
            DropTypes::Plain(n) => write!(f, "{}", n),
        }
    }
}

pub fn raindrops(n: i32) -> String {
    format!("{}", DropSet::from(Drop(n)))
}
