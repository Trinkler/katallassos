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

// Ensure we're `no_std` when compiling for Wasm.
#![cfg_attr(not(feature = "std"), no_std)]

#![cfg_attr(not(feature = "std"), no_std)]
// The above line is needed to compile the Wasm binaries.

use parity_codec::{Decode, Encode};
// use primitives::H256;
use reals::Real;
use rstd::prelude::*;
use support::{decl_event, decl_module, decl_storage, dispatch::Result, StorageMap};
// use time::{Time, UncheckedTime};

// // Importing the rest of the files in this crate.
// mod contract_state;
// mod contracts;
// mod utilities;
// use contract_state::*;
// use contracts::*;
// use utilities::*;
// //
// Defines an alias for the Result type. It has the name MyResult because Substrate already uses
// the name Result for their own type Result<(), &'static str>.
type MyResult<T> = rstd::result::Result<T, &'static str>;

// This module's configuration trait.
pub trait Trait: system::Trait {
    type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
}

// This module's events.
decl_event!(
    pub enum Event<T>
    where
        AccountId = <T as system::Trait>::AccountId,
    {
        Nothing(AccountId),
    }
);

// This module's storage items.
decl_storage! {
    trait Store for Module<T: Trait> as ContractStorage {
        ContractStorage: map H256 => ContractState;
    }
}

// This module's dispatchable functions.
decl_module! {
    /// The module declaration.
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        // Initializing events
        // this is needed only if you are using events in your module
        fn deposit_event<T>() = default;

        fn deploy_contract(origin, attributes: Attributes) -> Result {
            // Getting the contract ID.
            let id = attributes.contract_id;

            // Checking if ID is available.
            if <ContractStorage<T>>::exists(id) {
                return Err("Contract ID already exists");
            }

            // TODO: Get current time.
            let t0 = Time::from_values(1969, 07, 20, 20, 17, 00);

            // Calculating the initial contract state.
            let state = contracts::initialize(t0, attributes)?;

            // Storing the contract state.
            <ContractStorage<T>>::insert(id, state);

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
        // type Digest = Digest; // This must be commented out for tests to work.
        type AccountId = u64;
        type Lookup = IdentityLookup<Self::AccountId>;
        type Header = Header;
        type Event = ();
        // type Log = DigestItem; // This must be commented out for tests to work.
    }
    impl Trait for Test {
        type Event = ();
    }
    type Actus = Module<Test>;

    // This function basically just builds a genesis storage key/value store according to
    // our desired mockup.
    fn new_test_ext() -> runtime_io::TestExternalities<Blake2Hasher> {
        system::GenesisConfig::<Test>::default()
            .build_storage()
            .unwrap()
            .0
            .into()
    }

    #[test]
    fn deploy_contract_works() {
        with_externalities(&mut new_test_ext(), || {
            // Tries to start a contract with the wrong type.
            let id = H256::zero();
            let mut attributes = Attributes::new(id);
            let result = Actus::deploy_contract(Origin::signed(1), attributes.clone());
            assert!(result.is_err());

            // Starts a PAM contract with the wrong attributes.
            attributes.contract_id = id;
            attributes.contract_type = Some(ContractType::PAM);
            attributes.currency = Some(H256::zero());
            attributes.day_count_convention = Some(DayCountConvention::_A365);
            attributes.initial_exchange_date = Time::from_values(1969, 07, 21, 02, 56, 15);
            attributes.maturity_date = Time::from_values(1979, 07, 21, 02, 56, 15);
            attributes.nominal_interest_rate = Real::from(1000);
            attributes.notional_principal = Real(Some(50000000));
            attributes.contract_deal_date = Time::from_values(1969, 07, 21, 02, 56, 15);
            attributes.contract_role = Some(ContractRole::RPA);
            attributes.creator_id = Some(H256::zero());
            attributes.counterparty_id = Some(H256::zero());
            let result = Actus::deploy_contract(Origin::signed(1), attributes.clone());
            assert!(result.is_err());

            // Starts a PAM contract with the right attributes.
            attributes.scaling_effect = None;
            let result = Actus::deploy_contract(Origin::signed(1), attributes.clone());
            assert!(result.is_ok());

            // Starts another contract with the same ID.
            attributes.scaling_effect = None;
            let result = Actus::deploy_contract(Origin::signed(1), attributes.clone());
            assert!(result.is_err());
        });
    }
}
