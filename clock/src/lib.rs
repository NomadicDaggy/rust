use std::fmt;

#[derive(Debug, PartialEq)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        // This doesn't have to hold any rollover or negativity logic
        // bescause add_minutes() does all that.
        let minute_sum = hours * 60 + minutes;

        Clock {
            hours: 0,
            minutes: 0,
        }
        .add_minutes(minute_sum)
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Clock {
            hours: (self.hours + (minutes + self.minutes).div_euclid(60)).rem_euclid(24),
            minutes: (self.minutes + minutes.rem_euclid(60)).rem_euclid(60),
        }
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{:0>2}:{:0>2}",
            self.hours,
            self.minutes,
        )
    }
}
