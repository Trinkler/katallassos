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
    pub fn progress_pam(
        event: Event,
        mut contract: ContractState,
    ) -> ContractResult<(ContractState, Real)> {
        // Getting t0 from the status_date attribute since they are equal.
        // (And status_date is not supposed to change)
        let t0 = contract.terms.status_date;

        match event.event_type {
            EventType::IED => {
                // Payoff Function
                let payoff = utilities::contract_role_sign(contract.terms.contract_role)
                    * Real::from(-1)
                    * (contract.terms.notional_principal + contract.terms.premium_discount_at_ied);
                // State Transition Function
                contract.states.notional_principal =
                    utilities::contract_role_sign(contract.terms.contract_role)
                        * contract.terms.notional_principal;
                if contract.terms.nominal_interest_rate == Real(None) {
                    contract.states.nominal_interest_rate = Real::from(0);
                } else {
                    contract.states.nominal_interest_rate = contract.terms.nominal_interest_rate;
                }
                if contract.terms.accrued_interest != Real(None) {
                    contract.states.accrued_interest = contract.terms.accrued_interest;
                } else if contract.terms.cycle_anchor_date_of_interest_payment != Time(None)
                    && contract.terms.cycle_anchor_date_of_interest_payment < event.time
                {
                    let y = utilities::year_fraction(
                        contract.terms.cycle_anchor_date_of_interest_payment,
                        event.time,
                        contract.terms.day_count_convention.unwrap(), // Unwraping poses no danger since day_count_convention is mandatory for the PAM contract. It will never panic.
                    );
                    contract.states.accrued_interest = y
                        * contract.states.notional_principal
                        * contract.states.nominal_interest_rate;
                } else {
                    contract.states.accrued_interest = Real::from(0);
                }
                contract.states.status_date = event.time;
                // Return the contract contract and payoff
                Ok((contract, payoff))
            }
            EventType::MD => {
                // Payoff Function
                let payoff = contract.states.notional_scaling_multiplier
                    * contract.states.notional_principal
                    + contract.states.interest_scaling_multiplier
                        * contract.states.accrued_interest
                    + contract.states.fee_accrued;
                // State Transition Function
                contract.states.notional_principal = Real::from(0);
                contract.states.accrued_interest = Real::from(0);
                contract.states.fee_accrued = Real::from(0);
                contract.states.status_date = event.time;
                // Return the contract contract and payoff
                Ok((contract, payoff))
            }
            EventType::PP => {
                // Payoff Function
                // TODO: Add the user-initiated events based on the "PPMO".
                let payoff = Real::from(0);
                // State Transition Function
                contract.states.accrued_interest = contract.states.accrued_interest
                    + utilities::year_fraction(
                        contract.states.status_date,
                        event.time,
                        contract.terms.day_count_convention.unwrap(), // This unwrap will never panic.
                    ) * contract.states.nominal_interest_rate
                        * contract.states.notional_principal;
                if contract.terms.fee_basis == Some(FeeBasis::N) {
                    contract.states.fee_accrued = contract.states.fee_accrued
                        + utilities::year_fraction(
                            contract.states.status_date,
                            event.time,
                            contract.terms.day_count_convention.unwrap(), // This unwrap will never panic.
                        ) * contract.states.notional_principal
                            * contract.terms.fee_rate;
                } else {
                    let mut t_minus = Time(None);
                    let mut t_plus = Time(None);
                    for e in contract.schedule.clone() {
                        if e.event_type == EventType::FP {
                            if e.time >= t0 {
                                t_plus = e.time;
                                break;
                            }
                            t_minus = e.time;
                        }
                    }
                    contract.states.fee_accrued = utilities::year_fraction(
                        t_minus,
                        event.time,
                        contract.terms.day_count_convention.unwrap(), // This unwrap will never panic.
                    ) / year_fraction(
                        t_minus,
                        t_plus,
                        contract.terms.day_count_convention.unwrap(), // This unwrap will never panic.
                    ) * utilities::contract_role_sign(
                        contract.terms.contract_role,
                    ) * contract.terms.fee_rate;
                }
                // TODO: Add the user-initiated events based on the "PPMO".
                contract.states.notional_principal = contract.states.notional_principal;
                contract.states.status_date = event.time;
                // Return the contract contract and payoff
                Ok((contract, payoff))
            }
            EventType::PY => {
                // Payoff Function
                let mut payoff = Real::from(0);
                if contract.terms.penalty_type == Some(PenaltyType::A) {
                    payoff = utilities::contract_role_sign(contract.terms.contract_role)
                        * contract.terms.penalty_rate;
                }
                if contract.terms.penalty_type == Some(PenaltyType::N) {
                    payoff = utilities::contract_role_sign(contract.terms.contract_role)
                        * utilities::year_fraction(
                            contract.states.status_date,
                            event.time,
                            contract.terms.day_count_convention.unwrap(), // This unwrap will never panic.
                        )
                        * contract.states.notional_principal
                        * contract.terms.penalty_rate;
                }
                if contract.terms.penalty_type == Some(PenaltyType::I) {
                    payoff = utilities::contract_role_sign(contract.terms.contract_role)
                        * utilities::year_fraction(
                            contract.states.status_date,
                            event.time,
                            contract.terms.day_count_convention.unwrap(), // This unwrap will never panic.
                        )
                        * contract.states.notional_principal
                        * Real::max(
                            Real::from(0),
                            contract.states.nominal_interest_rate
                                - <oracle::Module<T>>::oracles(
                                    contract.terms.market_object_code_rate_reset.unwrap(), //This unwrap will never panic.
                                )
                                .value,
                        );
                }
                // State Transition Function
                contract.states.accrued_interest = contract.states.accrued_interest
                    + utilities::year_fraction(
                        contract.states.status_date,
                        event.time,
                        contract.terms.day_count_convention.unwrap(), // This unwrap will never panic.
                    ) * contract.states.nominal_interest_rate
                        * contract.states.notional_principal;
                if contract.terms.fee_basis == Some(FeeBasis::N) {
                    contract.states.fee_accrued = contract.states.fee_accrued
                        + utilities::year_fraction(
                            contract.states.status_date,
                            event.time,
                            contract.terms.day_count_convention.unwrap(), // This unwrap will never panic.
                        ) * contract.states.notional_principal
                            * contract.terms.fee_rate;
                } else {
                    let mut t_minus = Time(None);
                    let mut t_plus = Time(None);
                    for e in contract.schedule.clone() {
                        if e.event_type == EventType::FP {
                            if e.time >= t0 {
                                t_plus = e.time;
                                break;
                            }
                            t_minus = e.time;
                        }
                    }
                    contract.states.fee_accrued = utilities::year_fraction(
                        t_minus,
                        event.time,
                        contract.terms.day_count_convention.unwrap(), // This unwrap will never panic.
                    ) / year_fraction(
                        t_minus,
                        t_plus,
                        contract.terms.day_count_convention.unwrap(), // This unwrap will never panic.
                    ) * utilities::contract_role_sign(
                        contract.terms.contract_role,
                    ) * contract.terms.fee_rate;
                }
                contract.states.status_date = event.time;
                // Return the contract contract and payoff
                Ok((contract, payoff))
            }
            EventType::FP => {
                // Payoff Function
                let mut payoff = Real::from(0);
                if contract.terms.fee_basis == Some(FeeBasis::A) {
                    payoff = utilities::contract_role_sign(contract.terms.contract_role)
                        * contract.terms.fee_rate;
                }
                if contract.terms.fee_basis == Some(FeeBasis::N) {
                    payoff = contract.terms.fee_rate
                        * utilities::year_fraction(
                            contract.states.status_date,
                            event.time,
                            contract.terms.day_count_convention.unwrap(), // This unwrap will never panic.
                        )
                        * contract.states.notional_principal
                        + contract.states.fee_accrued;
                }
                // State Transition Function
                contract.states.accrued_interest = contract.states.accrued_interest
                    + utilities::year_fraction(
                        contract.states.status_date,
                        event.time,
                        contract.terms.day_count_convention.unwrap(), // This unwrap will never panic.
                    ) * contract.states.nominal_interest_rate
                        * contract.states.notional_principal;
                contract.states.fee_accrued = Real::from(0);
                contract.states.status_date = event.time;
                // Return the contract contract and payoff
                Ok((contract, payoff))
            }
            EventType::PRD => {
                // Payoff Function
                let payoff = utilities::contract_role_sign(contract.terms.contract_role)
                    * Real::from(-1)
                    * (contract.terms.price_at_purchase_date
                        + contract.states.accrued_interest
                        + utilities::year_fraction(
                            contract.states.status_date,
                            event.time,
                            contract.terms.day_count_convention.unwrap(), // This unwrap will never panic.
                        ) * contract.states.nominal_interest_rate
                            * contract.states.notional_principal);
                // State Transition Function
                contract.states.accrued_interest = contract.states.accrued_interest
                    + utilities::year_fraction(
                        contract.states.status_date,
                        event.time,
                        contract.terms.day_count_convention.unwrap(), // This unwrap will never panic.
                    ) * contract.states.nominal_interest_rate
                        * contract.states.notional_principal;
                if contract.terms.fee_basis == Some(FeeBasis::N) {
                    contract.states.fee_accrued = contract.states.fee_accrued
                        + utilities::year_fraction(
                            contract.states.status_date,
                            event.time,
                            contract.terms.day_count_convention.unwrap(), // This unwrap will never panic.
                        ) * contract.states.notional_principal
                            * contract.terms.fee_rate;
                } else {
                    let mut t_minus = Time(None);
                    let mut t_plus = Time(None);
                    for e in contract.schedule.clone() {
                        if e.event_type == EventType::FP {
                            if e.time >= t0 {
                                t_plus = e.time;
                                break;
                            }
                            t_minus = e.time;
                        }
                    }
                    contract.states.fee_accrued = utilities::year_fraction(
                        t_minus,
                        event.time,
                        contract.terms.day_count_convention.unwrap(), // This unwrap will never panic.
                    ) / year_fraction(
                        t_minus,
                        t_plus,
                        contract.terms.day_count_convention.unwrap(), // This unwrap will never panic.
                    ) * utilities::contract_role_sign(
                        contract.terms.contract_role,
                    ) * contract.terms.fee_rate;
                }
                contract.states.status_date = event.time;
                // Return the contract contract and payoff
                Ok((contract, payoff))
            }
            EventType::TD => {
                // Payoff Function
                let payoff = utilities::contract_role_sign(contract.terms.contract_role)
                    * (contract.terms.price_at_termination_date
                        + contract.states.accrued_interest
                        + utilities::year_fraction(
                            contract.states.status_date,
                            event.time,
                            contract.terms.day_count_convention.unwrap(), // This unwrap will never panic.
                        ) * contract.states.nominal_interest_rate
                            * contract.states.notional_principal);
                // State Transition Function
                contract.states.notional_principal = Real::from(0);
                contract.states.accrued_interest = Real::from(0);
                contract.states.fee_accrued = Real::from(0);
                contract.states.nominal_interest_rate = Real::from(0);
                contract.states.status_date = event.time;
                // Return the contract contract and payoff
                Ok((contract, payoff))
            }
            EventType::IP => {
                // Payoff Function
                let payoff = contract.states.interest_scaling_multiplier
                    * (contract.states.accrued_interest
                        + utilities::year_fraction(
                            contract.states.status_date,
                            event.time,
                            contract.terms.day_count_convention.unwrap(), // This unwrap will never panic.
                        ) * contract.states.nominal_interest_rate
                            * contract.states.notional_principal);
                // State Transition Function
                contract.states.accrued_interest = Real::from(0);
                if contract.terms.fee_basis == Some(FeeBasis::N) {
                    contract.states.fee_accrued = contract.states.fee_accrued
                        + utilities::year_fraction(
                            contract.states.status_date,
                            event.time,
                            contract.terms.day_count_convention.unwrap(), // This unwrap will never panic.
                        ) * contract.states.notional_principal
                            * contract.terms.fee_rate;
                } else {
                    let mut t_minus = Time(None);
                    let mut t_plus = Time(None);
                    for e in contract.schedule.clone() {
                        if e.event_type == EventType::FP {
                            if e.time >= t0 {
                                t_plus = e.time;
                                break;
                            }
                            t_minus = e.time;
                        }
                    }
                    contract.states.fee_accrued = utilities::year_fraction(
                        t_minus,
                        event.time,
                        contract.terms.day_count_convention.unwrap(), // This unwrap will never panic.
                    ) / year_fraction(
                        t_minus,
                        t_plus,
                        contract.terms.day_count_convention.unwrap(), // This unwrap will never panic.
                    ) * utilities::contract_role_sign(
                        contract.terms.contract_role,
                    ) * contract.terms.fee_rate;
                }
                contract.states.status_date = event.time;
                // Return the contract contract and payoff
                Ok((contract, payoff))
            }
            EventType::IPCI => {
                // Payoff Function
                let payoff = Real::from(0);
                // State Transition Function
                let notional_principal_minus = contract.states.notional_principal; // Temporary variable.
                contract.states.notional_principal = contract.states.notional_principal
                    + contract.states.accrued_interest
                    + utilities::year_fraction(
                        contract.states.status_date,
                        event.time,
                        contract.terms.day_count_convention.unwrap(), // This unwrap will never panic.
                    ) * contract.states.notional_principal
                        * contract.states.nominal_interest_rate;
                contract.states.accrued_interest = Real::from(0);
                if contract.terms.fee_basis == Some(FeeBasis::N) {
                    contract.states.fee_accrued = contract.states.fee_accrued
                        + utilities::year_fraction(
                            contract.states.status_date,
                            event.time,
                            contract.terms.day_count_convention.unwrap(), // This unwrap will never panic.
                        ) * notional_principal_minus
                            * contract.terms.fee_rate;
                } else {
                    let mut t_minus = Time(None);
                    let mut t_plus = Time(None);
                    for e in contract.schedule.clone() {
                        if e.event_type == EventType::FP {
                            if e.time >= t0 {
                                t_plus = e.time;
                                break;
                            }
                            t_minus = e.time;
                        }
                    }
                    contract.states.fee_accrued = utilities::year_fraction(
                        t_minus,
                        event.time,
                        contract.terms.day_count_convention.unwrap(), // This unwrap will never panic.
                    ) / year_fraction(
                        t_minus,
                        t_plus,
                        contract.terms.day_count_convention.unwrap(), // This unwrap will never panic.
                    ) * utilities::contract_role_sign(
                        contract.terms.contract_role,
                    ) * contract.terms.fee_rate;
                }
                contract.states.status_date = event.time;
                // Return the contract contract and payoff
                Ok((contract, payoff))
            }
            EventType::RR => {
                // Payoff Function
                let payoff = Real::from(0);
                // State Transition Function
                contract.states.accrued_interest = contract.states.accrued_interest
                    + utilities::year_fraction(
                        contract.states.status_date,
                        event.time,
                        contract.terms.day_count_convention.unwrap(), // This unwrap will never panic.
                    ) * contract.states.nominal_interest_rate
                        * contract.states.notional_principal;
                if contract.terms.fee_basis == Some(FeeBasis::N) {
                    contract.states.fee_accrued = contract.states.fee_accrued
                        + utilities::year_fraction(
                            contract.states.status_date,
                            event.time,
                            contract.terms.day_count_convention.unwrap(), // This unwrap will never panic.
                        ) * contract.states.notional_principal
                            * contract.terms.fee_rate;
                } else {
                    let mut t_minus = Time(None);
                    let mut t_plus = Time(None);
                    for e in contract.schedule.clone() {
                        if e.event_type == EventType::FP {
                            if e.time >= t0 {
                                t_plus = e.time;
                                break;
                            }
                            t_minus = e.time;
                        }
                    }
                    contract.states.fee_accrued = utilities::year_fraction(
                        t_minus,
                        event.time,
                        contract.terms.day_count_convention.unwrap(), // This unwrap will never panic.
                    ) / year_fraction(
                        t_minus,
                        t_plus,
                        contract.terms.day_count_convention.unwrap(), // This unwrap will never panic.
                    ) * utilities::contract_role_sign(
                        contract.terms.contract_role,
                    ) * contract.terms.fee_rate;
                }
                let delta_r = Real::min(
                    Real::max(
                        <oracle::Module<T>>::oracles(
                            contract.terms.market_object_code_rate_reset.unwrap(), //This unwrap will never panic.
                        )
                        .value
                            * contract.terms.rate_multiplier
                            + contract.terms.rate_spread
                            - contract.states.nominal_interest_rate,
                        contract.terms.period_floor,
                    ),
                    contract.terms.period_cap,
                );
                contract.states.nominal_interest_rate = Real::min(
                    Real::max(
                        contract.states.nominal_interest_rate + delta_r,
                        contract.terms.life_floor,
                    ),
                    contract.terms.life_cap,
                );
                contract.states.status_date = event.time;
                // Return the contract contract and payoff
                Ok((contract, payoff))
            }
            EventType::RRF => {
                // Payoff Function
                let payoff = Real::from(0);
                // State Transition Function
                contract.states.accrued_interest = contract.states.accrued_interest
                    + utilities::year_fraction(
                        contract.states.status_date,
                        event.time,
                        contract.terms.day_count_convention.unwrap(), // This unwrap will never panic.
                    ) * contract.states.nominal_interest_rate
                        * contract.states.notional_principal;
                if contract.terms.fee_basis == Some(FeeBasis::N) {
                    contract.states.fee_accrued = contract.states.fee_accrued
                        + utilities::year_fraction(
                            contract.states.status_date,
                            event.time,
                            contract.terms.day_count_convention.unwrap(), // This unwrap will never panic.
                        ) * contract.states.notional_principal
                            * contract.terms.fee_rate;
                } else {
                    let mut t_minus = Time(None);
                    let mut t_plus = Time(None);
                    for e in contract.schedule.clone() {
                        if e.event_type == EventType::FP {
                            if e.time >= t0 {
                                t_plus = e.time;
                                break;
                            }
                            t_minus = e.time;
                        }
                    }
                    contract.states.fee_accrued = utilities::year_fraction(
                        t_minus,
                        event.time,
                        contract.terms.day_count_convention.unwrap(), // This unwrap will never panic.
                    ) / year_fraction(
                        t_minus,
                        t_plus,
                        contract.terms.day_count_convention.unwrap(), // This unwrap will never panic.
                    ) * utilities::contract_role_sign(
                        contract.terms.contract_role,
                    ) * contract.terms.fee_rate;
                }
                contract.states.nominal_interest_rate = contract.terms.next_reset_rate;
                contract.states.status_date = event.time;
                // Return the contract contract and payoff
                Ok((contract, payoff))
            }
            EventType::SC => {
                // Payoff Function
                let payoff = Real::from(0);
                // State Transition Function
                contract.states.accrued_interest = contract.states.accrued_interest
                    + utilities::year_fraction(
                        contract.states.status_date,
                        event.time,
                        contract.terms.day_count_convention.unwrap(), // This unwrap will never panic.
                    ) * contract.states.nominal_interest_rate
                        * contract.states.notional_principal;
                if contract.terms.fee_basis == Some(FeeBasis::N) {
                    contract.states.fee_accrued = contract.states.fee_accrued
                        + utilities::year_fraction(
                            contract.states.status_date,
                            event.time,
                            contract.terms.day_count_convention.unwrap(), // This unwrap will never panic.
                        ) * contract.states.notional_principal
                            * contract.terms.fee_rate;
                } else {
                    let mut t_minus = Time(None);
                    let mut t_plus = Time(None);
                    for e in contract.schedule.clone() {
                        if e.event_type == EventType::FP {
                            if e.time >= t0 {
                                t_plus = e.time;
                                break;
                            }
                            t_minus = e.time;
                        }
                    }
                    contract.states.fee_accrued = utilities::year_fraction(
                        t_minus,
                        event.time,
                        contract.terms.day_count_convention.unwrap(), // This unwrap will never panic.
                    ) / year_fraction(
                        t_minus,
                        t_plus,
                        contract.terms.day_count_convention.unwrap(), // This unwrap will never panic.
                    ) * utilities::contract_role_sign(
                        contract.terms.contract_role,
                    ) * contract.terms.fee_rate;
                }
                // Unwrap will never panic because of the lazy evaluation.
                if contract.terms.scaling_effect.is_some()
                    && (contract.terms.scaling_effect.unwrap() == ScalingEffect::_000
                        || contract.terms.scaling_effect.unwrap() == ScalingEffect::I00)
                {
                    contract.states.notional_scaling_multiplier =
                        contract.states.notional_scaling_multiplier;
                } else {
                    contract.states.notional_scaling_multiplier = (<oracle::Module<T>>::oracles(
                        contract.terms.market_object_code_rate_reset.unwrap(), //This unwrap will never panic.
                    )
                    .value
                        - contract.terms.scaling_index_at_status_date)
                        / contract.terms.scaling_index_at_status_date;
                }
                // Unwrap will never panic because of the lazy evaluation.
                if contract.terms.scaling_effect.is_some()
                    && (contract.terms.scaling_effect.unwrap() == ScalingEffect::_000
                        || contract.terms.scaling_effect.unwrap() == ScalingEffect::_0N0)
                {
                    contract.states.interest_scaling_multiplier =
                        contract.states.interest_scaling_multiplier;
                } else {
                    contract.states.interest_scaling_multiplier = (<oracle::Module<T>>::oracles(
                        contract.terms.market_object_code_rate_reset.unwrap(), //This unwrap will never panic.
                    )
                    .value
                        - contract.terms.scaling_index_at_status_date)
                        / contract.terms.scaling_index_at_status_date;
                }
                contract.states.status_date = event.time;
                // Return the contract contract and payoff
                Ok((contract, payoff))
            }
            EventType::CE => {
                // Payoff Function
                let payoff = Real::from(0);
                // State Transition Function
                contract.states.accrued_interest = contract.states.accrued_interest
                    + utilities::year_fraction(
                        contract.states.status_date,
                        event.time,
                        contract.terms.day_count_convention.unwrap(), // This unwrap will never panic.
                    ) * contract.states.nominal_interest_rate
                        * contract.states.notional_principal;
                contract.states.status_date = event.time;
                // Return the contract contract and payoff
                Ok((contract, payoff))
            }
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
                Event::new(
                    Time::from_values(2015, 01, 02, 00, 00, 00),
                    EventType::IED
                )
            );
            contract = Contracts::progress_pam(contract.schedule[0], contract)
                .unwrap()
                .0;
            assert_eq!(contract.states.notional_principal, Real::from(1000));
            assert_eq!(contract.states.nominal_interest_rate, Real::from(0));
            assert_eq!(contract.states.accrued_interest, Real::from(0));

            // Event 3 is being used, instead of the next in the sequence 1, because the
            // given test vectors don't mention event 1 (probably because it has no effect
            // on the contract).
            assert_eq!(
                contract.schedule[3],
                Event::new(
                    Time::from_values(2015, 04, 02, 00, 00, 00),
                    EventType::MD
                )
            );
            contract = Contracts::progress_pam(contract.schedule[3], contract)
                .unwrap()
                .0;
            assert_eq!(contract.states.notional_principal, Real::from(0));
            assert_eq!(contract.states.nominal_interest_rate, Real::from(0));
            assert_eq!(contract.states.accrued_interest, Real::from(0));
        });
    }
}
