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

/// All ACTUS contract states as specifed in the ACTUS paper.
#[derive(Clone, Copy, Decode, Debug, Encode, Default, PartialEq)]
pub struct States {
    pub accrued_interest: Real,
    pub accrued_interest_2: Real,
    pub contract_performance: Option<ContractPerformance>,
    pub exercise_amount: Real,
    pub exercise_date: Time,
    pub fee_accrued: Real,
    pub interest_calculation_base: Real,
    pub interest_scaling_multiplier: Real,
    pub next_principal_redemption_payment: Real,
    pub nominal_interest_rate: Real,
    pub nominal_interest_rate_2: Real,
    pub non_performing_date: Time,
    pub notional_principal: Real,
    pub notional_principal_2: Real,
    pub notional_scaling_multiplier: Real,
    pub status_date: Time,
    pub time_at_maturity_date: Time,
}

impl States {
    // Creates an instance of States with every field set to None.
    pub fn new() -> States {
        States {
            accrued_interest: Real(None),
            accrued_interest_2: Real(None),
            contract_performance: None,
            exercise_amount: Real(None),
            exercise_date: Time(None),
            fee_accrued: Real(None),
            interest_calculation_base: Real(None),
            interest_scaling_multiplier: Real(None),
            next_principal_redemption_payment: Real(None),
            nominal_interest_rate: Real(None),
            nominal_interest_rate_2: Real(None),
            non_performing_date: Time(None),
            notional_principal: Real(None),
            notional_principal_2: Real(None),
            notional_scaling_multiplier: Real(None),
            status_date: Time(None),
            time_at_maturity_date: Time(None),
        }
    }
}
