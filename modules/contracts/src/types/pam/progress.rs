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

// TODO: Add support for user-initiated events.
impl<T: Trait> Module<T> {
    pub fn progress_pam(event: Event, mut contract: Contract) -> ContractResult<(Real, Contract)> {
        // Getting t0 from the status_date attribute since they are equal.
        // (And status_date is not supposed to change)
        let t0 = contract.terms.status_date;

        match event.event_type {
            EventType::IED => Ok((
                functions::pof_ied_pam(event, &contract),
                functions::stf_ied_pam(event, &t0, contract),
            )),
            EventType::MD => Ok((
                functions::pof_md_pam(event, &contract),
                functions::stf_md_pam(event, &t0, contract),
            )),
            EventType::PP => Ok((
                functions::pof_pp_pam(event, &contract),
                functions::stf_pp_pam(event, &t0, contract),
            )),
            EventType::PY => Ok((
                Self::pof_py_pam(event, &contract),
                functions::stf_py_pam(event, &t0, contract),
            )),
            EventType::FP => Ok((
                functions::pof_fp_pam(event, &contract),
                functions::stf_fp_pam(event, &t0, contract),
            )),
            EventType::PRD => Ok((
                functions::pof_prd_pam(event, &contract),
                functions::stf_prd_pam(event, &t0, contract),
            )),
            EventType::TD => Ok((
                functions::pof_td_pam(event, &contract),
                functions::stf_td_pam(event, &t0, contract),
            )),
            EventType::IP => Ok((
                functions::pof_ip_pam(event, &contract),
                functions::stf_ip_pam(event, &t0, contract),
            )),
            EventType::IPCI => Ok((
                functions::pof_ipci_pam(event, &contract),
                functions::stf_ipci_pam(event, &t0, contract),
            )),
            EventType::RR => Ok((
                functions::pof_rr_pam(event, &contract),
                Self::stf_rr_pam(event, &t0, contract),
            )),
            EventType::RRF => Ok((
                functions::pof_rrf_pam(event, &contract),
                functions::stf_rrf_pam(event, &t0, contract),
            )),
            EventType::SC => Ok((
                functions::pof_sc_pam(event, &contract),
                Self::stf_sc_pam(event, &t0, contract),
            )),
            EventType::CE => Ok((
                functions::pof_ce_pam(event, &contract),
                functions::stf_ce_pam(event, &t0, contract),
            )),
            _ => Err("Event not applicable"),
        }
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

    // This function basically just builds a genesis contract key/value store according to
    // our desired mockup.
    fn new_test_ext() -> runtime_io::TestExternalities {
        system::GenesisConfig::default()
            .build_storage::<Test>()
            .unwrap()
            .into()
    }

    #[test]
    fn progress_pam_works() {
        new_test_ext().execute_with(|| {
            let t0 = Time::from_values(2015, 01, 01, 00, 00, 00);
            let id = H256::random();
            let mut terms = Terms::new(id);
            terms.contract_deal_date = Time::from_values(2015, 01, 01, 00, 00, 00);
            terms.contract_id = id;
            terms.contract_role = Some(ContractRole::RPA);
            terms.contract_type = Some(ContractType::PAM);
            terms.counterparty_id = Some(H256::random());
            terms.creator_id = Some(H256::random());
            terms.currency = Some(1);
            terms.day_count_convention = Some(DayCountConvention::_30E360);
            terms.initial_exchange_date = Time::from_values(2015, 01, 02, 00, 00, 00);
            terms.maturity_date = Time::from_values(2015, 04, 02, 00, 00, 00);
            terms.nominal_interest_rate = Real::from(0);
            terms.notional_principal = Real::from(1000);
            terms.premium_discount_at_ied = Real::from(-5);
            terms.rate_spread = Real::from(0);
            terms.scaling_effect = None;

            let mut contract = Contracts::deploy_pam(t0, terms).unwrap();

            assert_eq!(
                contract.schedule[0],
                Event::new(Time::from_values(2015, 01, 02, 00, 00, 00), EventType::IED)
            );
            contract = Contracts::progress_pam(contract.schedule[0], contract)
                .unwrap()
                .1;
            assert_eq!(contract.states.notional_principal, Real::from(1000));
            assert_eq!(contract.states.nominal_interest_rate, Real::from(0));
            assert_eq!(contract.states.accrued_interest, Real::from(0));

            // Event 3 is being used, instead of the next in the sequence 1, because the
            // given test vectors don't mention event 1 (probably because it has no effect
            // on the contract).
            assert_eq!(
                contract.schedule[3],
                Event::new(Time::from_values(2015, 04, 02, 00, 00, 00), EventType::MD)
            );
            contract = Contracts::progress_pam(contract.schedule[3], contract)
                .unwrap()
                .1;
            assert_eq!(contract.states.notional_principal, Real::from(0));
            assert_eq!(contract.states.nominal_interest_rate, Real::from(0));
            assert_eq!(contract.states.accrued_interest, Real::from(0));
        });
    }
}
