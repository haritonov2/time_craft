use std::fmt;
use std::fmt::Display;

#[derive(PartialEq, PartialOrd, Debug)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    const DAY_IN_MINUTES: i32 = 1440;

    pub fn new(hours: i32, minutes: i32) -> Self {
        let (accurate_hours, accurate_minutes) = Self::parse_data(hours, minutes, None);

        Self {
            hours: accurate_hours,
            minutes: accurate_minutes,
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let (
            accurate_hours,
            accurate_minutes
        ) = Self::parse_data(self.hours, self.minutes, Some(minutes));

        Self {
            hours: accurate_hours,
            minutes: accurate_minutes,
        }
    }

    fn fmt_number(&self, n: i32) -> String {
        if -10 < n && n < 10 {
            format!("0{}", n.to_string())
        } else {
            n.to_string()
        }
    }

    fn parse_data(hours: i32, minutes: i32, extra_minutes: Option<i32>) -> (i32, i32) {
        let mut total_minutes = (hours * 60) + minutes;

        total_minutes += extra_minutes.unwrap_or_else(|| 0);

        let days = total_minutes / Self::DAY_IN_MINUTES;

        let accurate_minutes = total_minutes - (days * Self::DAY_IN_MINUTES);
        let is_back_in_time: bool = match accurate_minutes{
            n if n < 0 => true,
            _ => false,
        };
        let signed_minutes = match is_back_in_time {
            true => Self::DAY_IN_MINUTES + accurate_minutes,
            false => accurate_minutes,
        };

        let final_hours = signed_minutes / 60;
        let final_minutes = signed_minutes - (final_hours * 60);

        (final_hours, final_minutes)
    }
}

impl Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.fmt_number(self.hours), self.fmt_number(self.minutes))
    }
}
