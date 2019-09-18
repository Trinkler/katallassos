use super::*;

// This function adds a scheduled event to the List. Note that there will only be one
// scheduled event per contract at any given time.
impl<T: Trait> Module<T> {
    pub fn add(contract_id: H256, time: Time, index: u32) -> Result {
        // This check is necessary also because the number of elements in a vector
        // cannot exceed 'usize', which is at least 32 bits.
        if <Counter<T>>::get() == u32::max_value() {
            return Err("Scheduler list is full");
        }

        // Create the new event.
        let event = ScheduledEvent {
            contract_id: contract_id,
            time: time,
            index: index,
        };
        // Get the current Scheduler List from storage.
        let mut list = <List<T>>::get();
        // Add the event to the Scheduler List.
        list.push(event);
        // Put the Scheduler List into storage and increase the Scheduler Counter.
        <List<T>>::put(list);
        <Counter<T>>::mutate(|n| *n += 1);

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
    fn add_works() {
        with_externalities(&mut new_test_ext(), || {
            let contract_id = H256::zero();
            let time = Time::from_values(1969, 07, 20, 20, 17, 00);
            let index = 0;
            let event = ScheduledEvent {
                contract_id: contract_id,
                time: time,
                index: index,
            };

            // Test the normal case.
            Scheduler::add(contract_id, time, index);
            assert_eq!(event, <List<Test>>::get()[0]);
            assert_eq!(1, <Counter<Test>>::get());

            // Test case when list is full.
            <Counter<Test>>::put(u32::max_value());
            assert_eq!(
                Err("Scheduler list is full"),
                Scheduler::add(contract_id, time, index)
            );
        });
    }
}
