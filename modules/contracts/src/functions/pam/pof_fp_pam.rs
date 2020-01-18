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

pub fn pof_fp_pam(event: Event, contract: &Contract) -> Real {
    let mut payoff = Real::from(0);
    if contract.terms.fee_basis == Some(FeeBasis::A) {
        payoff =
            utilities::contract_role_sign(contract.terms.contract_role) * contract.terms.fee_rate;
    }
    if contract.terms.fee_basis == Some(FeeBasis::N) {
        payoff = contract.terms.fee_rate
            * utilities::year_fraction(
                contract.states.status_date,
                event.time,
                contract.terms.day_count_convention.unwrap(), // This unwrap will never panic.
            )
            * contract.states.notional_principal
            + contract.states.fee_accrued;
    }
    // Return the calculated payoff
    payoff
}
