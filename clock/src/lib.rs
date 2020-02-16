use std::fmt::{self, Formatter, Display};
use std::i32;

#[derive(Debug, PartialEq)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

const MINUTES_PER_HOUR: i32 = 60;
const HOURS_IN_DAY: i32 = 24;
const MINUTES_PER_DAY: i32 = HOURS_IN_DAY * MINUTES_PER_HOUR;

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let positive_mod = |a, b| (a % b + b) % b;

        let total_minutes = positive_mod((hours * MINUTES_PER_HOUR) + minutes, MINUTES_PER_DAY);
        let clock_hours = total_minutes / MINUTES_PER_HOUR;
        let clock_minutes = positive_mod(total_minutes, MINUTES_PER_HOUR);

        return Clock { hours: clock_hours, minutes: clock_minutes };
    }

    pub fn add_minutes(self, minutes: i32) -> Self {
        return Clock::new(self.hours, self.minutes + minutes);
    }
}

impl Display for Clock {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}