use super::*;

/// This is NOT an ACTUS utility function. This function calculates the addition of a time and a
/// period, it is used when initializing and progressing contracts.
pub fn sum_period(
    t: Time,
    p: Option<Period>,
    end_of_month_convention: Option<EndOfMonthConvention>,
) -> Time {
    // Checking if any of the inputs is None.
    if t == Time(None) || p == None {
        return Time(None);
    }

    // Unwrapping the variables.
    let mut unchecked_t = t.0.unwrap();
    let p = p.unwrap();
    let end_of_month_convention = end_of_month_convention.unwrap_or(EndOfMonthConvention::SD);

    match p {
        Period::Days(int) => {
            return t.add_days(int);
        }
        Period::Months(int) => {
            unchecked_t.year += (unchecked_t.month as u16 + int) / 12;
            unchecked_t.month = ((unchecked_t.month as u16 - 1 + int) % 12 + 1) as i8;
            return Time::from_unchecked(end_of_month_shift(unchecked_t, end_of_month_convention));
        }
        Period::Years(int) => {
            unchecked_t.year += int;
            return Time::from_unchecked(end_of_month_shift(unchecked_t, end_of_month_convention));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sum_period_works() {
        // Testing t==None || p==None.
        let t = Time::from_values(2019, 06, 01, 12, 00, 00);
        let p = Some(Period::Days(7));
        assert_eq!(sum_period(t, None, None), Time(None));
        assert_eq!(sum_period(Time(None), p, None), Time(None));
        assert_eq!(sum_period(Time(None), None, None), Time(None));
        assert_ne!(sum_period(t, p, None), Time(None));

        // Testing Period::Days.
        let t = Time::from_values(2019, 06, 06, 12, 00, 00);
        let p = Some(Period::Days(15));
        let t2 = Time::from_values(2019, 06, 21, 12, 00, 00);
        assert_eq!(sum_period(t, p, None), t2);

        // Testing Period::Months.
        let t = Time::from_values(2019, 06, 30, 12, 00, 00);
        let p = Some(Period::Months(8));
        let t2 = Time::from_values(2020, 02, 29, 12, 00, 00);
        assert_eq!(sum_period(t, p, None), t2);

        // Testing Period::Years.
        let t = Time::from_values(2019, 06, 06, 12, 00, 00);
        let p = Some(Period::Years(7));
        let t2 = Time::from_values(2026, 06, 06, 12, 00, 00);
        assert_eq!(sum_period(t, p, None), t2);
    }
}
