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

// This function executes all events for which the time has come.
impl<T: Trait> Module<T> {
    pub fn scheduler_run(now: Time) -> Result {
        // Get the current Scheduler Heap from storage.
        let mut heap = <Self as Store>::Scheduler::get();

        // This loop goes through every scheduled event that is smaller than the
        // current time.
        while heap.peek().is_some() && now >= heap.peek().unwrap().time {
            let mut scheduled_event = heap.pop().unwrap();

            // Get the state of the ACTUS contract and the corresponding
            // contract event type to be executed.
            let mut state = <Self as Store>::ContractStates::get(scheduled_event.contract_id);
            let event = state.schedule[scheduled_event.index as usize];

            // Make the ACTUS contract progress.
            <Module<T>>::progress(event, scheduled_event.contract_id)?;

            // This loop executes the remaining events of the current contract for which
            // the time has come. This is more efficient than just pushing the next event
            // to the Scheduler heap.
            scheduled_event.index += 1;
            while scheduled_event.index < state.schedule.len() as u32 {
                // Get the next event for this contract.
                let event = state.schedule[scheduled_event.index as usize];
                // Compare the event's time with the current time.
                if now >= event.time {
                    // Make the ACTUS contract progress.
                    <Module<T>>::progress(event, scheduled_event.contract_id)?;
                    // Increment the index.
                    scheduled_event.index += 1;
                } else {
                    // Update the time for the scheduled event and push it to the heap.
                    scheduled_event.time = event.time;
                    heap.push(scheduled_event);
                    break;
                }
            }
        }

        // Put the Scheduler Heap into storage.
        <Self as Store>::Scheduler::put(heap);

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
    fn scheduler_run_works() {
        new_test_ext().execute_with(|| {
            let t0 = Time::from_values(2015, 01, 01, 00, 00, 00);
            let id = H256::random();
            let mut attributes = Attributes::new(id);
            attributes.contract_deal_date = Time::from_values(2015, 01, 01, 00, 00, 00);
            attributes.contract_id = id;
            attributes.contract_role = Some(ContractRole::RPA);
            attributes.contract_type = Some(ContractType::PAM);
            attributes.counterparty_id = Some(H256::random());
            attributes.creator_id = Some(H256::random());
            attributes.currency = Some(1);
            attributes.settlement_currency = Some(1);
            attributes.day_count_convention = Some(DayCountConvention::_30E360);
            attributes.initial_exchange_date = Time::from_values(2015, 01, 02, 00, 00, 00);
            attributes.maturity_date = Time::from_values(2015, 04, 02, 00, 00, 00);
            attributes.nominal_interest_rate = Real::from(0);
            attributes.notional_principal = Real::from(1000);
            attributes.premium_discount_at_ied = Real::from(-5);
            attributes.rate_spread = Real::from(0);
            attributes.scaling_effect = None;

            <assets::Module<Test>>::mint(
                attributes.creator_id.unwrap(),
                attributes.currency.unwrap(),
                attributes.notional_principal,
            );

            let mut state = Contracts::deploy_pam(t0, attributes).unwrap();
            <Contracts as Store>::ContractStates::insert(id, state.clone());
            let event = ScheduledEvent {
                time: state.schedule[0].time,
                contract_id: id,
                index: 0,
            };
            let mut heap = MinHeap::new();
            heap.push(event);
            <Contracts as Store>::Scheduler::put(heap);

            let result = Contracts::scheduler_run(Time::from_values(2015, 01, 02, 00, 00, 05));
            let new_state = <Contracts as Store>::ContractStates::get(id);
            assert_eq!(new_state.variables.notional_principal, Real::from(1000));
            assert_eq!(new_state.variables.nominal_interest_rate, Real::from(0));
            assert_eq!(new_state.variables.accrued_interest, Real::from(0));
            let event = <Contracts as Store>::Scheduler::get().pop().unwrap();
            assert_eq!(event.time, Time::from_values(2015, 04, 02, 00, 00, 00));
            assert_eq!(event.index, 1);
        });
    }
}
