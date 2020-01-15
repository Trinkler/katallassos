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

/// Annuity Amount Function: it is used to calculate the annuity amount that needs to be paid at a
/// given time in an annuity contract. See section 4.9 of the ACTUS paper for details.
pub fn annuity_amount(
    arr: Vec<Time>,
    day_count_convention: DayCountConvention,
    nominal_value: Real,
    nominal_accrued: Real,
    nominal_rate: Real,
) -> Real {
    let mut x1 = Real::from(1);
    let mut x2 = Real::from(0);

    // This is a reverse range, it starts at arr.len()-2 and ends at 0 (both inclusive).
    for i in (0..(arr.len() - 1)).rev() {
        x1 *=
            Real::from(1) + nominal_rate * year_fraction(arr[i], arr[i + 1], day_count_convention);
        x2 += x1;
    }

    (nominal_value + nominal_accrued) * x1 / (Real::from(1) + x2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn annuity_amount_works() {
        let p = Time::from_values(1969, 07, 16, 13, 32, 00);
        let q = Time::from_values(1969, 07, 20, 20, 17, 00);
        let r = Time::from_values(1969, 07, 21, 02, 56, 15);
        let s = Time::from_values(1969, 07, 24, 16, 51, 00);
        let nominal_value = Real::from(1000);
        let nominal_accrued = Real::from(42);
        let nominal_rate = Real::from(2);

        let numerator = (Real::from(1)
            + nominal_rate * year_fraction(p, q, DayCountConvention::A365))
            * (Real::from(1) + nominal_rate * year_fraction(q, r, DayCountConvention::A365))
            * (Real::from(1) + nominal_rate * year_fraction(r, s, DayCountConvention::A365));

        let denominator = Real::from(1)
            + (Real::from(1) + nominal_rate * year_fraction(p, q, DayCountConvention::A365))
                * (Real::from(1) + nominal_rate * year_fraction(q, r, DayCountConvention::A365))
                * (Real::from(1) + nominal_rate * year_fraction(r, s, DayCountConvention::A365))
            + (Real::from(1) + nominal_rate * year_fraction(q, r, DayCountConvention::A365))
                * (Real::from(1) + nominal_rate * year_fraction(r, s, DayCountConvention::A365))
            + (Real::from(1) + nominal_rate * year_fraction(r, s, DayCountConvention::A365));

        let result = (nominal_value + nominal_accrued) * numerator / denominator;

        assert_eq!(
            result,
            annuity_amount(
                vec![p, q, r, s],
                DayCountConvention::A365,
                nominal_value,
                nominal_accrued,
                nominal_rate
            )
        );
    }
}
