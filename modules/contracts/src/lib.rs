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

#![cfg_attr(not(feature = "std"), no_std)]
// The above line is needed to compile the Wasm binaries.

// Importing crates necessary to work with Substrate.
use codec::{Decode, Encode};
use primitives::H256;
use support::{decl_module, decl_storage, dispatch::Result, StorageMap, StorageValue};
// This import is used to convert the timestamp to a Time.
use sr_primitives::traits::SaturatedConversion;

// Importing types and structures.
use structures::*;

// Importing the rest of the files in this crate.
mod contract_state;
mod contract_types;
mod deploy;
mod init;
mod progress;
mod scheduler;
mod utilities;
use contract_state::*;
use contract_types::*;
use deploy::*;
use init::*;
use progress::*;
use scheduler::*;
use utilities::*;

// This module's configuration trait.
pub trait Trait: system::Trait + oracle::Trait + assets::Trait + timestamp::Trait {}

// This module's storage items.
decl_storage! {
    trait Store for Module<T: Trait> as ContractsStorage {
        pub ContractStates: map H256 => ContractState;
        pub Scheduler: MinHeap<ScheduledEvent> = MinHeap::new();
    }
}

// This module's dispatchable functions.
decl_module! {
    // The module declaration.
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {

        fn on_initialize(_now: T::BlockNumber) {
            // At the beginning of each block execution, system triggers all
            // `on_initialize` functions, which allows us to run the schedule.
            Self::init();
        }

        pub fn dispatch_deploy(origin, terms: Terms) -> Result {
            // Call corresponding internal function.
            // TODO: Check for third party signatures
            Self::deploy(terms)?;

            // Return Ok if successful.
            Ok(())
        }

        pub fn dispatch_progress(origin, event: Event, contract_id: H256) -> Result {
            // TODO: Assert rules for user initiated events

            // Call corresponding internal function.
            Self::progress(event, contract_id)?;

            // Return Ok if successful.
            Ok(())
        }
    }
}
