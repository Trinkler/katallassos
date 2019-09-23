use super::*;

// This function sets an (or creates a new) oracle.
impl<T: Trait> Module<T> {
    pub fn set(id: H256, value: Real) -> Result {
        let unix_time = <timestamp::Module<T>>::get().as_();
        let time = Time::from_unix(unix_time);

        // Create the oracle state struct.
        let state = OracleState {
            time: time,
            value: value,
        };

        // Store input value in storage.
        <Oracles<T>>::insert(id, state);

        // Return Ok if successful.
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
    impl timestamp::Trait for Test {
        type Moment = u64;
        type OnTimestampSet = ();
    }
    impl Trait for Test {}
    type Oracle = Module<Test>;

    fn new_test_ext() -> runtime_io::TestExternalities<Blake2Hasher> {
        system::GenesisConfig::<Test>::default()
            .build_storage()
            .unwrap()
            .0
            .into()
    }

    #[test]
    fn set_works() {
        with_externalities(&mut new_test_ext(), || {
            let id = H256::zero();
            let time = Time::from_values(1970, 01, 01, 00, 00, 00);
            let value = Real::from(1000);

            // Set oracle state to storage
            assert_ok!(Oracle::set(id, value));

            // Get oracle state from storage.
            assert_eq!(time, <Oracles<Test>>::get(id).time);
            assert_eq!(value, <Oracles<Test>>::get(id).value);
        });
    }
}
