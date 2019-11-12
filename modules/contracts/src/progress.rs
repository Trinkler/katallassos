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
    pub fn progress(event: ContractEvent, contract_id: H256) -> Result {
        // Getting the state.
        let mut state = <ContractStates<T>>::get(contract_id);

        // Calculating the resulting contract state.
        let mut payoff = Real::from(0);
        match state.attributes.contract_type {
            Some(ContractType::PAM) => {
                let result = Self::progress_pam(event, state)?;
                state = result.0;
                payoff = result.1;
            }
            _ => {
                Err("Contract type not supported")?;
            }
        }

        // Executing the payoff.
        // Note: not sure if those unwrap() will not panic.
        if payoff >= Real::from(0) {
            <assets::Module<T>>::transfer(
                state.attributes.counterparty_id.unwrap(),
                state.attributes.creator_id.unwrap(),
                state.attributes.settlement_currency.unwrap(),
                payoff.abs(),
            )?;
        } else {
            <assets::Module<T>>::transfer(
                state.attributes.creator_id.unwrap(),
                state.attributes.counterparty_id.unwrap(),
                state.attributes.settlement_currency.unwrap(),
                payoff.abs(),
            )?;
        }

        // TODO: Set contract performance variable to something other than `Performant`

        // Storing the contract state.
        <ContractStates<T>>::insert(contract_id, state);

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
    fn progress_works() {
        with_externalities(&mut new_test_ext(), || {
            let t0 = Time::from_values(2015, 01, 01, 00, 00, 00);
            let id = H256::random();
            let creator_id = H256::random();
            let counterparty_id = H256::random();
            let currency = 1;

            let mut attributes = Attributes::new(id);
            attributes.contract_deal_date = Time::from_values(2015, 01, 01, 00, 00, 00);
            attributes.contract_id = id;
            attributes.contract_role = Some(ContractRole::RPA);
            attributes.contract_type = Some(ContractType::PAM);
            attributes.counterparty_id = Some(counterparty_id);
            attributes.creator_id = Some(creator_id);
            attributes.currency = Some(currency);
            attributes.day_count_convention = Some(DayCountConvention::_30E360);
            attributes.initial_exchange_date = Time::from_values(2015, 01, 02, 00, 00, 00);
            attributes.maturity_date = Time::from_values(2015, 04, 02, 00, 00, 00);
            attributes.nominal_interest_rate = Real::from(0);
            attributes.notional_principal = Real::from(1000);
            attributes.premium_discount_at_ied = Real::from(-5);
            attributes.rate_spread = Real::from(0);
            attributes.scaling_effect = None;

            <assets::Module<Test>>::mint(creator_id, currency, Real::from(1000));
            <assets::Module<Test>>::mint(counterparty_id, currency, Real::from(1000));

            let mut state = Contracts::deploy_pam(t0, attributes).unwrap();
            <ContractStates<Test>>::insert(id, state.clone());

            assert_eq!(
                state.schedule[0],
                ContractEvent::new(
                    Time::from_values(2015, 01, 02, 00, 00, 00),
                    ContractEventType::IED
                )
            );
            Contracts::progress(state.schedule[0], id);
            state = <ContractStates<Test>>::get(id);
            assert_eq!(state.variables.nominal_value_1, Real::from(1000));
            assert_eq!(state.variables.nominal_rate, Real::from(0));
            assert_eq!(state.variables.nominal_accrued_1, Real::from(0));
            assert_eq!(
                <assets::AssetsBalances<Test>>::get((currency, creator_id)),
                Real::from(5)
            );
            assert_eq!(
                <assets::AssetsBalances<Test>>::get((currency, counterparty_id)),
                Real::from(1995)
            );

            // Event 2 is being used, instead of the next in the sequence 1, because the
            // given test vectors don't mention event 1 (probably because it has no effect
            // on the state).
            assert_eq!(
                state.schedule[2],
                ContractEvent::new(
                    Time::from_values(2015, 04, 02, 00, 00, 00),
                    ContractEventType::PR
                )
            );
            Contracts::progress(state.schedule[2], id);
            state = <ContractStates<Test>>::get(id);
            assert_eq!(state.variables.nominal_value_1, Real::from(0));
            assert_eq!(state.variables.nominal_rate, Real::from(0));
            assert_eq!(state.variables.nominal_accrued_1, Real::from(0));
            assert_eq!(
                <assets::AssetsBalances<Test>>::get((currency, creator_id)),
                Real::from(1005)
            );
            assert_eq!(
                <assets::AssetsBalances<Test>>::get((currency, counterparty_id)),
                Real::from(995)
            );
        });
    }
}
