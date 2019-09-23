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

//! # Oracle module
//!
//! ## Overview
//! The Oracle module allows the root user to create and update oracles. An oracle in this
//! context is simply a structure that holds a value (implemented using Real) and a timestamp
//! (implemented using Time). Each oracle is uniquely identified by a 256-bit integer
//! (implemented using H256).

#![cfg_attr(not(feature = "std"), no_std)]
// The above line is needed to compile the Wasm binaries.

// Importing crates necessary to work with Substrate.
use parity_codec::{Decode, Encode};
use primitives::H256;
use runtime_std::prelude::*;
use support::{decl_module, decl_storage, dispatch::Result, StorageMap};
use system::ensure_root;
// This import is used to convert the timestamp to a Time.
use runtime_primitives::traits::As;

// Importing crates from Katal's runtime.
use structures::*;

// Importing the rest of the files in this crate.
mod oracle_state;
mod set;
use oracle_state::*;
use set::*;

// This module's configuration trait.
pub trait Trait: system::Trait + timestamp::Trait {}

// This module's storage items.
decl_storage! {
    trait Store for Module<T: Trait> as OracleStorage {
        pub Oracles: map H256 => OracleState;
    }
}

// This module's dispatchable functions.
decl_module! {
    // The module declaration.
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {

        // Set the value of an existing data feed or creating a new one.
        pub fn dispatch_set(origin, id: H256, value: Real) -> Result {
            // Only chain root should be able to set this value.
            ensure_root(origin)?;

            // Call corresponding internal function.
            Self::set(id, value)?;

            // Return Ok if successful.
            Ok(())
        }
    }
}
