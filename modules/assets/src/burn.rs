use super::*;

//
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
            let supply = Real::from(1000);
            let from_address = H256::random();
            let from_balance = Real::from(450);
            let asset_id = 1;
            let amount = Real::from(50);

            <AssetsSupply<Test>>::insert(asset_id, supply);
            <AssetsBalances<Test>>::insert((asset_id, from_address), from_balance);

            Assets::burn(from_address, asset_id, amount);

            assert_eq!(supply - amount, <AssetsSupply<Test>>::get(asset_id));
            assert_eq!(
                from_balance - amount,
                <AssetsBalances<Test>>::get((asset_id, from_address))
            );
        });
    }
}
