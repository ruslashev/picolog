use std::fmt;
use std::time::{SystemTime, UNIX_EPOCH};

pub struct Timestamp {
    year: u16,
    month: u16,
    day: u16,
    hours: u8,
    minutes: u8,
    seconds: u8,
}

impl Timestamp {
    pub fn new() -> Self {
        let seconds = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("current time before Unix epoch")
            .as_secs();

        Self::from_unix(seconds)
    }

    fn from_unix(total_seconds: u64) -> Self {
        let total_minutes = total_seconds / 60;
        let total_hours = total_minutes / 60;

        let seconds = (total_seconds % 60) as u8;
        let minutes = (total_minutes % 60) as u8;
        let hours = (total_hours % 24) as u8;

        let mut month_lengths = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
        let mut day = total_hours / 24;
        let mut year = 1970;
        let mut month = 0;

        loop {
            if is_leap_year(year) {
                if day < 366 {
                    break;
                }
                day -= 366;
            } else {
                if day < 365 {
                    break;
                }
                day -= 365;
            }
            year += 1;
        }

        day += 1;

        if is_leap_year(year) {
            month_lengths[1] = 29;
        }

        while day > month_lengths[month as usize] {
            day -= month_lengths[month as usize];
            month += 1;
        }

        month += 1;

        #[allow(clippy::cast_possible_truncation)]
        let day = day as u16;

        Self {
            year,
            month,
            day,
            hours,
            minutes,
            seconds,
        }
    }
}

impl fmt::Display for Timestamp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}-{:02}-{:02} {:02}:{:02}:{:02}",
            self.year, self.month, self.day, self.hours, self.minutes, self.seconds
        )
    }
}

fn is_leap_year(year: u16) -> bool {
    year % 400 == 0 || (year % 4 == 0 && year % 100 != 0)
}

#[cfg(test)]
mod tests {
    use super::Timestamp;

    #[test]
    fn test() {
        let upper_bound = 2000000000; // May 18 2033 03:33:20 UTC+0

        for u in 0..upper_bound {
            let a = Timestamp::from_unix(u);
            let b = time::OffsetDateTime::from_unix_timestamp(u as i64).unwrap();

            assert_eq!(a.year, b.year() as u16);
            assert_eq!(a.month, b.month() as u16);
            assert_eq!(a.day, b.day() as u16);
            assert_eq!(a.hours, b.hour());
            assert_eq!(a.minutes, b.minute());
            assert_eq!(a.seconds, b.second());
        }
    }
}
