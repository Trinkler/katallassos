// Copyright 2019 by Trinkler Software AG (Switzerland).
// This file is part of the Katal Chain.
//
// Katal Chain is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version <http://www.gnu.org/licenses/>.
//
// Katal Chain is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

//! # Time
//!
//! ## Overview
//! The Time library implements a new data type to represent time in the ISO8601 format. This standard
//! represents a date and a time using the format Year-Month-Day Hour:Minute:Second. The standard has
//! optional support for time zones, but we choose not to use it so all the times represented using
//! this data type are used to be in the Coordinated Universal Time (UTC). The standard also has
//! support for leap seconds by allowing the seconds field to range from 0 to 60. We however feel
//! that this is not necessary for our uses and may in fact cause issues, so this library does not
//! support leap seconds and the seconds only range from 0 to 59.
//!
//! ## Technical description
//! In more detail, the data type Time is a struct containing a single Option of the type UncheckedTime,
//! which is a struct with the fields 'year', 'month', 'day', 'hour', 'minute' and 'second'. All the
//! fields are i8, except for 'year' which is an u16. Equality and comparison operations are also
//! implemented and work as is expected, the only quirk being that 'None' is considered smaller
//! than any time.
//!
//! ## Methods
//! This library also implements several methods to deal with times. The four most important are
//! 'is_valid', 'from_values', 'from_unchecked' and 'from_unix'.
//!
//! ### is_valid
//! This method checks if an already created instance of Time represents a valid time and returns a
//! corresponding boolean. It even takes into account leap days. This is meant to be used after a
//! Time instance is modified, to check that it's still valid.
//!
//! ### from_values
//! This method creates a new instance of Time given the desired values (year, month, etc) as input.
//! It also checks if the input values are a valid time, and if they are not it returns 'None'. Since
//! this is a safe way of creating Time instances this method should always preferred instead of the
//! default constructor.
//!
//! ### from_unchecked
//! This method creates a new instance of Time given an instance of UncheckedTime as input.
//! If the inputted UncheckedTime is not valid time it will return a 'None'. This is also a safe
//| constructor for Time.
//!
//! ### from_unix
//! This method converts an unix time into a ISO8601 time and then creates a corresponding Time
//! instance. If the input unix time exceeds the range of allowed ISO8601 times, it will return 'None'.
//! When converting between the two formats leap seconds are ignored.

use super::*;

/// This struct represents the ISO8601 time format.
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct UncheckedTime {
    pub year: u16,
    // i8 is used because 'parity_codec' doesn't support u8.
    pub month: i8,
    pub day: i8,
    pub hour: i8,
    pub minute: i8,
    pub second: i8,
}

/// This struct implements the Time data type. It is a tuple containing a single Option of
/// the type UncheckedTime.
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct Time(pub Option<UncheckedTime>);

