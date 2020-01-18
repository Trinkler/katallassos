// Copyright 2020 by Trinkler Software AG (Switzerland).
// This file is part of Katal Chain.
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

use super::*;

/// Year Fraction Convention: given two input time *s* and *t*, with *s<t*, and the desired day count
/// convention it calculates the fraction of a year between the two times and returns it as a Real.
/// See section 4.6 of the ACTUS paper for details.
pub fn year_fraction(s: Time, t: Time, day_count_convention: DayCountConvention) -> Real {
    if s == Time(None) || t == Time(None) || s > t {
        return Real(None);
    }
    match day_count_convention {
        DayCountConvention::AAISDA => {
            let mut year_1 = s.0.unwrap().year;
            let year_2 = t.0.unwrap().year;

            let mut diff_leap: i64 = 0;
            let mut diff_normal: i64 = 0;

            if year_1 == year_2 {
                if Time::is_leap_year(year_1) {
                    diff_leap += Time::diff_days(s, t).unwrap();
                } else {
                    diff_normal += Time::diff_days(s, t).unwrap();
                }
            } else {
                if Time::is_leap_year(year_1) {
                    diff_leap +=
                        Time::diff_days(s, Time::from_values(year_1 + 1, 01, 01, 00, 00, 00))
                            .unwrap();
                } else {
                    diff_normal +=
                        Time::diff_days(s, Time::from_values(year_1 + 1, 01, 01, 00, 00, 00))
                            .unwrap();
                }

                while year_1 + 1 < year_2 {
                    year_1 += 1;
                    if Time::is_leap_year(year_1) {
                        diff_leap += 366;
                    } else {
                        diff_normal += 365;
                    }
                }

                if Time::is_leap_year(year_2) {
                    diff_leap +=
                        Time::diff_days(Time::from_values(year_2, 01, 01, 00, 00, 00), t).unwrap();
                } else {
                    diff_normal +=
                        Time::diff_days(Time::from_values(year_2, 01, 01, 00, 00, 00), t).unwrap();
                }
            }
            Real::from(diff_normal) / Real::from(365) + Real::from(diff_leap) / Real::from(366)
        }
        DayCountConvention::A360 => {
            let diff = Time::diff_days(s, t).unwrap();
            Real::from(diff) / Real::from(360)
        }
        DayCountConvention::A365 => {
            let diff = Time::diff_days(s, t).unwrap();
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
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn year_fraction_works() {
        let r = Time::from_values(2019, 9, 5, 12, 00, 00);
        let s = Time::from_values(2019, 12, 31, 12, 00, 00);
        let t = Time::from_values(2020, 6, 1, 12, 00, 00);

        // Testing error cases.
        assert_eq!(
            year_fraction(Time(None), t, DayCountConvention::AAISDA),
            Real(None)
        );
        assert_eq!(
            year_fraction(r, Time(None), DayCountConvention::AAISDA),
            Real(None)
        );
        assert_eq!(year_fraction(t, r, DayCountConvention::AAISDA), Real(None));

        // Testing some normal cases.
        assert_eq!(
            year_fraction(r, s, DayCountConvention::AAISDA),
            Real::from(118) / Real::from(365)
        );
        assert_eq!(
            year_fraction(r, t, DayCountConvention::AAISDA),
            Real::from(119) / Real::from(365) + Real::from(151) / Real::from(366)
        );
        assert_eq!(
            year_fraction(r, t, DayCountConvention::A360),
            Real::from(270) / Real::from(360)
        );
        assert_eq!(
            year_fraction(r, t, DayCountConvention::A365),
            Real::from(270) / Real::from(365)
        );
        assert_eq!(
            year_fraction(s, t, DayCountConvention::_30E360),
            (Real::from(360) * Real::from(1) + Real::from(30) * Real::from(-6) + Real::from(-29))
                / Real::from(360)
        );
    }
}
