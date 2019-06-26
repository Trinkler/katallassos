use super::*;

/// Schedule: a function mapping two different times *s* and *t*, with *s<t*, and a cycle *c* onto a
/// sequence of times. See section 4.1 of the ACTUS paper for details.
pub fn schedule(
    s: Time,
    t: Time,
    cycle: Option<Cycle>,
    end_of_month_convention: Option<EndOfMonthConvention>,
) -> Vec<Time> {
    let mut vec: Vec<Time> = Vec::new();

    // Checking some assumptions about the inputs.
    if s != Time(None) && t != Time(None) && s >= t {
        return vec;
    }
    if s == Time(None) && t == Time(None) {
        return vec;
    }

    // Checking two specific cases of the schedule function.
    if t == Time(None) {
        vec.push(s);
        return vec;
    }
    if cycle == None {
        vec.push(s);
        vec.push(t);
        return vec;
    }

    // Checking the main case of the schedule function.
    let mut unchecked_s = s.0.unwrap();
    let unchecked_t = t.0.unwrap();
    let cycle = cycle.unwrap();
    let end_of_month_convention = end_of_month_convention.unwrap_or(EndOfMonthConvention::SD);

    match cycle {
        Cycle::Days(int, stub) => {
            if int == 0 {
                return vec;
            }
            vec.push(s);
            let mut x = s;
            while x != Time(None) && x < t {
                x = Time::add_days(x, int);
                vec.push(x);
            }
            if x > t && stub {
                vec.pop();
            }
        }
        Cycle::Months(int, stub) => {
            if int == 0 {
                return vec;
            }
            vec.push(s);
            while unchecked_s < unchecked_t {
                unchecked_s.year += (unchecked_s.month as u16 + int) / 12;
                unchecked_s.month = ((unchecked_s.month as u16 - 1 + int) % 12 + 1) as i8;
                vec.push(Time::from_unchecked(end_of_month_shift(
                    unchecked_s,
                    end_of_month_convention,
                )));
            }
            if unchecked_s > unchecked_t && stub {
                vec.pop();
            }
        }
        Cycle::Years(int, stub) => {
            if int == 0 {
                return vec;
            }
            vec.push(s);
            while unchecked_s < unchecked_t {
                unchecked_s.year += int;
                vec.push(Time::from_unchecked(end_of_month_shift(
                    unchecked_s,
                    end_of_month_convention,
                )));
            }
            if unchecked_s > unchecked_t && stub {
                vec.pop();
            }
        }
    }

    vec
}

/// Array Schedule: a generalization of a regular schedule. It allows the creation of a schedule
/// that consists of several diferent schedules with different cycles pieced together. See section
/// 4.2 of the ACTUS paper for details.
pub fn array_schedule(
    arr_s: Vec<Time>,
    t: Time,
    arr_cycle: Vec<Option<Cycle>>,
    end_of_month_convention: Option<EndOfMonthConvention>,
) -> Vec<Time> {
    let mut vec: Vec<Time> = Vec::new();

    if arr_s.len() != arr_cycle.len() {
        return vec;
    }

    // Waiting for this feature to be added in Rust. Purpose of this block is to check if the
    // array is sorted. (https://github.com/rust-lang/rust/issues/53485)
    // if !arr_s.is_sorted() {
    //     return vec;
    // }

    let m = arr_s.len();
    let mut vec_2: Vec<Time> = Vec::new();

    for i in 0..(m - 1) {
        vec_2 = schedule(
            arr_s[i],
            arr_s[i + 1],
            arr_cycle[i],
            end_of_month_convention,
        );
        vec_2.pop();
        vec.append(&mut vec_2);
    }

    vec_2 = schedule(arr_s[m - 1], t, arr_cycle[m - 1], end_of_month_convention);
    vec.append(&mut vec_2);

    vec
}

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

