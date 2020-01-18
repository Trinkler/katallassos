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

/// Contract Role Sign Convention: it maps a given contract role to either a 1 or a -1,
/// representing a direction for cashflows. See section 4.7 of the ACTUS paper for details.
pub fn contract_role_sign(contract_role: Option<ContractRole>) -> Real {
    match contract_role {
        Some(ContractRole::RPA) => Real::from(1),
        Some(ContractRole::RPL) => Real::from(-1),
        Some(ContractRole::LG) => Real::from(1),
        Some(ContractRole::ST) => Real::from(-1),
        Some(ContractRole::BUY) => Real::from(1),
        Some(ContractRole::SEL) => Real::from(-1),
        Some(ContractRole::RFL) => Real::from(1),
        Some(ContractRole::PFL) => Real::from(-1),
        Some(ContractRole::COL) => Real::from(1),
        Some(ContractRole::GUA) => Real::from(-1),
        Some(ContractRole::OBL) => Real::from(1),
        None => Real(None),
    }
}
