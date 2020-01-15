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

pub fn stf_ied_pam(event: Event, t0: &Time, mut contract: Contract) -> Contract {
    contract.states.notional_principal =
        utilities::contract_role_sign(contract.terms.contract_role)
            * contract.terms.notional_principal;
    if contract.terms.nominal_interest_rate == Real(None) {
        contract.states.nominal_interest_rate = Real::from(0);
    } else {
        contract.states.nominal_interest_rate = contract.terms.nominal_interest_rate;
    }
    if contract.terms.accrued_interest != Real(None) {
        contract.states.accrued_interest = contract.terms.accrued_interest;
    } else if contract.terms.cycle_anchor_date_of_interest_payment != Time(None)
        && contract.terms.cycle_anchor_date_of_interest_payment < event.time
    {
        let y = utilities::year_fraction(
            contract.terms.cycle_anchor_date_of_interest_payment,
            event.time,
            contract.terms.day_count_convention.unwrap(), // Unwraping poses no danger since day_count_convention is mandatory for the PAM contract. It will never panic.
        );
        contract.states.accrued_interest =
            y * contract.states.notional_principal * contract.states.nominal_interest_rate;
    } else {
        contract.states.accrued_interest = Real::from(0);
    }
    contract.states.status_date = event.time;
    // Return the progressed contract state
    contract
}