/// Business Day Shift Convention: it defines if the schedule times are supposed to fall on only
/// business days or not. It shifts an input time according to the desired rule. See section 4.4 of
/// the ACTUS paper for details.
pub fn business_day_shift(
    mut date: UncheckedTime,
    business_day_convention: BusinessDayConvention,
) -> UncheckedTime {
    match business_day_convention {
        BusinessDayConvention::NULL => (),
        // BusinessDayConvention::SCF => {}
        // BusinessDayConvention::SCMF => {}
        // BusinessDayConvention::CSF => {}
        // BusinessDayConvention::CSMF => {}
        // BusinessDayConvention::SCP => {}
        // BusinessDayConvention::SCMP => {}
        // BusinessDayConvention::CSP => {}
        // BusinessDayConvention::CSMP => {}
    }
    date
}

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

/// Year Fraction Convention: given two input time *s* and *t*, with *s<t*, and the desired day count
/// convention it calculates the fraction of a year between the two times and returns it as a Real.
/// See section 4.6 of the ACTUS paper for details.
pub fn year_fraction(s: Time, t: Time, day_cont_convention: DayCountConvention) -> Real {
    if s == Time(None) || t == Time(None) || s > t {
        return Real(None);
    }
    match day_cont_convention {
        DayCountConvention::_AAISDA => {
            let mut year_1 = s.0.unwrap().year;
            let mut month_1 = s.0.unwrap().month;
            let day_1 = s.0.unwrap().day;
            let year_2 = t.0.unwrap().year;
            let mut month_2 = t.0.unwrap().month;
            let day_2 = t.0.unwrap().day;

            let mut diff_leap: i64 = 0;
            let mut diff_normal: i64 = 0;

            let flag_1 = Time::is_leap_year(year_1);
            let flag_2 = Time::is_leap_year(year_2);

            if flag_1 {
                diff_leap -= day_1 as i64;
            } else {
                diff_normal -= day_1 as i64;
            }
            if flag_2 {
                diff_leap += day_2 as i64;
            } else {
                diff_normal += day_2 as i64;
            }

            let mut x: i64 = 0;
            while month_1 != 0 {
                x += Time::days_in_month(year_1, month_1) as i64;
                month_1 -= 1;
            }
            if flag_1 {
                diff_leap -= x;
            } else {
                diff_normal -= x;
            }
            x = 0;
            while month_2 != 0 {
                x += Time::days_in_month(year_2, month_2) as i64;
                month_2 -= 1;
            }
            if flag_2 {
                diff_leap += x;
            } else {
                diff_normal += x;
            }

            while year_1 < year_2 {
                if Time::is_leap_year(year_1) {
                    diff_leap += 366;
                } else {
                    diff_normal += 365;
                }
                year_1 += 1;
            }

            Real::from(diff_normal) / Real::from(365) + Real::from(diff_leap) / Real::from(366)
        }
        DayCountConvention::_A360 => {
            let mut year_1 = s.0.unwrap().year;
            let mut month_1 = s.0.unwrap().month;
            let day_1 = s.0.unwrap().day;
            let year_2 = t.0.unwrap().year;
            let mut month_2 = t.0.unwrap().month;
            let day_2 = t.0.unwrap().day;

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

            Real::from(diff) / Real::from(360)
        }
        DayCountConvention::_A365 => {
            let mut year_1 = s.0.unwrap().year;
            let mut month_1 = s.0.unwrap().month;
            let day_1 = s.0.unwrap().day;
            let year_2 = t.0.unwrap().year;
            let mut month_2 = t.0.unwrap().month;
            let day_2 = t.0.unwrap().day;

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

            Real::from(diff) / Real::from(365)
        }
        DayCountConvention::_30E360 => {
            let year_1 = Real::from(s.0.unwrap().year as i64);
            let month_1 = Real::from(s.0.unwrap().month as i64);
            let mut day_1 = Real::from(s.0.unwrap().day as i64);
            let year_2 = Real::from(t.0.unwrap().year as i64);
            let month_2 = Real::from(t.0.unwrap().month as i64);
            let mut day_2 = Real::from(t.0.unwrap().day as i64);

            if day_1 == Real::from(31) {
                day_1 = Real::from(30);
            }
            if day_2 == Real::from(31) {
                day_2 = Real::from(30);
            }

            (Real::from(360) * (year_2 - year_1)
                + Real::from(30) * (month_2 - month_1)
                + (day_2 - day_1))
                / Real::from(360)
        }
        DayCountConvention::_30360 => {
            let year_1 = Real::from(s.0.unwrap().year as i64);
            let month_1 = Real::from(s.0.unwrap().month as i64);
            let day_1 = Real::from(s.0.unwrap().day as i64);
            let year_2 = Real::from(t.0.unwrap().year as i64);
            let month_2 = Real::from(t.0.unwrap().month as i64);
            let day_2 = Real::from(t.0.unwrap().day as i64);

            (Real::from(360) * (year_2 - year_1)
                + Real::from(30) * (month_2 - month_1)
                + (day_2 - day_1))
                / Real::from(360)
        }
        // DayCountConvention::_30E360ISDA => (),
        // DayCountConvention::_BUS252 => (),
    }
}

