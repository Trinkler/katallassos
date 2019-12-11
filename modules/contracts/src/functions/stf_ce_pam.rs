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

pub fn stf_ce_pam(event: Event, t0: &Time, mut contract: Contract) -> Contract {
    contract.states.accrued_interest = contract.states.accrued_interest
        + utilities::year_fraction(
            contract.states.status_date,
            event.time,
            contract.terms.day_count_convention.unwrap(), // This unwrap will never panic.
        ) * contract.states.nominal_interest_rate
            * contract.states.notional_principal;
    contract.states.status_date = event.time;
    // Return the progressed contract state
    contract
}
