//
// Copyright 2019 by Trinkler Software AG (Switzerland).
// This file is part of Katal.
//
// Katal is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version <http://www.gnu.org/licenses/>.
//
// Katal is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//

use super::*;

/// Business Day Shift Convention: it defines if the schedule times are supposed to fall on only
/// business days or not. It shifts an input time according to the desired rule. See section 4.4 of
/// the ACTUS paper for details.
/// Note: So far, it is not implemented, since it may not be needed in Katal.
pub fn business_day_shift(
    mut date: UncheckedTime,
    business_day_convention: BusinessDayConvention,
) -> UncheckedTime {
    match business_day_convention {
        BusinessDayConvention::SCF => (),
        BusinessDayConvention::SCMF => (),
        BusinessDayConvention::CSF => (),
        BusinessDayConvention::CSMF => (),
        BusinessDayConvention::SCP => (),
        BusinessDayConvention::SCMP => (),
        BusinessDayConvention::CSP => (),
        BusinessDayConvention::CSMP => (),
    }
    date
}

#[cfg(test)]
mod tests {
    use super::*;

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
        assert_eq!(business_day_shift(t, BusinessDayConvention::NS), t);
    }
}
