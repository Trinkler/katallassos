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

/// All ACTUS contract variables as specifed in the ACTUS paper.
#[derive(Clone, Copy, Decode, Encode, Default, PartialEq)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct Variables {
    pub fee_accrued: Real,
    pub interest_calculation_base: Real,
    pub interest_scaling_multiplier: Real,
    pub last_event_date: Time,
    pub next_principal_redemption_payment: Real,
    pub nominal_accrued_1: Real,
    pub nominal_accrued_2: Real,
    pub nominal_rate: Real,
    pub nominal_value_1: Real,
    pub nominal_value_2: Real,
    pub notional_scaling_multiplier: Real,
    pub payoff_at_settlement: Real,
    pub performance: Option<ContractPerformance>,
    pub time_at_maturity_date: Time,
}

impl Variables {
    pub fn new() -> Variables {
        Variables {
            fee_accrued: Real(None),
            interest_calculation_base: Real(None),
            interest_scaling_multiplier: Real(None),
            last_event_date: Time(None),
            next_principal_redemption_payment: Real(None),
            nominal_accrued_1: Real(None),
            nominal_accrued_2: Real(None),
            nominal_rate: Real(None),
            nominal_value_1: Real(None),
            nominal_value_2: Real(None),
            notional_scaling_multiplier: Real(None),
            payoff_at_settlement: Real(None),
            performance: None,
            time_at_maturity_date: Time(None),
        }
    }
}
