use std::fmt;
use std::fmt::Display;

pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        Self {
            hours,
            minutes,
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        println!("{}", minutes);

        Self {
            hours: self.hours,
            minutes: self.minutes + minutes,
        }
    }

    fn fmt_number(&self, n: i32) -> String {
        if -10 < n && n < 10 {
            format!("0{}", n.to_string())
        } else {
            n.to_string()
        }
    }
}

impl Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.fmt_number(self.hours), self.fmt_number(self.minutes))
    }
}
