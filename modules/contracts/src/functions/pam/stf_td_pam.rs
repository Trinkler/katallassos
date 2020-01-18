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

pub fn stf_td_pam(event: Event, t0: &Time, mut contract: Contract) -> Contract {
    contract.states.notional_principal = Real::from(0);
    contract.states.accrued_interest = Real::from(0);
    contract.states.fee_accrued = Real::from(0);
    contract.states.nominal_interest_rate = Real::from(0);
    contract.states.status_date = event.time;
    // Return the progressed contract state
    contract
}
