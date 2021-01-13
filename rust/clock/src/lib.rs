use std::fmt;
use std::fmt::Formatter;

#[derive(Debug, PartialEq)]
pub struct Clock {
    minutes: i32,
}

const HOURS: i32 = 24;
const MINUTES: i32 = 60;
const MAX_MINUTES: i32 = HOURS * MINUTES;

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        Clock { minutes: 0 }.add_minutes(hours * MINUTES + minutes)
    }

    pub fn add_minutes(mut self, minutes: i32) -> Self {
        let minutes = minutes % MAX_MINUTES;
        self.minutes = (self.minutes + minutes + MAX_MINUTES) % MAX_MINUTES;
        self
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let hours = self.minutes / MINUTES;
        let minutes = self.minutes % MINUTES;
        write!(f, "{:02}:{:02}", hours, minutes)
    }
}
