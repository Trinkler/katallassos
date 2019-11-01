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
        if !<AssetsBalances<T>>::exists((asset_id, from_address)) {
            return Err("From_address doesn't exist at given Asset_ID.");
        }

        // Checking that from_address has enough balance.
        if amount > <AssetsBalances<T>>::get((asset_id, from_address)) {
            return Err("From_address doesn't have enough balance.");
        }

        // Decreasing supply.
        let new_supply = <AssetsSupply<T>>::get(asset_id) - amount;
        <AssetsSupply<T>>::insert(asset_id, new_supply);

        // Deducting amount from from_address.
        let new_balance = <AssetsBalances<T>>::get((asset_id, from_address)) - amount;
        if new_balance == Real::from(0) {
            <AssetsBalances<T>>::remove((asset_id, from_address));
        } else {
            <AssetsBalances<T>>::insert((asset_id, from_address), new_balance);
        }

        // Return Ok.
        Ok(())
    }
}

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
    type Assets = Module<Test>;

    fn new_test_ext() -> runtime_io::TestExternalities<Blake2Hasher> {
        system::GenesisConfig::<Test>::default()
            .build_storage()
            .unwrap()
            .0
            .into()
    }

    #[test]
    fn burn_works() {
        with_externalities(&mut new_test_ext(), || {
            // Initialize some values.
            let supply = Real::from(1000);
            let from_address = H256::random();
            let from_balance = Real::from(450);
            let asset_id = 1;

            // Manually store addresses with balances.
            <AssetsSupply<Test>>::insert(asset_id, supply);
            <AssetsBalances<Test>>::insert((asset_id, from_address), from_balance);

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
            assert_eq!(supply - amount, <AssetsSupply<Test>>::get(asset_id));
            assert_eq!(
                from_balance - amount,
                <AssetsBalances<Test>>::get((asset_id, from_address))
            );
        });
    }
}
