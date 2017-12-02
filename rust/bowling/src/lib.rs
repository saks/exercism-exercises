#![feature(vec_remove_item)]

#[derive(PartialEq, Debug)]
enum State {
    Ready,
    SecondTry,
}

#[derive(Debug)]
struct Bonus {
    n: u8,
}

impl Bonus {
    pub fn new(n: u8) -> Self {
        Self { n }
    }

    fn is_usable(&self) -> bool {
        self.n > 0
    }

    fn use_once(&mut self) {
        self.n -= 1;
    }
}

#[derive(Default)]
pub struct BowlingGame {
    frames: u8,
    score: u16,
    pins_left: u8,
    bonus_frames: Vec<Bonus>,
    bonus_rolls: u8,
    state: State,
}

impl Default for State {
    fn default() -> State {
        State::Ready
    }
}

impl BowlingGame {
    pub fn new() -> Self {
        Self::default()
    }

    fn handle_strike_frames(&mut self, n: u8) {
        let mut strikes_left = vec![];

        while let Some(mut strike) = self.bonus_frames.pop() {
            self.score += u16::from(n);
            strike.use_once();
            if strike.is_usable() {
                strikes_left.push(strike);
            }
        }

        self.bonus_frames = strikes_left;
    }

    pub fn roll(&mut self, n: u8) -> Result<(), &'static str> {
        if n > 10 {
            return Err("only 10 pins");
        }

        if self.frames > 9 {
            if self.bonus_rolls > 0 {
                self.bonus_rolls -= 1;
            } else {
                return Err("game is done");
            }
        }

        self.score += u16::from(n);
        self.handle_strike_frames(n);

        match self.state {
            State::Ready => {
                if n == 10 {
                    match self.frames {
                        0...8 => self.bonus_frames.push(Bonus::new(2)),
                        9 => self.bonus_rolls = 2,
                        _ => {}
                    }
                    self.state = State::Ready;
                    self.frames += 1;
                } else {
                    self.state = State::SecondTry;
                    self.pins_left = 10 - n;
                }
            }
            State::SecondTry => {
                if n > self.pins_left {
                    return Err("cheating!");
                }
                self.pins_left -= n;

                if self.pins_left == 0 {
                    // SPARE
                    match self.frames {
                        0...8 => self.bonus_frames.push(Bonus::new(1)),
                        9 => self.bonus_rolls = 1,
                        _ => {}
                    }
                }
                self.state = State::Ready;

                self.pins_left = 0;
                self.frames += 1;
            }
        }

        Ok(())
    }

    pub fn score(&self) -> Result<u16, &'static str> {
        if self.frames < 10 || self.bonus_rolls != 0 {
            return Err("not finished");
        }

        Ok(self.score)
    }
}
