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

// Importing crates from Katal's runtime.
use structures::*;

// Importing the rest of the files in this crate.
mod oracle_state;
mod set;
use oracle_state::*;
use set::*;

// This module's configuration trait.
pub trait Trait: system::Trait {}

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
        pub fn dispatch_set(origin, id: H256, time: Time, value: Real) -> Result {
            // Only chain root should be able to set this value.
            ensure_root(origin)?;

            // Call corresponding internal function.
            Self::set(id, time, value)?;

            // Return Ok if successful.
            Ok(())
        }
    }
}

// Tests for this module.
#[cfg(test)]
mod tests {
    use super::*;
    use primitives::{Blake2Hasher, H256};
    use runtime_io::with_externalities;
    use runtime_primitives::{
        testing::{Digest, DigestItem, Header},
        traits::{BlakeTwo256, IdentityLookup},
        BuildStorage,
    };
    use support::{assert_ok, impl_outer_origin};

    impl_outer_origin! {
        pub enum Origin for Test {}
    }

    #[derive(Clone, Eq, PartialEq)]
    pub struct Test;
    impl system::Trait for Test {
        type Origin = Origin;
        type Index = u64;
        type BlockNumber = u64;
        type Hash = H256;
        type Hashing = BlakeTwo256;
        type Digest = Digest;
        type AccountId = u64;
        type Lookup = IdentityLookup<Self::AccountId>;
        type Header = Header;
        type Event = ();
        type Log = DigestItem;
    }
    impl Trait for Test {}
    type Oracle = Module<Test>;

    fn new_test_ext() -> runtime_io::TestExternalities<Blake2Hasher> {
        system::GenesisConfig::<Test>::default()
            .build_storage()
            .unwrap()
            .0
            .into()
    }

    #[test]
    fn dispatch_set_works() {
        with_externalities(&mut new_test_ext(), || {
            let id = H256::zero();
            let time = Time::from_values(1969, 07, 20, 20, 17, 00);
            let value = Real::from(1000);

            // Set oracle state to storage
            assert_ok!(Oracle::dispatch_set(Origin::ROOT, id, time, value));

            // Get oracle state from storage.
            assert_eq!(time, <Oracles<Test>>::get(id).time);
            assert_eq!(value, <Oracles<Test>>::get(id).value);
        });
    }
}
