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

// This function sets an (or creates a new) oracle.
impl<T: Trait> Module<T> {
    pub fn set(id: H256, value: Real) -> Result {
        let unix_time = <timestamp::Module<T>>::get().saturated_into::<u64>();
        let time = Time::from_unix(unix_time);

        // Create the oracle state struct.
        let state = OracleState {
            time: time,
            value: value,
        };

        // Store input value in storage.
        <Self as Store>::Oracles::insert(id, state);

        // Return Ok if successful.
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use primitives::H256;
    // The testing primitives are very useful for avoiding having to work with signatures
    // or public keys. `u64` is used as the `AccountId` and no `Signature`s are required.
    use sr_primitives::{
        testing::Header,
        traits::{BlakeTwo256, IdentityLookup},
        Perbill,
    };
    use support::{assert_ok, impl_outer_origin, parameter_types};

    impl_outer_origin! {
        pub enum Origin for Test {}
    }

    // For testing the module, we construct most of a mock runtime. This means
    // first constructing a configuration type (`Test`) which `impl`s each of the
    // configuration traits of modules we want to use.
    #[derive(Clone, Eq, PartialEq)]
    pub struct Test;
    parameter_types! {
        pub const BlockHashCount: u64 = 250;
        pub const MaximumBlockWeight: u32 = 1024;
        pub const MaximumBlockLength: u32 = 2 * 1024;
        pub const AvailableBlockRatio: Perbill = Perbill::one();
    }
    impl system::Trait for Test {
        type Origin = Origin;
        type Index = u64;
        type Call = ();
        type BlockNumber = u64;
        type Hash = H256;
        type Hashing = BlakeTwo256;
        type AccountId = u64;
        type Lookup = IdentityLookup<Self::AccountId>;
        type Header = Header;
        type Event = ();
        type BlockHashCount = BlockHashCount;
        type MaximumBlockWeight = MaximumBlockWeight;
        type AvailableBlockRatio = AvailableBlockRatio;
        type MaximumBlockLength = MaximumBlockLength;
        type Version = ();
    }

    pub const MILLISECS_PER_BLOCK: u64 = 6000;
    pub const SLOT_DURATION: u64 = MILLISECS_PER_BLOCK;
    parameter_types! {
        pub const MinimumPeriod: u64 = SLOT_DURATION / 2;
    }
    impl timestamp::Trait for Test {
        type Moment = u64;
        type OnTimestampSet = ();
        type MinimumPeriod = MinimumPeriod;
    }
    impl Trait for Test {}
    type Oracle = Module<Test>;

    // This function basically just builds a genesis storage key/value store according to
    // our desired mockup.
    fn new_test_ext() -> runtime_io::TestExternalities {
        system::GenesisConfig::default()
            .build_storage::<Test>()
            .unwrap()
            .into()
    }

    #[test]
    fn set_should_work() {
        new_test_ext().execute_with(|| {
            let id = H256::zero();
            let time = Time::from_values(1970, 01, 01, 00, 00, 00);
            let value = Real::from(1000);

            // Set oracle state to storage
            assert_ok!(Oracle::set(H256::zero(), value));
            // Get oracle state from storage.
            assert_eq!(time, <Oracle as Store>::Oracles::get(id).time);
            assert_eq!(value, <Oracle as Store>::Oracles::get(id).value);
        });
    }

    #[test]
    fn dispatch_set_should_work() {
        new_test_ext().execute_with(|| {
            let id = H256::zero();
            let time = Time::from_values(1970, 01, 01, 00, 00, 00);
            let value = Real::from(1000);

            // Set oracle state to storage
            assert_ok!(Oracle::dispatch_set(Origin::ROOT, id, value));
            // Get oracle state from storage.
            assert_eq!(time, <Oracle as Store>::Oracles::get(id).time);
            assert_eq!(value, <Oracle as Store>::Oracles::get(id).value);
        });
    }
}
