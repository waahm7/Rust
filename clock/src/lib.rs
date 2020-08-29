use std::fmt;

#[derive(Debug, Eq, PartialEq)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        if minutes % 60 < 0 {
            Clock {
                hours: (hours + minutes / 60 - 1).rem_euclid(24),
                minutes: minutes.rem_euclid(60),
            }
        } else {
            Clock {
                hours: (hours + minutes / 60).rem_euclid(24),
                minutes: minutes.rem_euclid(60),
            }
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Clock::new(self.hours,self.minutes+minutes)
    }
}
impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:0>2}:{:0>2}", self.hours, self.minutes)
    }
}
