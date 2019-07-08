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

use parity_codec::{Decode, Encode};
use reals::Real;
use rstd::prelude::*;
use support::{decl_event, decl_module, decl_storage, dispatch::Result, StorageMap};
use time::{Time, UncheckedTime};

// Importing the rest of the files in this crate.
mod contract_state;
mod contracts;
mod utilities;
use contract_state::*;
use contracts::*;
use utilities::*;

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
        ContractStates: map u128 => ContractState;
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

            // Checking if id is available.
            if <ContractStates<T>>::exists(id) {
                // TODO: Get current time.
                let t0 = Time::from_values(2019, 07, 04, 00, 00, 00);

                // Calculating the initial contract state.
                let state = contracts::initialize(t0, attributes)?;

                // Storing the contract state.
                <ContractStates<T>>::insert(id, state);
            }

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
    type ACTUS = Module<Test>;

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
    fn it_works_for_default_value() {
        with_externalities(&mut new_test_ext(), || {});
    }
}
