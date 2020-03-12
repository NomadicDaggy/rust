use std::fmt;

// Debug has to be derived because ...
// PartialEq has to be derived because ...
#[derive(Debug)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        Clock {
            hours,
            minutes,
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let new_minutes = self.minutes + minutes;
        Clock {
            hours: self.hours,
            minutes: new_minutes,
        }
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}:{}",
            format!("{:0>2}", self.hours % 24),
            format!("{:0>2}", self.minutes % 60)
        )
    }
}

impl PartialEq for Clock {
    fn eq(&self, other: &Self) -> bool {
        let hours_match = self.hours % 24 == other.hours % 24;
        let minutes_match = self.minutes % 60 == other.minutes % 60;
        hours_match && minutes_match
    }
}
