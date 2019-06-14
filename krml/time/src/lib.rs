//! # Time module
//!
//! ## Overview
//!

// These are necessary to work with Substrate.
use parity_codec::{Decode, Encode};

/// The struct that represents the ISO8601 time format.
#[derive(Copy, Clone, Decode, Encode, Default, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct UncheckedTime {
    pub year: u16,
    // parity codec doesn't support u8.
    pub month: i8,
    pub day: i8,
    pub hour: i8,
    pub minute: i8,
    pub second: i8,
}

/// The struct that implements the time data type. It is a tuple containing a single Option of
/// the type UncheckedTime.
#[derive(Copy, Clone, Decode, Encode, Default, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct Time(pub Option<UncheckedTime>);

impl Time {
    pub fn is_leap_year(year: u16) -> bool {
        if year % 4 != 0 {
            return false;
        }
        if year % 100 != 0 {
            return true;
        }
        if year % 400 != 0 {
            return false;
        }
        return true;
    }

    pub fn days_in_month(year: u16, month: i8) -> i8 {
        if month == 1
            || month == 3
            || month == 5
            || month == 7
            || month == 8
            || month == 10
            || month == 12
        {
            return 31;
        } else if month == 4 || month == 6 || month == 9 || month == 11 {
            return 30;
        } else if Time::is_leap_year(year) {
            return 29;
        } else {
            return 28;
        }
    }

    pub fn new(year: u16, month: i8, day: i8, hour: i8, minute: i8, second: i8) -> Time {
        if month < 1
            || month > 12
            || day < 1
            || day > Time::days_in_month(year, month)
            || hour < 0
            || hour > 23
            || minute < 0
            || minute > 59
            || second < 0
            || second > 59
        {
            Time(None)
        } else {
            Time(Some(UncheckedTime {
                year: year,
                month: month,
                day: day,
                hour: hour,
                minute: minute,
                second: second,
            }))
        }
    }

    pub fn is_valid(&self) -> bool {
        let year = self.0.unwrap().year;
        let month = self.0.unwrap().month;
        let day = self.0.unwrap().day;
        let hour = self.0.unwrap().hour;
        let minute = self.0.unwrap().minute;
        let second = self.0.unwrap().second;
        if month < 1
            || month > 12
            || day < 1
            || day > Time::days_in_month(year, month)
            || hour < 0
            || hour > 23
            || minute < 0
            || minute > 59
            || second < 0
            || second > 59
        {
            false
        } else {
            true
        }
    }

    // Time utility, convert from unix time to ISO8601, ignores leap seconds
    pub fn from_unix(mut unix_time: u64) -> Time {
        // Unix epoch
        let mut year: u16 = 1970;
        let mut month: i8 = 01;
        let mut day: i8 = 01;
        let mut hour: i8 = 00;
        let mut minute: i8 = 00;
        let mut second: i8 = 00;
        // seconds
        second += (unix_time % 60) as i8;
        unix_time /= 60;
        // minutes
        minute += (unix_time % 60) as i8;
        unix_time /= 60;
        // hours
        hour += (unix_time % 24) as i8;
        unix_time /= 24;
        // 400 years has 146097 days.
        let t = unix_time / 146097;
        year += t * 400 as u16;
        unix_time -= t * 146097;
        // 100 years has 36524 days.
        let t = unix_time / 36524;
        year += t * 100 as u16;
        unix_time -= t * 36524;
        // 4 years has 1461 days.
        let t = unix_time / 1461;
        year += t * 4 as u16;
        unix_time -= t * 1461;
        // casting a bool into a integer, true =1 false =0
        let mut leap_days = Time::is_leap_year(year) as u64;
        // At most this will be 3 years.
        while unix_time >= 365 + leap_days {
            year += 1;
            unix_time -= 365 + leap_days;
            leap_days = Time::is_leap_year(year) as u64
        }
        // January has 31 days
        let mut number_days = 31;
        while unix_time >= number_days {
            month += 1;
            unix_time -= number_days;
            number_days = Time::days_in_month(year, month) as u64;
        }
        // days
        day += unix_time as i8;
        // return
        Time(Some(UncheckedTime {
            year: year,
            month: month,
            day: day,
            hour: hour,
            minute: minute,
            second: second,
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

}
