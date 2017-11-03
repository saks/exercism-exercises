#[derive(Debug, PartialEq)]
pub struct Clock {
    hours: i16,
    minutes: i16,
}

impl Clock {
    pub fn new(hours: i16, minutes: i16) -> Self {
        Clock { hours, minutes }.normalize()
    }

    pub fn add_minutes(mut self, n: i16) -> Self {
        self.minutes += n;
        self.normalize()
    }

    pub fn to_string(&self) -> String {
        format!("{:02}:{:02}", self.hours, self.minutes)
    }

    fn normalize(mut self) -> Self {
        self.hours += self.minutes / 60;
        self.minutes %= 60;
        self.hours %= 24;

        if self.minutes < 0 {
            self.hours -= 1;
            self.minutes += 60;
        }

        if self.hours < 0 {
            self.hours += 24;
        }

        self
    }
}
