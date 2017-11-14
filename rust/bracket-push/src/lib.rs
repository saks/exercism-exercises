#[derive(Debug)]
pub struct Brackets {
    input: &'static str,
    stack: Vec<char>,
}

fn pair(ch: char) -> Option<char> {
    match ch {
        ')' => Some('('),
        ']' => Some('['),
        '}' => Some('{'),
        _ => None,
    }
}

impl Brackets {
    pub fn new(input: &'static str) -> Self {
        Self {
            input,
            stack: Vec::with_capacity(input.len()),
        }
    }

    fn last_is(&mut self, ch: char) -> bool {
        if let Some(last) = self.stack.pop() {
            return last == ch;
        }

        false
    }

    pub fn are_balanced(&mut self) -> bool {
        // TODO: try fold
        for ch in self.input.chars() {
            match ch {
                '(' | '[' | '{' => {
                    self.stack.push(ch);
                }
                _ => {
                    if let Some(opening) = pair(ch) {
                        if !self.last_is(opening) {
                            return false;
                        }
                    }
                }
            }
        }

        self.stack.is_empty()
    }
}

impl From<&'static str> for Brackets {
    fn from(s: &'static str) -> Self {
        Brackets::new(s)
    }
}
