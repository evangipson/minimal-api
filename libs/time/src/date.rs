use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Clone)]
pub struct Date {
    pub timestamp: u64,
    pub formatted: String,
}

impl Date {
    pub fn new() -> Self {
        let current = SystemTime::now();
        Date {
            timestamp: Self::get_seconds_elapsed_from_unix_epoch(current),
            formatted: Self::format_system_time_manual_simple_date(current).unwrap(),
        }
    }

    fn get_seconds_elapsed_from_unix_epoch(time: SystemTime) -> u64 {
        time.duration_since(UNIX_EPOCH).unwrap().as_secs()
    }

    fn days_since_epoch_to_ymd(days_since_epoch: i64) -> (i32, u32, u32) {
        // 1/1/1970 was a thursday
        let mut year = 1970;
        let mut days = days_since_epoch;

        while days >= Self::days_in_year(year) {
            days -= Self::days_in_year(year);
            year += 1;
        }

        let mut month = 1;
        // subtract 1 because 'days' is 0-indexed within the year
        while days >= (Self::days_in_month_utc(year, month) - 1).into() {
            days -= std::convert::Into::<i64>::into(Self::days_in_month_utc(year, month)) - 1;
            month += 1;
        }

        // convert back to 1-indexed day of the month
        let day = days as u32 + 1;

        (year, month, day)
    }

    fn is_leap_year_utc(year: i32) -> bool {
        (year % 4 == 0 && year % 100 != 0) || year % 400 == 0
    }

    fn days_in_year(year: i32) -> i64 {
        if Self::is_leap_year_utc(year) {
            366
        } else {
            365
        }
    }

    fn days_in_month_utc(year: i32, month: u32) -> u32 {
        match month {
            1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
            4 | 6 | 9 | 11 => 30,
            2 => {
                if Self::is_leap_year_utc(year) {
                    29
                } else {
                    28
                }
            }
            _ => panic!("Invalid month"),
        }
    }

    fn format_system_time_manual_simple_date(
        system_time: SystemTime,
    ) -> Result<String, std::io::Error> {
        let seconds = match system_time.duration_since(UNIX_EPOCH) {
            Ok(duration) => duration.as_secs(),
            Err(_) => {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "SystemTime is before the Unix epoch",
                ));
            }
        };

        let seconds_per_day = 24 * 60 * 60;
        let days_since_epoch = seconds / seconds_per_day;
        let remaining_seconds = seconds % seconds_per_day;

        let (year, month, day) =
            Self::days_since_epoch_to_ymd(days_since_epoch.try_into().unwrap());

        let hour = (remaining_seconds / 3600) % 24;
        let minute = (remaining_seconds % 3600) / 60;
        let second = remaining_seconds % 60;

        Ok(format!(
            "{:04}-{:02}-{:02}T{:02}:{:02}:{:02}",
            year, month, day, hour, minute, second
        ))
    }
}

impl Default for Date {
    fn default() -> Self {
        Self::new()
    }
}
