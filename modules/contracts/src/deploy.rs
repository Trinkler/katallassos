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
    pub fn deploy(terms: Terms) -> Result {
        // Getting the contract ID.
        // TODO: Determine contract_id as a hash of its terms.
        let id = terms.contract_id;

        // Checking if ID is available.
        if <Self as Store>::Contracts::exists(id) {
            return Err("Contract ID already exists");
        }

        // Get current time.
        let t0 = Time::from_unix(<timestamp::Module<T>>::get().saturated_into::<u64>());

        // Calculating the initial contract state.
        let state;
        match terms.contract_type {
            Some(ContractType::PAM) => {
                state = Self::deploy_pam(t0, terms)?;
            }
            Some(ContractType::ANN) => {
                state = Self::deploy_ann(t0, terms)?;
            }
            _ => {
                state = Err("Contract type not supported")?;
            }
        }

        // Adding first event to the heap.
        let mut heap = <Self as Store>::Scheduler::get();
        let event = ScheduledEvent {
            time: state.schedule[0].time,
            contract_id: id,
            index: 0,
        };
        heap.push(event);
        <Self as Store>::Scheduler::put(heap);

        // Storing the contract state.
        <Self as Store>::Contracts::insert(id, state);

        // Return Ok if successful.
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
    use support::{assert_ok, impl_outer_origin, parameter_types};

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

    pub const MILLISECS_PER_BLOCK: u64 = 6000;
    pub const SLOT_DURATION: u64 = MILLISECS_PER_BLOCK;
    parameter_types! {
        pub const MinimumPeriod: u64 = SLOT_DURATION / 2;
    }
    impl timestamp::Trait for Test {
        type Moment = u64;
        type OnTimestampSet = ();
        type MinimumPeriod = MinimumPeriod;
    }
    impl oracle::Trait for Test {}
    impl assets::Trait for Test {}
    impl Trait for Test {}
    type Contracts = Module<Test>;

    // This function basically just builds a genesis storage key/value store according to
    // our desired mockup.
    fn new_test_ext() -> runtime_io::TestExternalities {
        system::GenesisConfig::default()
            .build_storage::<Test>()
            .unwrap()
            .into()
    }

    #[test]
    fn deploy_works() {
        new_test_ext().execute_with(|| {
            // Mock parameters and initialize terms
            let t0 = Time::from_values(1969, 07, 20, 20, 17, 00);
            let id = H256::random();
            let mut terms = Terms::new(id);

            // Starts a PAM contract with the right terms.
            terms.counterparty_id = Some(H256::random());
            terms.contract_deal_date = Time::from_values(1968, 07, 21, 02, 56, 15);
            terms.contract_id = id;
            terms.contract_role = Some(ContractRole::RPA);
            terms.contract_type = Some(ContractType::PAM);
            terms.creator_id = Some(H256::random());
            terms.currency = Some(1);
            terms.day_count_convention = Some(DayCountConvention::A365);
            terms.initial_exchange_date = Time::from_values(1969, 07, 21, 02, 56, 15);
            terms.maturity_date = Time::from_values(1979, 07, 21, 02, 56, 15);
            terms.nominal_interest_rate = Real::from(1000);
            terms.notional_principal = Real(Some(50000000));
            terms.scaling_effect = None;
            let result = Contracts::deploy(terms.clone());
            assert!(result.is_ok());

            // Checks if contract state has been stored
            assert_eq!(<Contracts as Store>::Contracts::exists(id), true);

            // Checks if scheduler was correctly updated.
            let event = <Contracts as Store>::Scheduler::get().pop().unwrap();
            let state = <Contracts as Store>::Contracts::get(id);
            assert_eq!(event.time, state.schedule[0].time);
            assert_eq!(event.contract_id, state.terms.contract_id);
            assert_eq!(event.index, 0);
        });
    }
}