/// Contract Role Sign Convention: it maps a given contract role to either a 1 or a -1, representing
/// a direction for cashflows. See section 4.7 of the ACTUS paper for details.
pub fn contract_role_sign(contract_role: ContractRole) -> Real {
    match contract_role {
        ContractRole::RPA => Real::from(1),
        ContractRole::RPL => Real::from(-1),
        ContractRole::CLO => Real::from(1),
        ContractRole::CNO => Real::from(1),
        ContractRole::COL => Real::from(1),
        ContractRole::LG => Real::from(1),
        ContractRole::ST => Real::from(-1),
        ContractRole::BUY => Real::from(1),
        ContractRole::SEL => Real::from(-1),
        ContractRole::RFL => Real::from(1),
        ContractRole::PFL => Real::from(-1),
        ContractRole::RF => Real::from(1),
        ContractRole::PF => Real::from(-1),
    }
}

/// Contract Default Convention:it maps a given contract status to either a 1 or a 0, representing
/// a performant or a defaulted contract. See section 4.8 of the ACTUS paper for details.
pub fn contract_default(contract_status: ContractStatus) -> Real {
    match contract_status {
        ContractStatus::PF => Real::from(1),
        ContractStatus::DL => Real::from(1),
        ContractStatus::DQ => Real::from(1),
        ContractStatus::DF => Real::from(0),
    }
}

