use std::fmt;

#[derive(Debug)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

const MINUTES_PER_DAY: i32 = 24 * 60;

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        Clock::new_from_minutes(hours * 60 + minutes)
    }

    pub fn add_minutes(self, minutes: i32) -> Self {
        Clock::new_from_minutes(self.minutes() + minutes)
    }

    fn minutes(&self) -> i32 {
        self.hours * 60 + self.minutes
    }

    fn new_from_minutes(mut minutes: i32) -> Self {
        minutes = minutes % MINUTES_PER_DAY;

        if minutes < 0 {
            minutes += MINUTES_PER_DAY;
        }

        return Clock {
            hours: minutes / 60,
            minutes: minutes % 60,
        };
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}

impl PartialEq for Clock {
    fn eq(&self, other: &Clock) -> bool {
        self.minutes() == other.minutes()
    }
}