impl Time {
    /// It checks if a given Time instance represents a valid time and returns 'true' or 'false'
    /// accordingly.
    pub fn is_valid(&self) -> bool {
        let year = self.0.unwrap().year;
        let month = self.0.unwrap().month;
        let day = self.0.unwrap().day;
        let hour = self.0.unwrap().hour;
        let minute = self.0.unwrap().minute;
        let second = self.0.unwrap().second;
        if year > 9999
            || month < 1
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

    /// A safe constructor for the Time type. It takes six different values as input, one for each
    /// UncheckedTime field. If the input values do not represent a valid time it returns 'None'.
    pub fn from_values(year: u16, month: i8, day: i8, hour: i8, minute: i8, second: i8) -> Time {
        if year > 9999
            || month < 1
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

    /// A safe constructor for the Time type. It takes UncheckedTime as input, if it doesn't represent
    /// a valid time the function returns 'None'.
    pub fn from_unchecked(input: UncheckedTime) -> Time {
        if input.year > 9999
            || input.month < 1
            || input.month > 12
            || input.day < 1
            || input.day > Time::days_in_month(input.year, input.month)
            || input.hour < 0
            || input.hour > 23
            || input.minute < 0
            || input.minute > 59
            || input.second < 0
            || input.second > 59
        {
            Time(None)
        } else {
            Time(Some(input))
        }
    }

    /// It converts an unix time (as an u64) to the corresponding ISO8601 time. The conversion ignores
    /// leap seconds. If the input time is not a valid ISO8601 time it returns 'None'.
    pub fn from_unix(mut unix_time: u64) -> Time {
        // Checking for maximum. This time corresponds to 9999-12-31 23:59:59.
        if unix_time > 253402300799 {
            return Time(None);
        }

        // Initializing the variables with the unix epoch.
        let mut year: u16 = 1970;
        let mut month: i8 = 01;
        let mut day: i8 = 01;
        let mut hour: i8 = 00;
        let mut minute: i8 = 00;
        let mut second: i8 = 00;

        // Converting the seconds.
        second += (unix_time % 60) as i8;
        unix_time /= 60;

        // Converting the minutes.
        minute += (unix_time % 60) as i8;
        unix_time /= 60;

        // Converting the hours.
        hour += (unix_time % 24) as i8;
        unix_time /= 24;

        // Converting the years using leap year arithmetic.
        // 400 years always has 146097 days when accounting for leap days.
        let t = unix_time / 146097;
        year += (t * 400) as u16;
        unix_time -= t * 146097;
        // 100 years always has 36524 days when accounting for leap days.
        let t = unix_time / 36524;
        year += (t * 100) as u16;
        unix_time -= t * 36524;
        // 4 years always has 1461 days when accounting for leap days.
        let t = unix_time / 1461;
        year += (t * 4) as u16;
        unix_time -= t * 1461;
        // This line casts a bool into a integer, true=1 false=0.
        let mut leap_days = Time::is_leap_year(year) as u64;
        // This loop will run at most three times since at this point 'unix_time'
        // can't possibly be bigger than or equal to 4 years.
        while unix_time >= 365 + leap_days {
            year += 1;
            unix_time -= 365 + leap_days;
            leap_days = Time::is_leap_year(year) as u64;
        }

        // Converting the months.
        // First month is of course January, which has 31 days.
        let mut number_days = 31;
        while unix_time >= number_days {
            month += 1;
            unix_time -= number_days;
            number_days = Time::days_in_month(year, month) as u64;
        }

        // Converting the days.
        day += unix_time as i8;

        // Create and return the Time instance.
        Time(Some(UncheckedTime {
            year: year,
            month: month,
            day: day,
            hour: hour,
            minute: minute,
            second: second,
        }))
    }

    /// Adds a given number of days (as an u16) to a date (as a Time) and returns the resulting Time.
    pub fn add_days(self, days: u16) -> Time {
        // Checking the None case and getting the UncheckedTime.
        if self == Time(None) {
            return Time(None);
        }
        let mut time = self.0.unwrap();
        let mut days: u32 = days as u32;

        // Changing the date to the first of January. This will make the calculation easier.
        days += (time.day - 1) as u32;
        time.day = 1;
        while time.month > 1 {
            days += Time::days_in_month(time.year, time.month - 1) as u32;
            time.month -= 1;
        }

        // Converting the years.
        let mut leap_days = Time::is_leap_year(time.year) as u32;
        while days >= 365 + leap_days {
            time.year += 1;
            days -= 365 + leap_days;
            leap_days = Time::is_leap_year(time.year) as u32;
        }

        // Converting the months.
        // First month is of course January, which has 31 days.
        let mut number_days = 31;
        while days >= number_days {
            time.month += 1;
            days -= number_days;
            number_days = Time::days_in_month(time.year, time.month) as u32;
        }

        // Converting the days.
        time.day += days as i8;

        // Create and return the Time instance.
        Time::from_unchecked(time)
    }

    /// Calculates the difference, in days (as an option of an i64), between two dates (as
    /// Times). It is calculated on a Julian day difference basis. In this convention the
    /// first day of the period is included and the last day is excluded.
    pub fn diff_days(start: Time, end: Time) -> Option<i64> {
        // Checking the None case and that end>=start. It returns None otherwise.
        if start == Time(None) || end == Time(None) || end < start {
            return None;
        }

        // Getting the fields.
        let mut year_1 = start.0.unwrap().year;
        let mut month_1 = start.0.unwrap().month;
        let day_1 = start.0.unwrap().day;
        let year_2 = end.0.unwrap().year;
        let mut month_2 = end.0.unwrap().month;
        let day_2 = end.0.unwrap().day;

        let mut diff: i64 = 0;

        diff += (day_2 - day_1) as i64;

        while month_1 != 0 {
            diff -= Time::days_in_month(year_1, month_1) as i64;
            month_1 -= 1;
        }
        while month_2 != 0 {
            diff += Time::days_in_month(year_2, month_2) as i64;
            month_2 -= 1;
        }

        while year_1 < year_2 {
            diff += 365 + (Time::is_leap_year(year_1) as i64);
            year_1 += 1;
        }

        Some(diff)
    }

    /// It returns 'true' if the input year is a leap year and 'false' otherwise.
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

    /// For a given year and month, it returns the number of days in that month.
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

    /// It returns the day of the week for a given date. The output is 1=Monday, 2=Tuesday, ... ,
    /// 7=Sunday (which is the ISO week date). The calculation is done using Zeller's congruence
    /// algorithm.
    pub fn day_of_week(year: u16, month: i8, day: i8) -> i8 {
        let q = day as u16;
        let mut m = month as u16;
        let mut y = year;
        if month == 1 || month == 2 {
            m += 12;
            y -= 1;
        }
        let h = (q + 13 * (m + 1) / 5 + y + y / 4 - y / 100 + y / 400) % 7;
        (((h + 5) % 7) + 1) as i8
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_valid_works() {
        let a = Time(Some(UncheckedTime {
            year: 1970,
            month: 01,
            day: 01,
            hour: 00,
            minute: 00,
            second: 00,
        }));
        let b = Time(Some(UncheckedTime {
            year: 1970,
            month: 13,
            day: 01,
            hour: 00,
            minute: 00,
            second: 00,
        }));
        let c = Time(Some(UncheckedTime {
            year: 1970,
            month: 01,
            day: 32,
            hour: 00,
            minute: 00,
            second: 00,
        }));
        let d = Time(Some(UncheckedTime {
            year: 1970,
            month: 01,
            day: 01,
            hour: 25,
            minute: 00,
            second: 00,
        }));
        let e = Time(Some(UncheckedTime {
            year: 1970,
            month: 01,
            day: 01,
            hour: 00,
            minute: 60,
            second: 00,
        }));
        let f = Time(Some(UncheckedTime {
            year: 1970,
            month: 01,
            day: 01,
            hour: 00,
            minute: 00,
            second: 60,
        }));
        assert_eq!(Time::is_valid(&a), true);
        assert_eq!(Time::is_valid(&b), false);
        assert_eq!(Time::is_valid(&c), false);
        assert_eq!(Time::is_valid(&d), false);
        assert_eq!(Time::is_valid(&e), false);
        assert_eq!(Time::is_valid(&f), false);
    }

    #[test]
    fn from_values_works() {
        let a = Time(Some(UncheckedTime {
            year: 1970,
            month: 01,
            day: 01,
            hour: 00,
            minute: 00,
            second: 00,
        }));
        let b = Time(None);
        assert_eq!(Time::from_values(1970, 01, 01, 00, 00, 00), a);
        assert_eq!(Time::from_values(1970, 13, 01, 00, 00, 00), b);
        assert_eq!(Time::from_values(1970, 01, 32, 00, 00, 00), b);
        assert_eq!(Time::from_values(1970, 01, 01, 25, 00, 00), b);
        assert_eq!(Time::from_values(1970, 01, 01, 00, 60, 00), b);
        assert_eq!(Time::from_values(1970, 01, 01, 00, 00, 60), b);
    }

    #[test]
    fn from_unchecked_works() {
        let a = Time(Some(UncheckedTime {
            year: 1970,
            month: 01,
            day: 01,
            hour: 00,
            minute: 00,
            second: 00,
        }));
        let b = Time(None);
        assert_eq!(
            Time::from_unchecked(UncheckedTime {
                year: 1970,
                month: 01,
                day: 01,
                hour: 00,
                minute: 00,
                second: 00,
            }),
            a
        );
        assert_eq!(
            Time::from_unchecked(UncheckedTime {
                year: 1970,
                month: 13,
                day: 01,
                hour: 00,
                minute: 00,
                second: 00,
            }),
            b
        );
        assert_eq!(
            Time::from_unchecked(UncheckedTime {
                year: 1970,
                month: 01,
                day: 32,
                hour: 00,
                minute: 00,
                second: 00,
            }),
            b
        );
        assert_eq!(
            Time::from_unchecked(UncheckedTime {
                year: 1970,
                month: 01,
                day: 01,
                hour: 25,
                minute: 00,
                second: 00,
            }),
            b
        );
        assert_eq!(
            Time::from_unchecked(UncheckedTime {
                year: 1970,
                month: 01,
                day: 01,
                hour: 00,
                minute: 60,
                second: 00,
            }),
            b
        );
        assert_eq!(
            Time::from_unchecked(UncheckedTime {
                year: 1970,
                month: 01,
                day: 01,
                hour: 00,
                minute: 00,
                second: 60,
            }),
            b
        );
    }

    #[test]
    fn from_unix_works() {
        let a = Time(Some(UncheckedTime {
            year: 1970,
            month: 01,
            day: 01,
            hour: 00,
            minute: 00,
            second: 00,
        }));
        let b = Time(Some(UncheckedTime {
            year: 2019,
            month: 06,
            day: 15,
            hour: 02,
            minute: 34,
            second: 56,
        }));
        assert_eq!(Time::from_unix(0), a);
        assert_eq!(Time::from_unix(1560566096), b);
        assert_ne!(Time::from_unix(514862620), b);
    }

    #[test]
    fn add_days_works() {
        let a = Time(Some(UncheckedTime {
            year: 1969,
            month: 07,
            day: 20,
            hour: 00,
            minute: 00,
            second: 00,
        }));
        let b = Time(Some(UncheckedTime {
            year: 2148,
            month: 12,
            day: 23,
            hour: 00,
            minute: 00,
            second: 00,
        }));
        let c = Time(Some(UncheckedTime {
            year: 9969,
            month: 07,
            day: 20,
            hour: 00,
            minute: 00,
            second: 00,
        }));
        let d = Time(None);
        assert_eq!(Time::add_days(a, 0), a);
        assert_eq!(Time::add_days(a, 65535), b);
        assert_eq!(Time::add_days(c, 36500), d);
    }

    #[test]
    fn diff_days_works() {
        let a = Time(Some(UncheckedTime {
            year: 1969,
            month: 07,
            day: 20,
            hour: 00,
            minute: 00,
            second: 00,
        }));
        let b = Time(Some(UncheckedTime {
            year: 2148,
            month: 12,
            day: 23,
            hour: 00,
            minute: 00,
            second: 00,
        }));
        let c = Time(None);
        assert_eq!(Time::diff_days(a, c), None);
        assert_eq!(Time::diff_days(c, a), None);
        assert_eq!(Time::diff_days(b, a), None);
        assert_eq!(Time::diff_days(a, b), Some(65535));
    }

    #[test]
    fn is_leap_year_works() {
        assert_eq!(Time::is_leap_year(1970), false);
        assert_eq!(Time::is_leap_year(1972), true);
        assert_eq!(Time::is_leap_year(1800), false);
        assert_eq!(Time::is_leap_year(2000), true);
    }

    #[test]
    fn days_in_month_works() {
        assert_eq!(Time::days_in_month(2019, 6), 30);
        assert_eq!(Time::days_in_month(2019, 7), 31);
        assert_eq!(Time::days_in_month(2019, 2), 28);
        assert_eq!(Time::days_in_month(2020, 2), 29);
    }

    #[test]
    fn day_of_week_works() {
        assert_eq!(Time::day_of_week(2100, 3, 1), 1);
        assert_eq!(Time::day_of_week(2000, 2, 29), 2);
        assert_eq!(Time::day_of_week(2019, 6, 15), 6);
        assert_eq!(Time::day_of_week(1969, 7, 20), 7);
    }
}
