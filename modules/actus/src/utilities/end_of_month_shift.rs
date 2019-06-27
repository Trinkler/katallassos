use super::*;

/// End of Month Shift Convention: it defines for schedules with monthly or yearly cycles if the
/// schedule times are supposed to try to fall on the same day every time or to always fall on the
/// end of the month. It shifts an input time according to the desired rule. See section 4.3 of the
/// ACTUS paper for details.
pub fn end_of_month_shift(
    mut date: UncheckedTime,
    end_of_month_convention: EndOfMonthConvention,
) -> UncheckedTime {
    let days_in_month = Time::days_in_month(date.year, date.month);

    match end_of_month_convention {
        EndOfMonthConvention::EOM => date.day = days_in_month,
        EndOfMonthConvention::SD => {
            if date.day > days_in_month {
                date.day = days_in_month
            }
        }
    }

    date
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn end_of_month_shift_works() {
        let mut t = time::UncheckedTime {
            year: 2004,
            month: 2,
            day: 31,
            hour: 00,
            minute: 00,
            second: 00,
        };
        let mut z = time::UncheckedTime {
            year: 2004,
            month: 2,
            day: 29,
            hour: 00,
            minute: 00,
            second: 00,
        };
        assert_eq!(end_of_month_shift(t, EndOfMonthConvention::EOM), z);
        assert_eq!(end_of_month_shift(t, EndOfMonthConvention::SD), z);
        t.day = 15;
        z.day = 15;
        assert_eq!(end_of_month_shift(t, EndOfMonthConvention::SD), z);
    }
}
