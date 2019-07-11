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

use primitives::traits::One;
use primitives::traits::{Member, SimpleArithmetic, StaticLookup, Zero};
use support::{decl_event, decl_module, decl_storage, ensure, Parameter, StorageMap, StorageValue};
use system::ensure_signed;

/// The module configuration trait.
pub trait Trait: system::Trait {
    /// The overarching event type.
    type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;

    /// The units in which we record balances.
    type Balance: Member + Parameter + SimpleArithmetic + Default + Copy;

    /// The arithmetic type of asset identifier.
    type AssetId: Parameter + SimpleArithmetic + Default + Copy;
}

decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        fn deposit_event<T>() = default;
        /// Issue a new class of fungible ownership. There are, and will only ever be, `total`
        /// such ownership and they'll all belong to the `origin` initially. It will have an
        /// identifier `AssetId` instance: this will be specified in the `Issued` event.
        fn issue(origin, #[compact] total: T::Balance) {
            let origin = ensure_signed(origin)?;

            let id = Self::next_asset_id();
            <NextAssetId<T>>::mutate(|id| *id += One::one());

            <Balances<T>>::insert((id, origin.clone()), total);
            <TotalSupply<T>>::insert(id, total);

            Self::deposit_event(RawEvent::Issued(id, origin, total));
        }

        /// Move some ownership from one holder to another.
        fn transfer(origin,
            #[compact] id: T::AssetId,
            target: <T::Lookup as StaticLookup>::Source,
            #[compact] amount: T::Balance
        ) {
            let origin = ensure_signed(origin)?;
            let origin_account = (id, origin.clone());
            let origin_balance = <Balances<T>>::get(&origin_account);
            let target = T::Lookup::lookup(target)?;
            ensure!(!amount.is_zero(), "transfer amount should be non-zero");
            ensure!(origin_balance >= amount, "origin account balance must be greater than or equal to the transfer amount");

            Self::deposit_event(RawEvent::Transferred(id, origin, target.clone(), amount));
            <Balances<T>>::insert(origin_account, origin_balance - amount);
            <Balances<T>>::mutate((id, target), |balance| *balance += amount);
        }

        /// Destroy any ownership of `id` owned by `origin`.
        fn destroy(origin, #[compact] id: T::AssetId) {
            let origin = ensure_signed(origin)?;
            let balance = <Balances<T>>::take((id, origin.clone()));
            ensure!(!balance.is_zero(), "origin balance should be non-zero");

            <TotalSupply<T>>::mutate(id, |total_supply| *total_supply -= balance);
            Self::deposit_event(RawEvent::Destroyed(id, origin, balance));
        }
    }
}

decl_event!(
	pub enum Event<T>
		where <T as system::Trait>::AccountId,
		      <T as Trait>::Balance,
		      <T as Trait>::AssetId {
		/// Some ownership were issued.
		Issued(AssetId, AccountId, Balance),
		/// Some ownership were transferred.
		Transferred(AssetId, AccountId, AccountId, Balance),
		/// Some ownership were destroyed.
		Destroyed(AssetId, AccountId, Balance),
	}
);

decl_storage! {
    trait Store for Module<T: Trait> as Ownership {
        /// The number of units of ownership held by any given account.
        Balances: map (T::AssetId, T::AccountId) => T::Balance;
        /// The next asset identifier up for grabs.
        NextAssetId get(next_asset_id): T::AssetId;
        /// The total unit supply of an asset.
        TotalSupply: map T::AssetId => T::Balance;
    }
}

// The main implementation block for the module.
impl<T: Trait> Module<T> {
    // Public immutables

    /// Get the asset `id` balance of `who`.
    pub fn balance(id: T::AssetId, who: T::AccountId) -> T::Balance {
        <Balances<T>>::get((id, who))
    }

