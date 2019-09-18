use super::*;

// This function is called once every block and executes all events for which the time
// has come. For now it only deals with the ACTUS contracts.
impl<T: Trait> Module<T> {
    pub fn init(now: Time) -> Result {
        // Get the current Scheduler List from storage.
        let mut list = <List<T>>::get();

        // Initialize the counter.
        let mut i = 0;

        // This loop goes through all scheduled events in the List.
        while i < list.len() {
            // Check if the time has come for this event.
            if now >= list[i].time {
                // Get the contract_id and index of the scheduled event.
                let contract_id = list[i].contract_id;
                let index = list[i].index;

                // Get the state of the ACTUS contract and the corresponding
                // contract event type to be executed.
                let mut state = <actus::Contracts<T>>::get(contract_id);
                let event = state.schedule[index as usize];

                // Make the ACTUS contract progress.
                <actus::Module<T>>::progress_contract(event, state.clone())?;

                // Either substitute the current scheduled event by the next one in the
                // contract or, if there is no next scheduled event, simply remove it
                // from the list.
                if index + 1 < state.schedule.len() as u32 {
                    list[i].index += 1;
                    list[i].time = state.schedule[index as usize].time;
                } else {
                    list.swap_remove(i);
                }
            // If the time has not come for this event, increase the counter.
            } else {
                i += 1;
            }
        }

        // Put the Scheduler List into storage and update the Scheduler Counter.
        <Counter<T>>::put(list.len() as u32);
        <List<T>>::put(list);

        // Return Ok.
        Ok(())
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use primitives::{Blake2Hasher, H256};
//     use runtime_io::with_externalities;
//     use runtime_primitives::{
//         testing::{Digest, DigestItem, Header},
//         traits::{BlakeTwo256, IdentityLookup},
//         BuildStorage,
//     };
//     use support::{assert_ok, impl_outer_origin};
//
//     impl_outer_origin! {
//         pub enum Origin for Test {}
//     }
//
//     #[derive(Clone, Eq, PartialEq)]
//     pub struct Test;
//     impl system::Trait for Test {
//         type Origin = Origin;
//         type Index = u64;
//         type BlockNumber = u64;
//         type Hash = H256;
//         type Hashing = BlakeTwo256;
//         type Digest = Digest;
//         type AccountId = u64;
//         type Lookup = IdentityLookup<Self::AccountId>;
//         type Header = Header;
//         type Event = ();
//         type Log = DigestItem;
//     }
//     impl oracle::Trait for Test {}
//     impl actus::Trait for Test {}
//     impl Trait for Test {}
//     type Scheduler = Module<Test>;
//
//     fn new_test_ext() -> runtime_io::TestExternalities<Blake2Hasher> {
//         system::GenesisConfig::<Test>::default()
//             .build_storage()
//             .unwrap()
//             .0
//             .into()
//     }
//
//     #[test]
//     fn init_works() {
//         with_externalities(&mut new_test_ext(), || {
//             let t0 = Time::from_values(2015, 01, 01, 00, 00, 00);
//             let id = H256::zero();
//             let mut attributes = Attributes::new(id);
//             attributes.contract_id = id;
//             attributes.contract_type = Some(ContractType::PAM);
//             attributes.currency = Some(H256::zero());
//             attributes.day_count_convention = Some(DayCountConvention::_30E360);
//             attributes.initial_exchange_date = Time::from_values(2015, 01, 02, 00, 00, 00);
//             attributes.maturity_date = Time::from_values(2015, 04, 02, 00, 00, 00);
//             attributes.nominal_interest_rate = Real::from(0);
//             attributes.notional_principal = Real::from(1000);
//             attributes.contract_deal_date = Time::from_values(2015, 01, 01, 00, 00, 00);
//             attributes.contract_role = Some(ContractRole::RPA);
//             attributes.creator_id = Some(H256::zero());
//             attributes.counterparty_id = Some(H256::zero());
//             attributes.scaling_effect = None;
//             attributes.rate_spread = Real::from(0);
//             attributes.premium_discount_at_ied = Real::from(-5);
//
//             let mut state = Actus::initialize_pam(t0, attributes).unwrap();
//         });
//     }
// }
