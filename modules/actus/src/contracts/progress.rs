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

impl<T: Trait> Module<T> {
    pub fn progress(event: ContractEvent, mut state: ContractState) -> MyResult<ContractState> {
        match state.attributes.contract_type {
            Some(ContractType::PAM) => Self::progress_pam(event, state),
            _ => Err("Contract type not supported"),
        }
    }
}
