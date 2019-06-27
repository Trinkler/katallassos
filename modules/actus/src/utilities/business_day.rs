use super::*;

/// Business Day Calendar: for a given calendar, it determines if the inputted time falls on a
/// business day or not. See section 4.5 of the ACTUS paper for details.
pub fn business_day(date: UncheckedTime, calendar: Calendar) -> bool {
    match calendar {
        Calendar::NC => true,
        // Calendar::MTF => {
        //     let weekday = Time::day_of_week(date.year, date.month, date.day);
        //     if weekday == 6 || weekday == 7 {
        //         false
        //     } else {
        //         true
        //     }
        // }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn business_day_works() {
        let mut t = time::UncheckedTime {
            year: 2004,
            month: 2,
            day: 31,
            hour: 00,
            minute: 00,
            second: 00,
        };
        assert_eq!(business_day(t, Calendar::NC), true);
    }
}
