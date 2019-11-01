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

/// Contract Default Convention:it maps a given contract status to either a 1 or a 0, representing
/// a performant or a defaulted contract. See section 4.8 of the ACTUS paper for details.
pub fn contract_default(contract_performance: Option<ContractPerformance>) -> Real {
    match contract_performance {
        Some(ContractPerformance::PF) => Real::from(1),
        Some(ContractPerformance::DL) => Real::from(1),
        Some(ContractPerformance::DQ) => Real::from(1),
        Some(ContractPerformance::DF) => Real::from(0),
        None => Real(None),
    }
}
