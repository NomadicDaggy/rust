use std::fmt;

#[derive(Debug)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    // New Clock is created through add_minutes(), so the logic can live
    // in only one place.
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
        let mut minute_sum = (self.hours * 60 + self.minutes + minutes) % (24 * 60);

        // minute_sum can still be negative- the modulus operator doesn't take
        // away the sign. This could also probably be accomplished more elegantly
        // within the initial minute_sum assignment. Don't know how though.
        if minute_sum < 0 {
            minute_sum += 24 * 60;
        }

        Clock {
            hours: minute_sum / 60,
            minutes: minute_sum % 60,
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
