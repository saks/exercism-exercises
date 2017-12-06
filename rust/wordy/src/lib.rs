pub struct WordProblem {
    cmd: Vec<String>,
}

#[derive(PartialEq)]
enum Operations {
    Minus,
    Plus,
    Mult,
    Division,
    Nothing,
}

fn apply(x: i32, y: i32, op: &Operations) -> Result<i32, &'static str> {
    match *op {
        Operations::Minus => Ok(x - y),
        Operations::Plus => Ok(x + y),
        Operations::Mult => Ok(x * y),
        Operations::Division => Ok(x / y),
        Operations::Nothing => Err(""),
    }
}

impl WordProblem {
    pub fn new(cmd: &'static str) -> Self {
        let cmd = String::from(cmd)
            .replace("What is ", "")
            .replace("?", "")
            .replace("multiplied by", "multiplied")
            .replace("divided by", "divided")
            .split(' ')
            .map(String::from)
            .collect();

        Self { cmd: cmd }
    }

    pub fn answer(&self) -> Result<i32, &'static str> {
        let mut result: i32;
        let mut operation = Operations::Nothing;

        match self.cmd[0].parse::<i32>() {
            Ok(n) => result = n,
            Err(_) => return Err(""),
        }

        for op in &self.cmd[1..] {
            if Operations::Nothing == operation {
                match op.as_str() {
                    "minus" => operation = Operations::Minus,
                    "plus" => operation = Operations::Plus,
                    "multiplied" => operation = Operations::Mult,
                    "divided" => operation = Operations::Division,
                    _ => return Err(""),
                }
            } else {
                match op.parse::<i32>() {
                    Ok(n) => {
                        result = apply(result, n, &operation)?;
                        operation = Operations::Nothing;
                    }
                    Err(_) => return Err(""),
                }
            }
        }

        Ok(result)
    }
}