    /// Get the total supply of an asset `id`.
    pub fn total_supply(id: T::AssetId) -> T::Balance {
        <TotalSupply<T>>::get(id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use runtime_io::with_externalities;
    use srml_support::{assert_noop, assert_ok, impl_outer_origin, parameter_types};
    use substrate_primitives::{Blake2Hasher, H256};
    // The testing primitives are very useful for avoiding having to work with signatures
    // or public keys. `u64` is used as the `AccountId` and no `Signature`s are required.
    use primitives::{
        testing::Header,
        traits::{BlakeTwo256, IdentityLookup},
    };

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
    }
    impl system::Trait for Test {
        type Origin = Origin;
        type Index = u64;
        type BlockNumber = u64;
        type Hash = H256;
        type Hashing = BlakeTwo256;
        type AccountId = u64;
        type Lookup = IdentityLookup<Self::AccountId>;
        type Header = Header;
        type Event = ();
        type BlockHashCount = BlockHashCount;
    }
    impl Trait for Test {
        type Event = ();
        type Balance = u64;
        type AssetId = u32;
    }
    type Ownership = Module<Test>;

    // This function basically just builds a genesis storage key/value store according to
    // our desired mockup.
    fn new_test_ext() -> runtime_io::TestExternalities<Blake2Hasher> {
        system::GenesisConfig::default()
            .build_storage::<Test>()
            .unwrap()
            .0
            .into()
    }

    #[test]
    fn issuing_asset_units_to_issuer_should_work() {
        with_externalities(&mut new_test_ext(), || {
            assert_ok!(Ownership::issue(Origin::signed(1), 100));
            assert_eq!(Ownership::balance(0, 1), 100);
        });
    }

    #[test]
    fn querying_total_supply_should_work() {
        with_externalities(&mut new_test_ext(), || {
            assert_ok!(Ownership::issue(Origin::signed(1), 100));
            assert_eq!(Ownership::balance(0, 1), 100);
            assert_ok!(Ownership::transfer(Origin::signed(1), 0, 2, 50));
            assert_eq!(Ownership::balance(0, 1), 50);
            assert_eq!(Ownership::balance(0, 2), 50);
            assert_ok!(Ownership::transfer(Origin::signed(2), 0, 3, 31));
            assert_eq!(Ownership::balance(0, 1), 50);
            assert_eq!(Ownership::balance(0, 2), 19);
            assert_eq!(Ownership::balance(0, 3), 31);
            assert_ok!(Ownership::destroy(Origin::signed(3), 0));
            assert_eq!(Ownership::total_supply(0), 69);
        });
    }

    #[test]
    fn transferring_amount_above_available_balance_should_work() {
        with_externalities(&mut new_test_ext(), || {
            assert_ok!(Ownership::issue(Origin::signed(1), 100));
            assert_eq!(Ownership::balance(0, 1), 100);
            assert_ok!(Ownership::transfer(Origin::signed(1), 0, 2, 50));
            assert_eq!(Ownership::balance(0, 1), 50);
            assert_eq!(Ownership::balance(0, 2), 50);
        });
    }

    #[test]
    fn transferring_amount_less_than_available_balance_should_not_work() {
        with_externalities(&mut new_test_ext(), || {
            assert_ok!(Ownership::issue(Origin::signed(1), 100));
            assert_eq!(Ownership::balance(0, 1), 100);
            assert_ok!(Ownership::transfer(Origin::signed(1), 0, 2, 50));
            assert_eq!(Ownership::balance(0, 1), 50);
            assert_eq!(Ownership::balance(0, 2), 50);
            assert_ok!(Ownership::destroy(Origin::signed(1), 0));
            assert_eq!(Ownership::balance(0, 1), 0);
            assert_noop!(
                Ownership::transfer(Origin::signed(1), 0, 1, 50),
                "origin account balance must be greater than or equal to the transfer amount"
            );
        });
    }

    #[test]
    fn transferring_less_than_one_unit_should_not_work() {
        with_externalities(&mut new_test_ext(), || {
            assert_ok!(Ownership::issue(Origin::signed(1), 100));
            assert_eq!(Ownership::balance(0, 1), 100);
            assert_noop!(
                Ownership::transfer(Origin::signed(1), 0, 2, 0),
                "transfer amount should be non-zero"
            );
        });
    }

    #[test]
    fn transferring_more_units_than_total_supply_should_not_work() {
        with_externalities(&mut new_test_ext(), || {
            assert_ok!(Ownership::issue(Origin::signed(1), 100));
            assert_eq!(Ownership::balance(0, 1), 100);
            assert_noop!(
                Ownership::transfer(Origin::signed(1), 0, 2, 101),
                "origin account balance must be greater than or equal to the transfer amount"
            );
        });
    }

    #[test]
    fn destroying_asset_balance_with_positive_balance_should_work() {
        with_externalities(&mut new_test_ext(), || {
            assert_ok!(Ownership::issue(Origin::signed(1), 100));
            assert_eq!(Ownership::balance(0, 1), 100);
            assert_ok!(Ownership::destroy(Origin::signed(1), 0));
        });
    }

    #[test]
    fn destroying_asset_balance_with_zero_balance_should_not_work() {
        with_externalities(&mut new_test_ext(), || {
            assert_ok!(Ownership::issue(Origin::signed(1), 100));
            assert_eq!(Ownership::balance(0, 2), 0);
            assert_noop!(
                Ownership::destroy(Origin::signed(2), 0),
                "origin balance should be non-zero"
            );
        });
    }
}
