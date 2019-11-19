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

// This function creates a new ACTUS contract.
impl<T: Trait> Module<T> {
    pub fn deploy(attributes: Attributes) -> Result {
        // Getting the contract ID.
        let id = attributes.contract_id;

        // Checking if ID is available.
        if <ContractStates<T>>::exists(id) {
            return Err("Contract ID already exists");
        }

        // Get current time.
        let t0 = Time::from_unix(<timestamp::Module<T>>::get().as_());

        // Calculating the initial contract state.
        let state;
        match attributes.contract_type {
            Some(ContractType::PAM) => {
                state = Self::deploy_pam(t0, attributes)?;
            }
            _ => {
                state = Err("Contract type not supported")?;
            }
        }

        // Adding first event to the heap.
        let mut heap = <Scheduler<T>>::get();
        let event = ScheduledEvent {
            time: state.schedule[0].time,
            contract_id: id,
            index: 0,
        };
        heap.push(event);
        <Scheduler<T>>::put(heap);

        // Storing the contract state.
        <ContractStates<T>>::insert(id, state);

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
    impl oracle::Trait for Test {}
    impl assets::Trait for Test {}
    impl Trait for Test {}
    type Contracts = Module<Test>;

    fn new_test_ext() -> runtime_io::TestExternalities<Blake2Hasher> {
        system::GenesisConfig::<Test>::default()
            .build_storage()
            .unwrap()
            .0
            .into()
    }

    #[test]
    fn deploy_works() {
        with_externalities(&mut new_test_ext(), || {
            // Mock parameters and initialize attributes
            let t0 = Time::from_values(1969, 07, 20, 20, 17, 00);
            let id = H256::random();
            let mut attributes = Attributes::new(id);

            // Starts a PAM contract with the right attributes.
            attributes.counterparty_id = Some(H256::random());
            attributes.contract_deal_date = Time::from_values(1968, 07, 21, 02, 56, 15);
            attributes.contract_id = id;
            attributes.contract_role = Some(ContractRole::RPA);
            attributes.contract_type = Some(ContractType::PAM);
            attributes.creator_id = Some(H256::random());
            attributes.currency = Some(1);
            attributes.day_count_convention = Some(DayCountConvention::A365);
            attributes.initial_exchange_date = Time::from_values(1969, 07, 21, 02, 56, 15);
            attributes.maturity_date = Time::from_values(1979, 07, 21, 02, 56, 15);
            attributes.nominal_interest_rate = Real::from(1000);
            attributes.notional_principal = Real(Some(50000000));
            attributes.scaling_effect = None;
            let result = Contracts::deploy(attributes.clone());
            assert!(result.is_ok());

            // Checks if contract state has been stored
            assert_eq!(ContractStates::<Test>::exists(id), true);

            // Checks if scheduler was correctly updated.
            let event = <Scheduler<Test>>::get().pop().unwrap();
            let state = ContractStates::<Test>::get(id);
            assert_eq!(event.time, state.schedule[0].time);
            assert_eq!(event.contract_id, state.attributes.contract_id);
            assert_eq!(event.index, 0);
        });
    }
}
