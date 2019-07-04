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
    fee_accrued: Real,
    interest_calculation_base: Real,
    interest_scaling_multiplier: Real,
    last_event_date: Time,
    next_principal_redemption_payment: Real,
    nominal_accrued_1: Real,
    nominal_accrued_2: Real,
    nominal_rate: Real,
    nominal_value_1: Real,
    nominal_value_2: Real,
    notional_scaling_multiplier: Real,
    payoff_at_settlement: Real,
    performance: Option<ContractStatus>,
    time_at_maturity_date: Time,
}
