use super::*;

// This function removes a scheduled event from the List. We assume that there will only be
// one scheduled event per contract at any given time.
impl<T: Trait> Module<T> {
    pub fn remove(contract_id: H256) -> Result {
        // Get the current Scheduler List from storage.
        let mut list = <List<T>>::get();

        // Search for the scheduled event with the desired contract_id and
        // then removes it from the list.
        for i in 0..list.len() {
            if list[i].contract_id == contract_id {
                list.swap_remove(i);
                break;
            }
        }

        // Put the Scheduler List into storage and decrease the Scheduler Counter.
        <List<T>>::put(list);
        <Counter<T>>::mutate(|n| *n -= 1);

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
    impl oracle::Trait for Test {}
    impl actus::Trait for Test {}
    impl Trait for Test {}
    type Scheduler = Module<Test>;

    fn new_test_ext() -> runtime_io::TestExternalities<Blake2Hasher> {
        system::GenesisConfig::<Test>::default()
            .build_storage()
            .unwrap()
            .0
            .into()
    }

    #[test]
    fn remove_works() {
        with_externalities(&mut new_test_ext(), || {
            let contract_id = H256::zero();
            let time = Time::from_values(1969, 07, 20, 20, 17, 00);
            let index = 0;
            let event = ScheduledEvent {
                contract_id: contract_id,
                time: time,
                index: index,
            };
            let vec = vec![event];

            <List<Test>>::put(vec);
            <Counter<Test>>::put(1);

            Scheduler::remove(contract_id);

            assert!(<List<Test>>::get().is_empty());
            assert_eq!(0, <Counter<Test>>::get());
        });
    }
}
