use super::*;

//
impl<T: Trait> Module<T> {
    pub fn mint(to_address: H256, asset_id: u32, amount: Real) -> Result {
        // Checking that amount is non-negative.
        if amount < Real::from(0) {
            return Err("Amount can't be negative.");
        }

        // Increasing supply.
        if <AssetsSupply<T>>::exists(asset_id) {
            let new_supply = <AssetsSupply<T>>::get(asset_id) + amount;
            <AssetsSupply<T>>::insert(asset_id, new_supply);
        } else {
            <AssetsSupply<T>>::insert(asset_id, amount);
        }

        // Crediting amount to to_address.
        if <AssetsBalances<T>>::exists((asset_id, to_address)) {
            let new_balance = <AssetsBalances<T>>::get((asset_id, to_address)) + amount;
            <AssetsBalances<T>>::insert((asset_id, to_address), new_balance);
        } else {
            <AssetsBalances<T>>::insert((asset_id, to_address), amount);
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
    fn mint_works() {
        with_externalities(&mut new_test_ext(), || {
            let supply = Real::from(999000);
            let to_address = H256::random();
            let to_balance = Real::from(200);
            let asset_id = 1;
            let amount = Real::from(1000);

            <AssetsSupply<Test>>::insert(asset_id, supply);
            <AssetsBalances<Test>>::insert((asset_id, to_address), to_balance);

            Assets::mint(to_address, asset_id, amount);

            assert_eq!(supply + amount, <AssetsSupply<Test>>::get(asset_id));
            assert_eq!(
                to_balance + amount,
                <AssetsBalances<Test>>::get((asset_id, to_address))
            );
        });
    }
}
