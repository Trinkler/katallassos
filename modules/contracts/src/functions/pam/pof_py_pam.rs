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
impl<T: Trait> Module<T> {
    pub fn pof_py_pam(event: Event, contract: &Contract) -> Real {
        // TODO: Add O^{rf}(CURS, t)
        let mut payoff = Real::from(0);
        if contract.terms.penalty_type == Some(PenaltyType::A) {
            payoff = utilities::contract_role_sign(contract.terms.contract_role)
                * contract.terms.penalty_rate;
        }
        if contract.terms.penalty_type == Some(PenaltyType::N) {
            payoff = utilities::contract_role_sign(contract.terms.contract_role)
                * utilities::year_fraction(
                    contract.states.status_date,
                    event.time,
                    contract.terms.day_count_convention.unwrap(), // This unwrap will never panic.
                )
                * contract.states.notional_principal
                * contract.terms.penalty_rate;
        }
        if contract.terms.penalty_type == Some(PenaltyType::I) {
            payoff = utilities::contract_role_sign(contract.terms.contract_role)
                * utilities::year_fraction(
                    contract.states.status_date,
                    event.time,
                    contract.terms.day_count_convention.unwrap(), // This unwrap will never panic.
                )
                * contract.states.notional_principal
                * Real::max(
                    Real::from(0),
                    contract.states.nominal_interest_rate
                        - <oracle::Module<T>>::oracles(
                            contract.terms.market_object_code_rate_reset.unwrap(), //This unwrap will never panic.
                        )
                        .value,
                );
        }
        // Return the calculated payoff
        payoff
    }
}
