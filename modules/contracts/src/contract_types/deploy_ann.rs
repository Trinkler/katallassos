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

impl<T: Trait> Module<T> {
    pub fn deploy_ann(t0: Time, input: Attributes) -> MyResult<ContractState> {
        // The ContractID, necessary to create any contract.
        let mut attributes = Attributes::new(input.contract_id);

        // Setting the Status Date to t0, since we don't want attributes to change.
        attributes.status_date = t0;

        // Mandatory in all cases -> NN
        if input.contract_type.is_none()
            || input.currency.is_none()
            || input.day_count_convention.is_none()
            || input.initial_exchange_date.0.is_none()
            || input.maturity_date.0.is_none()
            || input.nominal_interest_rate.0.is_none()
            || input.notional_principal.0.is_none()
        {
            return Err("Error while initializing attributes. [0]");
        } else {
            attributes.contract_type = input.contract_type;
            attributes.currency = input.currency;
            attributes.day_count_convention = input.day_count_convention;
            attributes.initial_exchange_date = input.initial_exchange_date;
            attributes.maturity_date = input.maturity_date;
            attributes.nominal_interest_rate = input.nominal_interest_rate;
            attributes.notional_principal = input.notional_principal;
        }

        // Mandatory on stand-alone and parent contracts only and
        // not applicable on child contracts -> NN(_,_,1)
        if input.contract_deal_date.0.is_none()
            || input.contract_role.is_none()
            || input.creator_id.is_none()
        {
            return Err("Error while initializing attributes. [1]");
        } else {
            attributes.contract_deal_date = input.contract_deal_date;
            attributes.contract_role = input.contract_role;
            attributes.creator_id = input.creator_id;
        }

        // Mandatory on stand-alone and parent contracts only and
        // optional on child contracts -> NN(_,_,2)
        if input.counterparty_id.is_none() {
            return Err("Error while initializing attributes. [2]");
        } else {
            attributes.counterparty_id = input.counterparty_id;
        }

        // Optional in all cases -> x
        attributes.accrued_interest = input.accrued_interest;
        attributes.amortization_date = input.amortization_date;
        attributes.business_day_convention = input.business_day_convention;
        attributes.calendar = input.calendar;
        attributes.capitalization_end_date = input.capitalization_end_date;
        attributes.credit_line_amount = input.credit_line_amount;
        attributes.end_of_month_convention = input.end_of_month_convention;
        attributes.market_object_code = input.market_object_code;
        attributes.market_value_observed = input.market_value_observed;
        attributes.maturity_date = input.maturity_date;
        attributes.next_principal_redemption_payment = input.next_principal_redemption_payment;
        attributes.premium_discount_at_ied = input.premium_discount_at_ied;
        attributes.settlement_currency = input.settlement_currency;

        // Optional on stand-alone and parent contracts only and
        // not applicable on child contracts -> x(_,_,1)
        attributes.contract_performance = input.contract_performance;
        attributes.delinquency_period = input.delinquency_period;
        attributes.delinquency_rate = input.delinquency_rate;
        attributes.grace_period = input.grace_period;
        attributes.non_performing_date = input.non_performing_date;
        attributes.seniority = input.seniority;

        // Group 1
        // Business rule ‘a’ applies unconditionally
        attributes.fee_rate = input.fee_rate; // -> x(1,0,_)

        // Business rule ‘a’ applies if at least one of the unconditional CAs of this group is defined
        if input.fee_rate.0.is_some() {
            if input.fee_basis.is_none() {
                return Err("Error while initializing attributes. [3]");
            }
            attributes.fee_basis = input.fee_basis; // -> NN(1,1,_)
            attributes.fee_accrued = input.fee_accrued; // -> x(1,1,_)
        }
        // At least one of the CAs with c=2 has to be defined if at least one of the unconditional CAs
        // of the group is defined
        if input.fee_rate.0.is_some() {
            if input.cycle_anchor_date_of_fee.0.is_none() && input.cycle_of_fee.is_none() {
                return Err("Error while initializing attributes. [4]");
            }
            attributes.cycle_anchor_date_of_fee = input.cycle_anchor_date_of_fee; // -> x(1,2,_)
            attributes.cycle_of_fee = input.cycle_of_fee; // -> x(1,2,_)
        }

        // Group 2
        // Business rule ‘a’ applies unconditionally
        attributes.cycle_anchor_date_of_interest_payment =
            input.cycle_anchor_date_of_interest_payment; // -> x(2,0,_)
        attributes.cycle_of_interest_payment = input.cycle_of_interest_payment; // -> x(2,0,_)

        // Business rule ‘a’ applies if at least one of the unconditional CAs of this group is defined
        if input.cycle_anchor_date_of_interest_payment.0.is_some()
            || input.cycle_of_interest_payment.is_some()
        {
            if input.cycle_point_of_interest_payment == Some(CyclePointOfInterestPayment::B)
                && input.cycle_point_of_rate_reset != Some(CyclePointOfRateReset::B)
            {
                return Err("Error while initializing attributes. [5]");
            }
            attributes.cycle_point_of_interest_payment = input.cycle_point_of_interest_payment;
            // -> x(2,1,_)1
        }

        // Group 5
        // Business rule ‘a’ applies unconditionally
        attributes.purchase_date = input.purchase_date; // -> x(5,0,1)

        // Business rule ‘a’ applies if at least one of the unconditional CAs of this group is defined
        if input.purchase_date.0.is_some() {
            if input.price_at_purchase_date.0.is_none() {
                return Err("Error while initializing attributes. [6]");
            } else {
                attributes.price_at_purchase_date = input.price_at_purchase_date;
                // -> NN(5,1,1)
            }
        }

        // Group 6
        // Business rule ‘a’ applies unconditionally
        attributes.termination_date = input.termination_date; // -> x(6,0,1)

        // Business rule ‘a’ applies if at least one of the unconditional CAs of this group is defined
        if input.termination_date.0.is_some() {
            if input.price_at_termination_date.0.is_none() {
                return Err("Error while initializing attributes. [7]");
            } else {
                attributes.price_at_termination_date = input.price_at_termination_date;
                // -> NN(6,1,1)
            }
        }

        // Group 7
        // Business rule ‘a’ applies unconditionally
        attributes.scaling_effect = input.scaling_effect; // -> x(7,0,_)

        // Business rule ‘a’ applies if at least one of the unconditional CAs of this group is defined
        if input.scaling_effect.is_some() {
            if input.market_object_code_of_scaling_index.is_none()
                || input.scaling_index_at_status_date.0.is_none()
            {
                return Err("Error while initializing attributes. [8]");
            }
            attributes.market_object_code_of_scaling_index =
                input.market_object_code_of_scaling_index; // -> NN(7,1,_)
            attributes.scaling_index_at_status_date = input.scaling_index_at_status_date;
            // -> NN(7,1,_)
        }
        // At least one of the CAs with c=2 has to be defined if at least one of the unconditional CAs
        // of the group is defined
        if input.scaling_effect.is_some() {
            if input.cycle_anchor_date_of_scaling_index.0.is_none()
                && input.cycle_of_scaling_index.is_none()
            {
                return Err("Error while initializing attributes. [9]");
            }
            attributes.cycle_anchor_date_of_scaling_index =
                input.cycle_anchor_date_of_scaling_index; // -> x(7,2,_)
            attributes.cycle_of_scaling_index = input.cycle_of_scaling_index; // -> x(7,2,_)
        }

        // Group 8
        // Business rule ‘a’ applies unconditionally
        attributes.prepayment_effect = input.prepayment_effect; // -> x(8,0,_)

        // Business rule ‘a’ applies if at least one of the unconditional CAs of this group is defined
        if input.prepayment_effect.is_some() {
            attributes.cycle_anchor_date_of_optionality = input.cycle_anchor_date_of_optionality; // -> x(8,1,_)
            attributes.cycle_of_optionality = input.cycle_of_optionality; // -> x(8,1,_)
            attributes.option_exercise_end_date = input.option_exercise_end_date; // -> x(8,1,_)
            attributes.penalty_rate = input.penalty_rate; // -> x(8,1,_)
            attributes.penalty_type = input.penalty_type; // -> x(8,1,_)
            attributes.prepayment_period = input.prepayment_period; // -> x(8,1,1)
        }

        // Group 9
        // Business rule ‘a’ applies unconditionally
        attributes.cycle_anchor_date_of_rate_reset = input.cycle_anchor_date_of_rate_reset; // -> x(9,0,_)
        attributes.cycle_of_rate_reset = input.cycle_of_rate_reset; // -> x(9,0,_)

        // Business rule ‘a’ applies if at least one of the unconditional CAs of this group is defined
        if input.cycle_anchor_date_of_rate_reset.0.is_some() || input.cycle_of_rate_reset.is_some()
        {
            if input.market_object_code_rate_reset.is_none() || input.rate_spread.0.is_none() {
                return Err("Error while initializing attributes. [10]");
            } else {
                attributes.market_object_code_rate_reset = input.market_object_code_rate_reset; // -> NN(9,1,_)
                attributes.rate_spread = input.rate_spread; // -> NN(9,1,_)
            }
            attributes.fixing_days = input.fixing_days; // -> x(9,1,_)
            attributes.life_cap = input.life_cap; // -> x(9,1,_)
            attributes.life_floor = input.life_floor; // -> x(9,1,_)
            attributes.next_reset_rate = input.next_reset_rate; // -> x(9,1,_)
            attributes.period_cap = input.period_cap; // -> x(9,1,_)
            attributes.period_floor = input.period_floor; // -> x(9,1,_)
            attributes.rate_multiplier = input.rate_multiplier; // -> x(9,1,_)
            attributes.cycle_point_of_rate_reset = input.cycle_point_of_rate_reset;
            // -> x(9,1,_)1
        }

        // Checking if the attributes all have allowed values
        if attributes.is_valid() == false {
            return Err("Error while initializing attributes. [11]");
        }

        // Creating the schedule for all the events.
        let mut schedule: Vec<ContractEvent> = Vec::new();

        // Inital exchange date event
        let event = ContractEvent::new(attributes.initial_exchange_date, ContractEventType::IED);
        schedule.push(event);

        // Maturity date event
        let event = ContractEvent::new(attributes.maturity_date, ContractEventType::MD);
        schedule.push(event);

        // Principal prepayment event
        if attributes.prepayment_effect == Some(PrepaymentEffect::N) {
        } else {
            let mut s: Time = Time(None);
            if attributes.cycle_anchor_date_of_optionality == Time(None)
                && attributes.cycle_of_optionality == None
            {
                s = Time(None);
            } else if attributes.cycle_anchor_date_of_optionality == Time(None) {
                s = utilities::sum_cycle(
                    attributes.initial_exchange_date,
                    attributes.cycle_of_optionality,
                    attributes.end_of_month_convention,
                );
            } else {
                s = attributes.cycle_anchor_date_of_optionality;
            }

            let vec = utilities::schedule(
                s,
                attributes.maturity_date,
                attributes.cycle_of_optionality,
                attributes.end_of_month_convention,
            )?;

            for t in vec {
                let event = ContractEvent::new(t, ContractEventType::PP);
                schedule.push(event);
            }
        }

        // Penalty payment event
        if attributes.penalty_type == Some(PenaltyType::O) {
        } else {
            for e in schedule.clone() {
                if e.event_type == ContractEventType::PP {
                    let event = ContractEvent::new(e.time, ContractEventType::PY);
                    schedule.push(event);
                }
            }
        }

        // Fee payment event
        if attributes.fee_rate == Real(None) || attributes.fee_rate == Real::from(0) {
        } else {
            let mut s: Time = Time(None);
            if attributes.cycle_anchor_date_of_fee == Time(None) && attributes.cycle_of_fee == None
            {
                s = Time(None);
            } else if attributes.cycle_anchor_date_of_fee == Time(None) {
                s = utilities::sum_cycle(
                    attributes.initial_exchange_date,
                    attributes.cycle_of_fee,
                    attributes.end_of_month_convention,
                );
            } else {
                s = attributes.cycle_anchor_date_of_fee;
            }

            let vec = utilities::schedule(
                s,
                attributes.maturity_date,
                attributes.cycle_of_fee,
                attributes.end_of_month_convention,
            )?;

            for t in vec {
                let event = ContractEvent::new(t, ContractEventType::FP);
                schedule.push(event);
            }
        }

        // Purchase date event
        let event = ContractEvent::new(attributes.purchase_date, ContractEventType::PRD);
        schedule.push(event);

        // Termination date event
        let event = ContractEvent::new(attributes.termination_date, ContractEventType::TD);
        schedule.push(event);

        // Interest payment event
        if attributes.nominal_interest_rate == Real(None) {
        } else {
            let mut s: Time = Time(None);
            if attributes.cycle_anchor_date_of_interest_payment == Time(None)
                && attributes.cycle_of_interest_payment == None
            {
                s = Time(None);
            } else if attributes.capitalization_end_date != Time(None) {
                s = attributes.capitalization_end_date;
            } else if attributes.cycle_anchor_date_of_interest_payment == Time(None) {
                s = utilities::sum_cycle(
                    attributes.initial_exchange_date,
                    attributes.cycle_of_interest_payment,
                    attributes.end_of_month_convention,
                );
            } else {
                s = attributes.cycle_anchor_date_of_interest_payment;
            }

            let vec = utilities::schedule(
                s,
                attributes.maturity_date,
                attributes.cycle_of_interest_payment,
                attributes.end_of_month_convention,
            )?;

            for t in vec {
                let event = ContractEvent::new(t, ContractEventType::IP);
                schedule.push(event);
            }
        }

        // Interest capitalization event
        if attributes.capitalization_end_date == Time(None) {
        } else {
            let mut s: Time = Time(None);
            if attributes.cycle_anchor_date_of_interest_payment == Time(None)
                && attributes.cycle_of_interest_payment == None
            {
                s = Time(None);
            } else if attributes.cycle_anchor_date_of_interest_payment == Time(None) {
                s = utilities::sum_cycle(
                    attributes.initial_exchange_date,
                    attributes.cycle_of_interest_payment,
                    attributes.end_of_month_convention,
                );
            } else {
                s = attributes.cycle_anchor_date_of_interest_payment;
            }

            let vec = utilities::schedule(
                s,
                attributes.capitalization_end_date,
                attributes.cycle_of_interest_payment,
                attributes.end_of_month_convention,
            )?;

            for t in vec {
                let event = ContractEvent::new(t, ContractEventType::IPCI);
                schedule.push(event);
            }
        }

        // Rate reset variable event
        if attributes.cycle_anchor_date_of_rate_reset == Time(None)
            && attributes.cycle_of_rate_reset == None
        {
        } else {
            let mut s: Time = Time(None);
            if attributes.cycle_anchor_date_of_rate_reset == Time(None) {
                s = utilities::sum_cycle(
                    attributes.initial_exchange_date,
                    attributes.cycle_of_rate_reset,
                    attributes.end_of_month_convention,
                );
            } else {
                s = attributes.cycle_anchor_date_of_rate_reset;
            }

            let vec = utilities::schedule(
                s,
                attributes.maturity_date,
                attributes.cycle_of_rate_reset,
                attributes.end_of_month_convention,
            )?;

            if attributes.next_reset_rate != Real(None) {
                let mut t_rry = Time(None);
                for t in vec.clone() {
                    if t > attributes.status_date {
                        t_rry = t;
                        break;
                    }
                }
                for t in vec {
                    if t != t_rry {
                        let event = ContractEvent::new(t, ContractEventType::RR);
                        schedule.push(event);
                    }
                }
            } else {
                for t in vec {
                    let event = ContractEvent::new(t, ContractEventType::RR);
                    schedule.push(event);
                }
            }
        }

        // Rate reset fixed event
        if attributes.cycle_anchor_date_of_rate_reset == Time(None)
            && attributes.cycle_of_rate_reset == None
        {
        } else {
            let mut s: Time = Time(None);
            if attributes.cycle_anchor_date_of_rate_reset == Time(None) {
                s = utilities::sum_cycle(
                    attributes.initial_exchange_date,
                    attributes.cycle_of_rate_reset,
                    attributes.end_of_month_convention,
                );
            } else {
                s = attributes.cycle_anchor_date_of_rate_reset;
            }

            let vec = utilities::schedule(
                s,
                attributes.maturity_date,
                attributes.cycle_of_rate_reset,
                attributes.end_of_month_convention,
            )?;

            for t in vec {
                if t > attributes.status_date {
                    let event = ContractEvent::new(t, ContractEventType::RRF);
                    schedule.push(event);
                    break;
                }
            }
        }

        // Scaling index revision event
        if attributes.scaling_effect == Some(ScalingEffect::_000) {
        } else {
            let mut s: Time = Time(None);
            if attributes.cycle_anchor_date_of_scaling_index == Time(None)
                && attributes.cycle_of_scaling_index == None
            {
                s = Time(None);
            } else if attributes.cycle_anchor_date_of_scaling_index == Time(None) {
                s = utilities::sum_cycle(
                    attributes.initial_exchange_date,
                    attributes.cycle_of_scaling_index,
                    attributes.end_of_month_convention,
                );
            } else {
                s = attributes.cycle_anchor_date_of_scaling_index;
            }

            let vec = utilities::schedule(
                s,
                attributes.maturity_date,
                attributes.cycle_of_scaling_index,
                attributes.end_of_month_convention,
            )?;

            for t in vec {
                let event = ContractEvent::new(t, ContractEventType::SC);
                schedule.push(event);
            }
        }

        // Credit event (TODO)

        // Remove any events with Time == None
        // Note: The unusual control flow is because we want to use the swap_remove method,
        // which has O(1) complexity but requires a more complex solution to work.
        let mut i = 0;
        while i < schedule.len() {
            if schedule[i].time == Time(None) {
                schedule.swap_remove(i);
            } else {
                i += 1;
            }
        }

        // Ordering the schedule
        schedule.sort_unstable();

        // Initializing the variables
        let mut variables = Variables::new();

        // Time At Maturity Date variable
        variables.time_at_maturity_date = attributes.maturity_date;

        // Notional Principal variable
        if attributes.initial_exchange_date > t0 {
            variables.notional_principal = Real::from(0);
        } else {
            variables.notional_principal = utilities::contract_role_sign(attributes.contract_role)
                * attributes.notional_principal;
        }

        // Nominal Interest Rate variable
        if attributes.initial_exchange_date > t0 {
            variables.nominal_interest_rate = Real::from(0);
        } else {
            variables.nominal_interest_rate = attributes.nominal_interest_rate;
        }

        // Accrued Interest variable
        if attributes.nominal_interest_rate == Real(None) {
            variables.accrued_interest = Real::from(0);
        } else if attributes.accrued_interest != Real(None) {
            variables.accrued_interest = attributes.accrued_interest;
        } else {
            let mut t_minus = Time(None);
            for e in schedule.clone() {
                if e.event_type == ContractEventType::IP {
                    if e.time >= t0 {
                        break;
                    }
                    t_minus = e.time;
                }
            }
            variables.accrued_interest =
                utilities::year_fraction(t_minus, t0, attributes.day_count_convention.unwrap())
                    * variables.notional_principal
                    * variables.nominal_interest_rate;
        }

        // Fee Accrued variable
        if attributes.fee_rate == Real(None) {
            variables.fee_accrued = Real::from(0);
        } else if attributes.fee_accrued != Real(None) {
            variables.fee_accrued = attributes.fee_accrued;
        } else if attributes.fee_basis == Some(FeeBasis::N) {
            let mut t_minus = Time(None);
            for e in schedule.clone() {
                if e.event_type == ContractEventType::FP {
                    if e.time >= t0 {
                        break;
                    }
                    t_minus = e.time;
                }
            }
            variables.fee_accrued =
                utilities::year_fraction(t_minus, t0, attributes.day_count_convention.unwrap())
                    * variables.notional_principal
                    * attributes.fee_rate;
        } else {
            let mut t_minus = Time(None);
            let mut t_plus = Time(None);
            for e in schedule.clone() {
                if e.event_type == ContractEventType::FP {
                    if e.time >= t0 {
                        t_plus = e.time;
                        break;
                    }
                    t_minus = e.time;
                }
            }
            variables.fee_accrued =
                utilities::year_fraction(t_minus, t0, attributes.day_count_convention.unwrap())
                    / utilities::year_fraction(
                        t_minus,
                        t_plus,
                        attributes.day_count_convention.unwrap(),
                    )
                    * attributes.fee_rate;
        }

        // Notional Scaling Multiplier variable
        let temp = attributes.scaling_effect.unwrap_or(ScalingEffect::_000);
        if temp == ScalingEffect::_0N0 || temp == ScalingEffect::IN0 {
            variables.notional_scaling_multiplier = attributes.scaling_index_at_status_date;
        } else {
            variables.notional_scaling_multiplier = Real::from(1);
        }

        // Interest Scaling Multiplier variable
        let temp = attributes.scaling_effect.unwrap_or(ScalingEffect::_000);
        if temp == ScalingEffect::I00 || temp == ScalingEffect::IN0 {
            variables.interest_scaling_multiplier = attributes.scaling_index_at_status_date;
        } else {
            variables.interest_scaling_multiplier = Real::from(1);
        }

        // Contract Performance variable
        variables.contract_performance = attributes.contract_performance;

        // Status Date variable
        variables.status_date = t0;

        // Returning the initialized Contract State
        Ok(ContractState {
            attributes: attributes,
            variables: variables,
            schedule: schedule,
        })
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
    fn deploy_pam_works() {
        with_externalities(&mut new_test_ext(), || {
            // Tries to start a contract with the wrong attributes.
            let t0 = Time::from_values(1969, 07, 20, 20, 17, 00);
            let id = H256::random();
            let mut attributes = Attributes::new(id);
            let result = Contracts::deploy_pam(t0, attributes.clone());
            assert!(result.is_err());

            // Starts a PAM contract with the wrong attributes.
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
            let result = Contracts::deploy_pam(t0, attributes.clone());
            assert!(result.is_err());

            // Starts a PAM contract with the right attributes.
            attributes.scaling_effect = None;
            let result = Contracts::deploy_pam(t0, attributes.clone());
            assert!(result.is_ok());
        });
    }
}
