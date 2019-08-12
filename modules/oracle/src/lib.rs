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

#![cfg_attr(not(feature = "std"), no_std)]
// The above line is needed to compile the Wasm binaries.

use parity_codec::{Decode, Encode};
use primitives::H256;
use reals::*;
use support::{decl_module, decl_storage, dispatch::Result, StorageMap};
use system::ensure_root;
use time::*;

// This struct defines the state of an oracle.
#[derive(Clone, Decode, Encode, Default, PartialEq)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct OracleState {
    pub time: Time,
    pub value: Real,
}

// The module's configuration trait.
pub trait Trait: system::Trait {}

// This module's storage items.
decl_storage! {
    trait Store for Module<T: Trait> as OracleStorage {
        OracleStorage: map H256 => OracleState;
    }
}

// This module's dispatchable functions.
decl_module! {
    // The module declaration.
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {

        // Set the value of an existing data feed or creating a new one.
        pub fn set(origin, id: H256, time: Time, value: Real) -> Result {
            // Only chain root should be able to set this value.
            ensure_root(origin)?;

            // Create the oracle state struct.
            let state = OracleState {
                time: time,
                value: value,
            };

            // Store input value in storage.
            <Self as Store>::OracleStorage::insert(id, state);

            Ok(())
        }
    }
}

// tests for this module
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

    // For testing the module, we construct most of a mock runtime. This means
    // first constructing a configuration type (`Test`) which `impl`s each of the
    // configuration traits of modules we want to use.
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
    impl Trait for Test {
        type Event = ();
    }
    type oracle = Module<Test>;

    // This function basically just builds a genesis storage key/value store according to
    // our desired mockup.
    fn new_test_ext() -> runtime_io::TestExternalities<Blake2Hasher> {
        system::GenesisConfig::<Test>::default()
            .build_storage()
            .unwrap()
            .0
            .into()
    }

    // #[test]
    // fn it_can_set_and_get_random_values() {
    //     with_externalities(&mut new_test_ext(), || {
    //         let price: u64 = rand::random::<u64>();
    //         // Set price to storage
    //         assert_ok!(oracle::set(Origin::ROOT, price));
    //         // Get price from storage
    //         assert_eq!(oracle::price(), price);
    //     });
    // }
}
