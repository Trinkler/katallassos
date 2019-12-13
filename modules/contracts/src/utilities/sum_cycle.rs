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

use super::*;

/// This is NOT an ACTUS utility function. This function calculates the addition of a time and a
/// cycle, it is used when initializing and progressing contracts.
pub fn sum_cycle(
    t: Time,
    c: Option<Cycle>,
    end_of_month_convention: Option<EndOfMonthConvention>,
) -> Time {
    // Checking if any of the inputs is None.
    if t == Time(None) || c == None {
        return Time(None);
    }

    // Unwrapping the variables.
    let mut unchecked_t = t.0.unwrap();
    let c = c.unwrap();
    let end_of_month_convention = end_of_month_convention.unwrap_or(EndOfMonthConvention::SD);

    match c {
        Cycle::Days(int, _) => {
            return t.add_days(int);
        }
        Cycle::Months(int, _) => {
            unchecked_t.year += (unchecked_t.month as u16 + int) / 12;
            unchecked_t.month = ((unchecked_t.month as u16 - 1 + int) % 12 + 1) as u8;
            return Time::from_unchecked(end_of_month_shift(unchecked_t, end_of_month_convention));
        }
        Cycle::Years(int, _) => {
            unchecked_t.year += int;
            return Time::from_unchecked(end_of_month_shift(unchecked_t, end_of_month_convention));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sum_cycle_works() {
        // Testing t==None || c==None.
        let t = Time::from_values(2019, 06, 01, 12, 00, 00);
        let c = Some(Cycle::Days(7, true));
        assert_eq!(sum_cycle(t, None, None), Time(None));
        assert_eq!(sum_cycle(Time(None), c, None), Time(None));
        assert_eq!(sum_cycle(Time(None), None, None), Time(None));
        assert_ne!(sum_cycle(t, c, None), Time(None));

        // Testing Cycle::Days.
        let t = Time::from_values(2019, 06, 06, 12, 00, 00);
        let c = Some(Cycle::Days(15, true));
        let t2 = Time::from_values(2019, 06, 21, 12, 00, 00);
        assert_eq!(sum_cycle(t, c, None), t2);

        // Testing Cycle::Months.
        let t = Time::from_values(2019, 06, 30, 12, 00, 00);
        let c = Some(Cycle::Months(8, true));
        let t2 = Time::from_values(2020, 02, 29, 12, 00, 00);
        assert_eq!(sum_cycle(t, c, None), t2);

        // Testing Cycle::Years.
        let t = Time::from_values(2019, 06, 06, 12, 00, 00);
        let c = Some(Cycle::Years(7, true));
        let t2 = Time::from_values(2026, 06, 06, 12, 00, 00);
        assert_eq!(sum_cycle(t, c, None), t2);
    }
}
