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

use super::*;

// This function burns tokens of a given asset and from a specific address.
impl<T: Trait> Module<T> {
    pub fn burn(from_address: H256, asset_id: u32, amount: Real) -> Result {
        // Checking that amount is non-negative.
        if amount < Real::from(0) {
            return Err("Amount can't be negative.");
        }

        // Checking that from_address and asset_id exists.
        if !<Self as Store>::AssetsBalances::exists((asset_id, from_address)) {
            return Err("From_address doesn't exist at given Asset_ID.");
        }

        // Checking that from_address has enough balance.
        if amount > <Self as Store>::AssetsBalances::get((asset_id, from_address)) {
            return Err("From_address doesn't have enough balance.");
        }

        // Decreasing supply.
        let new_supply = <Self as Store>::AssetsSupply::get(asset_id) - amount;
        <Self as Store>::AssetsSupply::insert(asset_id, new_supply);

        // Deducting amount from from_address.
        let new_balance = <Self as Store>::AssetsBalances::get((asset_id, from_address)) - amount;
        if new_balance == Real::from(0) {
            <Self as Store>::AssetsBalances::remove((asset_id, from_address));
        } else {
            <Self as Store>::AssetsBalances::insert((asset_id, from_address), new_balance);
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
    fn burn_works() {
        new_test_ext().execute_with(|| {
            // Initialize some values.
            let supply = Real::from(1000);
            let from_address = H256::random();
            let from_balance = Real::from(450);
            let asset_id = 1;

            // Manually store addresses with balances.
            <Assets as Store>::AssetsSupply::insert(asset_id, supply);
            <Assets as Store>::AssetsBalances::insert((asset_id, from_address), from_balance);

            // Test case of negative transfer amount.
            let mut amount = Real::from(-100);
            assert!(Assets::burn(from_address, asset_id, amount).is_err());

            // Test case of insuficient balance.
            amount = Real::from(1000000);
            assert!(Assets::burn(from_address, asset_id, amount).is_err());

            // Test case of non-existent address.
            amount = Real::from(50);
            assert!(Assets::burn(H256::random(), asset_id, amount).is_err());

            // Test case of non-existent asset_id.
            assert!(Assets::burn(from_address, 999, amount).is_err());

            // Test normal case.
            assert!(Assets::burn(from_address, asset_id, amount).is_ok());
            assert_eq!(
                supply - amount,
                <Assets as Store>::AssetsSupply::get(asset_id)
            );
            assert_eq!(
                from_balance - amount,
                <Assets as Store>::AssetsBalances::get((asset_id, from_address))
            );
        });
    }
}