/// Annuity Amount Function: it is used to calculate the annuity amount that needs to be paid at a
/// given time in an annuity contract. See section 4.9 of the ACTUS paper for details.
pub fn annuity_amount(
    arr: Vec<Time>,
    day_cont_convention: DayCountConvention,
    nominal_value: Real,
    nominal_accrued: Real,
    nominal_rate: Real,
) -> Real {
    let mut x1 = Real::from(1);
    let mut x2 = Real::from(0);

    // This is a reverse range, it starts at arr.len()-2 and ends at 0 (both inclusive).
    for i in (0..(arr.len() - 1)).rev() {
        x1 *= Real::from(1) + nominal_rate * year_fraction(arr[i], arr[i + 1], day_cont_convention);
        x2 += x1;
    }

    (nominal_value + nominal_accrued) * x1 / (Real::from(1) + x2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn schedule_works() {
        // Testing s==None && t==None.
        let mut vec: Vec<Time> = Vec::new();
        assert_eq!(schedule(Time(None), Time(None), None, None), vec);

        // Testing t>s.
        let s = Time::from_values(2019, 06, 01, 12, 00, 00);
        let t = Time::from_values(2019, 06, 15, 12, 00, 00);
        assert_eq!(schedule(t, s, None, None), vec);

        // Testing t==None.
        vec.push(s);
        assert_eq!(schedule(s, Time(None), None, None), vec);

        // Testing cycle==None.
        vec.push(t);
        assert_eq!(schedule(s, t, None, None), vec);

        // Testing Cycle::Days==0.
        let mut vec: Vec<Time> = Vec::new();
        let c = Some(Cycle::Days(0, true));
        assert_eq!(schedule(s, t, c, None), vec);

        // Testing Cycle::Days with long stub.
        let c = Some(Cycle::Days(5, true));
        vec.push(s);
        vec.push(Time::from_values(2019, 06, 06, 12, 00, 00));
        vec.push(Time::from_values(2019, 06, 11, 12, 00, 00));
        assert_eq!(schedule(s, t, c, None), vec);

        // Testing Cycle::Days with short stub.
        let c = Some(Cycle::Days(5, false));
        vec.push(Time::from_values(2019, 06, 16, 12, 00, 00));
        assert_eq!(schedule(s, t, c, None), vec);

        // Testing Cycle::Months==0.
        let mut vec: Vec<Time> = Vec::new();
        let t = Time::from_values(2020, 06, 01, 12, 00, 00);
        let c = Some(Cycle::Months(0, true));
        assert_eq!(schedule(s, t, c, None), vec);

        // Testing Cycle::Months with long stub.
        let c = Some(Cycle::Months(5, true));
        vec.push(s);
        vec.push(Time::from_values(2019, 11, 01, 12, 00, 00));
        vec.push(Time::from_values(2020, 04, 01, 12, 00, 00));
        assert_eq!(schedule(s, t, c, None), vec);

        // Testing Cycle::Months with short stub.
        let c = Some(Cycle::Months(5, false));
        vec.push(Time::from_values(2020, 09, 01, 12, 00, 00));
        assert_eq!(schedule(s, t, c, None), vec);

        // Testing Cycle::Years==0.
        let mut vec: Vec<Time> = Vec::new();
        let t = Time::from_values(2030, 06, 01, 12, 00, 00);
        let c = Some(Cycle::Years(0, true));
        assert_eq!(schedule(s, t, c, None), vec);

        // Testing Cycle::Years with long stub.
        let c = Some(Cycle::Years(5, true));
        vec.push(s);
        vec.push(Time::from_values(2024, 06, 01, 12, 00, 00));
        vec.push(Time::from_values(2029, 06, 01, 12, 00, 00));
        assert_eq!(schedule(s, t, c, None), vec);

        // Testing Cycle::Years with short stub.
        let c = Some(Cycle::Years(5, false));
        vec.push(Time::from_values(2034, 06, 01, 12, 00, 00));
        assert_eq!(schedule(s, t, c, None), vec);
    }

    #[test]
    fn array_schedule_works() {
        let t = Time::from_values(2021, 08, 03, 12, 00, 00);
        let mut arr_s: Vec<Time> = Vec::new();
        let mut arr_c: Vec<Option<Cycle>> = Vec::new();

        // Testing different sizes of vectors.
        arr_s.push(t);
        assert_eq!(array_schedule(arr_s, t, arr_c, None), Vec::new());

        // Testing an arbitrary schedule.
        let s1 = Time::from_values(2019, 06, 01, 12, 00, 00);
        let c1 = Some(Cycle::Days(1, true));
        let s2 = Time::from_values(2019, 06, 03, 12, 00, 00);
        let c2 = Some(Cycle::Months(1, true));
        let s3 = Time::from_values(2019, 08, 03, 12, 00, 00);
        let c3 = Some(Cycle::Years(1, true));
        let mut arr_s = vec![s1, s2, s3];
        let mut arr_c = vec![c1, c2, c3];
        let mut vec: Vec<Time> = Vec::new();
        vec.push(s1);
        vec.push(Time::from_values(2019, 06, 02, 12, 00, 00));
        vec.push(s2);
        vec.push(Time::from_values(2019, 07, 03, 12, 00, 00));
        vec.push(s3);
        vec.push(Time::from_values(2020, 08, 03, 12, 00, 00));
        vec.push(t);
        assert_eq!(array_schedule(arr_s, t, arr_c, None), vec);
    }

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

    #[test]
    fn business_day_shift_works() {
        let mut t = time::UncheckedTime {
            year: 2004,
            month: 2,
            day: 31,
            hour: 00,
            minute: 00,
            second: 00,
        };
        assert_eq!(business_day_shift(t, BusinessDayConvention::NULL), t);
    }

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

    #[test]
    fn year_fraction_works() {
        let s = Time::from_values(2012, 2, 10, 00, 00, 00);
        let t = Time::from_values(2017, 4, 9, 16, 20, 00);

        assert_eq!(
            year_fraction(Time(None), t, DayCountConvention::_AAISDA),
            Real(None)
        );
        assert_eq!(
            year_fraction(s, Time(None), DayCountConvention::_AAISDA),
            Real(None)
        );
        assert_eq!(year_fraction(t, s, DayCountConvention::_AAISDA), Real(None));

        // Test values are needed for the day count conventions!
    }

}
