//
// Copyright 2019 by Trinkler Software AG (Switzerland).
// This file is part of Katal.
//
// Katal is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version <http://www.gnu.org/licenses/>.
//
// Katal is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//

// #![recursion_limit = "128"]

use parity_codec::{Decode, Encode};
use reals::Real;
use rstd::prelude::*;
use support::{decl_event, decl_module, decl_storage, dispatch::Result, StorageMap};
use time::{Time, UncheckedTime};

// Importing the rest of the files in this crate.
mod attributes;
mod contract_events;
mod contract_state;
mod contracts;
mod utilities;
mod variables;
use self::attributes::*;
use self::contract_events::*;
use self::contract_state::*;
use self::contracts::*;
use self::utilities::*;
use self::variables::*;

// Defines an alias for the Result type. It has the name MyResult because Substrate already uses
// the name Result for their own type Result<(), &'static str>.
type MyResult<T> = rstd::result::Result<T, &'static str>;

// This module's configuration trait.
pub trait Trait: system::Trait {
    type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
}

// This module's events.
decl_event!(
    pub enum Event<T>
    where
        AccountId = <T as system::Trait>::AccountId,
    {
        Nothing(AccountId),
    }
);

// This module's storage items.
decl_storage! {
    trait Store for Module<T: Trait> as ContractStorage {
        ContractStates: map u128 => ContractState;
    }
}

