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

// This function transfers tokens of a given asset from one address to another. If the recipient address doesn't exist, it is created.
impl<T: Trait> Module<T> {
    pub fn transfer(from_address: H256, to_address: H256, asset_id: u32, amount: Real) -> Result {
        // Checking that amount is non-negative.
        if amount < Real::from(0) {
            return Err("Amount can't be negative.");
        }

        // Checking that from_address and to_address are different.
        if from_address == to_address {
            return Err("From_address and to_address can't be equal.");
        }

        // Checking that from_address and asset_id exists.
        if !<Self as Store>::Balances::exists((asset_id, from_address)) {
            return Err("From_address doesn't exist at given Asset_ID.");
        }

        // Checking that from_address has enough balance.
        if amount > <Self as Store>::Balances::get((asset_id, from_address)) {
            return Err("From_address doesn't have enough balance.");
        }

        // Deducting amount from from_address.
        let new_balance = <Self as Store>::Balances::get((asset_id, from_address)) - amount;
        if new_balance == Real::from(0) {
            <Self as Store>::Balances::remove((asset_id, from_address));
        } else {
            <Self as Store>::Balances::insert((asset_id, from_address), new_balance);
        }

        // Crediting amount to to_address.
        if <Self as Store>::Balances::exists((asset_id, to_address)) {
            let new_balance = <Self as Store>::Balances::get((asset_id, to_address)) + amount;
            <Self as Store>::Balances::insert((asset_id, to_address), new_balance);
        } else {
            <Self as Store>::Balances::insert((asset_id, to_address), amount);
        }

        // Return Ok.
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
    use support::{assert_noop, assert_ok, impl_outer_origin, parameter_types};

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
    impl Trait for Test {
        // If Events are ever added to this module, then the next line
        // needs to be commented out.
        // type Event = ();
    }
    // This next line should have the name of the module, in this
    // case it is Assets
    type Assets = Module<Test>;

    // This function basically just builds a genesis storage key/value store according to
    // our desired mockup.
    fn new_test_ext() -> runtime_io::TestExternalities {
        system::GenesisConfig::default()
            .build_storage::<Test>()
            .unwrap()
            .into()
    }

    #[test]
    fn transfer_works() {
        new_test_ext().execute_with(|| {
            // Initialize some values.
            let from_address = H256::random();
            let from_balance = Real::from(1000);
            let to_address = H256::random();
            let to_balance = Real::from(200);
            let asset_id = 1;

            // Manually store addresses with balances.
            <Assets as Store>::Balances::insert((asset_id, from_address), from_balance);
            <Assets as Store>::Balances::insert((asset_id, to_address), to_balance);

            // Test case of negative transfer amount.
            let mut amount = Real::from(-100);
            assert!(Assets::transfer(from_address, to_address, asset_id, amount).is_err());

            // Test case of insuficient balance.
            amount = Real::from(1000000);
            assert!(Assets::transfer(from_address, to_address, asset_id, amount).is_err());

            // Test case of equal addresses.
            amount = Real::from(100);
            assert!(Assets::transfer(from_address, from_address, asset_id, amount).is_err());

            // Test case of non-existent address.
            assert!(Assets::transfer(H256::random(), to_address, asset_id, amount).is_err());

            // Test case of non-existent asset_id.
            assert!(Assets::transfer(from_address, to_address, 999, amount).is_err());

            // Test normal case.
            assert!(Assets::transfer(from_address, to_address, asset_id, amount).is_ok());
            assert_eq!(
                from_balance - amount,
                <Assets as Store>::Balances::get((asset_id, from_address))
            );
            assert_eq!(
                to_balance + amount,
                <Assets as Store>::Balances::get((asset_id, to_address))
            );
        });
    }
}