// This module's dispatchable functions.
decl_module! {
    /// The module declaration.
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        // Initializing events
        // this is needed only if you are using events in your module
        fn deposit_event<T>() = default;

        fn deploy_pam(
            origin,
            accrued_interest: Real,
            business_day_convention: Option<BusinessDayConvention>,
            calendar: Option<Calendar>,
            capitalization_end_date: Time
            // contract_deal_date: Time,
            // contract_id: u128,
            // contract_role: Option<ContractRole>,
            // contract_performance: Option<ContractPerformance>,
            // contract_type: Option<ContractType>,
            // currency: Option<u128>,
            // cycle_anchor_date_of_fee: Time,
            // cycle_anchor_date_of_interest_payment: Time,
            // cycle_anchor_date_of_optionality: Time,
            // cycle_anchor_date_of_rate_reset: Time,
            // cycle_anchor_date_of_scaling_index: Time,
            // cycle_of_fee: Option<Cycle>,
            // cycle_of_interest_payment: Option<Cycle>,
            // cycle_of_optionality: Option<Cycle>,
            // cycle_of_rate_reset: Option<Cycle>,
            // cycle_of_scaling_index: Option<Cycle>,
            // cycle_point_of_interest_payment: Option<CyclePointOfInterestPayment>,
            // cycle_point_of_rate_reset: Option<CyclePointOfRateReset>,
            // day_count_convention: Option<DayCountConvention>,
            // delinquency_period: Option<Period>,
            // delinquency_rate: Real,
            // end_of_month_convention: Option<EndOfMonthConvention>,
            // ex_dividend_date: Time,
            // fee_accrued: Real,
            // fee_basis: Option<FeeBasis>,
            // fee_rate: Real,
            // fixing_days: Option<Period>,
            // grace_period: Option<Period>,
            // initial_exchange_date: Time,
            // counterparty_id: Option<u128>,
            // creator_id: Option<u128>,
            // life_cap: Real,
            // life_floor: Real,
            // market_object_code_of_scaling_index: Option<u128>,
            // market_object_code_rate_reset: Option<u128>,
            // market_value_observed: Real,
            // maturity_date: Time,
            // next_reset_rate: Real,
            // nominal_interest_rate: Real,
            // non_performing_date: Time,
            // notional_principal: Real,
            // option_exercise_end_date: Time,
            // penalty_rate: Real,
            // penalty_type: Option<PenaltyType>,
            // period_cap: Real,
            // period_floor: Real,
            // premium_discount_at_ied: Real,
            // prepayment_effect: Option<PrepaymentEffect>,
            // prepayment_period: Option<Period>,
            // price_at_purchase_date: Real,
            // price_at_termination_date: Real,
            // purchase_date: Time,
            // rate_multiplier: Real,
            // rate_spread: Real,
            // scaling_effect: Option<ScalingEffect>,
            // scaling_index_at_status_date: Real,
            // seniority: Option<Seniority>,
            // termination_date: Time
        ) -> Result {
            Ok(())
        }

        // fn deploy_pam(
        //
        // //     // TODO: Get current time.
        // //     let t0 = Time::from_values(2019, 07, 04, 00, 00, 00);
        // //
        // //     // Get the initial contract state.
        // //     let contract_state = contracts::initialize_pam(
        // //         t0,
        // //         accrued_interest,
        // //         business_day_convention,
        // //         calendar,
        // //         capitalization_end_date,
        // //         contract_deal_date,
        // //         contract_id,
        // //         contract_role,
        // //         contract_performance,
        // //         contract_type,
        // //         currency,
        // //         cycle_anchor_date_of_fee,
        // //         cycle_anchor_date_of_interest_payment,
        // //         cycle_anchor_date_of_optionality,
        // //         cycle_anchor_date_of_rate_reset,
        // //         cycle_anchor_date_of_scaling_index,
        // //         cycle_of_fee,
        // //         cycle_of_interest_payment,
        // //         cycle_of_optionality,
        // //         cycle_of_rate_reset,
        // //         cycle_of_scaling_index,
        // //         cycle_point_of_interest_payment,
        // //         cycle_point_of_rate_reset,
        // //         day_count_convention,
        // //         delinquency_period,
        // //         delinquency_rate,
        // //         end_of_month_convention,
        // //         ex_dividend_date,
        // //         fee_accrued,
        // //         fee_basis,
        // //         fee_rate,
        // //         fixing_days,
        // //         grace_period,
        // //         initial_exchange_date,
        // //         counterparty_id,
        // //         creator_id,
        // //         life_cap,
        // //         life_floor,
        // //         market_object_code_of_scaling_index,
        // //         market_object_code_rate_reset,
        // //         market_value_observed,
        // //         maturity_date,
        // //         next_reset_rate,
        // //         nominal_interest_rate,
        // //         non_performing_date,
        // //         notional_principal,
        // //         option_exercise_end_date,
        // //         penalty_rate,
        // //         penalty_type,
        // //         period_cap,
        // //         period_floor,
        // //         premium_discount_at_ied,
        // //         prepayment_effect,
        // //         prepayment_period,
        // //         price_at_purchase_date,
        // //         price_at_termination_date,
        // //         purchase_date,
        // //         rate_multiplier,
        // //         rate_spread,
        // //         scaling_effect,
        // //         scaling_index_at_status_date,
        // //         seniority,
        // //         termination_date,
        // //     )?;
        // //
        // //     <ContractStates<T>>::insert(contract_id, contract_state);
        // //
        //     Ok(())
        // }

    }
}

// tests for this module
// #[cfg(test)]
// mod tests {
//     use super::*;
//
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
//     // For testing the module, we construct most of a mock runtime. This means
//     // first constructing a configuration type (`Test`) which `impl`s each of the
//     // configuration traits of modules we want to use.
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
//     impl Trait for Test {
//         type Event = ();
//     }
//     type ACTUS = Module<Test>;
//
//     // This function basically just builds a genesis storage key/value store according to
//     // our desired mockup.
//     fn new_test_ext() -> runtime_io::TestExternalities<Blake2Hasher> {
//         system::GenesisConfig::<Test>::default()
//             .build_storage()
//             .unwrap()
//             .0
//             .into()
//     }
//
//     #[test]
//     fn it_works_for_default_value() {
//         with_externalities(&mut new_test_ext(), || {});
//     }
// }
