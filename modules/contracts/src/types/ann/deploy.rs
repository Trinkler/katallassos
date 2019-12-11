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
    pub fn deploy_ann(t0: Time, input: Terms) -> ContractResult<Contract> {
        // The ContractID, necessary to create any contract.
        let mut terms = Terms::new(input.contract_id);

        // Setting the Status Date to t0, since we don't want terms to change.
        terms.status_date = t0;

        // Mandatory in all cases -> NN
        if input.contract_type.is_none()
            || input.currency.is_none()
            || input.day_count_convention.is_none()
            || input.initial_exchange_date.0.is_none()
            || input.maturity_date.0.is_none()
            || input.nominal_interest_rate.0.is_none()
            || input.notional_principal.0.is_none()
        {
            return Err("Error while initializing terms. [0]");
        } else {
            terms.contract_type = input.contract_type;
            terms.currency = input.currency;
            terms.day_count_convention = input.day_count_convention;
            terms.initial_exchange_date = input.initial_exchange_date;
            terms.maturity_date = input.maturity_date;
            terms.nominal_interest_rate = input.nominal_interest_rate;
            terms.notional_principal = input.notional_principal;
        }

        // Mandatory on stand-alone and parent contracts only and
        // not applicable on child contracts -> NN(_,_,1)
        if input.contract_deal_date.0.is_none()
            || input.contract_role.is_none()
            || input.creator_id.is_none()
        {
            return Err("Error while initializing terms. [1]");
        } else {
            terms.contract_deal_date = input.contract_deal_date;
            terms.contract_role = input.contract_role;
            terms.creator_id = input.creator_id;
        }

        // Mandatory on stand-alone and parent contracts only and
        // optional on child contracts -> NN(_,_,2)
        if input.counterparty_id.is_none() {
            return Err("Error while initializing terms. [2]");
        } else {
            terms.counterparty_id = input.counterparty_id;
        }

        // Optional in all cases -> x
        terms.accrued_interest = input.accrued_interest;
        terms.amortization_date = input.amortization_date;
        terms.business_day_convention = input.business_day_convention;
        terms.calendar = input.calendar;
        terms.capitalization_end_date = input.capitalization_end_date;
        terms.credit_line_amount = input.credit_line_amount;
        terms.cycle_anchor_date_of_interest_payment = input.cycle_anchor_date_of_interest_payment;
        terms.cycle_of_interest_payment = input.cycle_of_interest_payment;
        terms.end_of_month_convention = input.end_of_month_convention;
        terms.market_object_code = input.market_object_code;
        terms.market_value_observed = input.market_value_observed;
        terms.maturity_date = input.maturity_date;
        terms.next_principal_redemption_payment = input.next_principal_redemption_payment;
        terms.premium_discount_at_ied = input.premium_discount_at_ied;
        terms.settlement_currency = input.settlement_currency;

        // Optional on stand-alone and parent contracts only and
        // not applicable on child contracts -> x(_,_,1)
        terms.contract_performance = input.contract_performance;
        terms.delinquency_period = input.delinquency_period;
        terms.delinquency_rate = input.delinquency_rate;
        terms.grace_period = input.grace_period;
        terms.non_performing_date = input.non_performing_date;
        terms.seniority = input.seniority;

        // Group 1
        // Business rule ‘a’ applies unconditionally
        terms.fee_rate = input.fee_rate; // -> x(1,0,_)

        // Business rule ‘a’ applies if at least one of the unconditional CAs of this group is defined
        if input.fee_rate.0.is_some() {
            if input.fee_basis.is_none() {
                return Err("Error while initializing terms. [3]");
            }
            terms.fee_basis = input.fee_basis; // -> NN(1,1,_)
            terms.fee_accrued = input.fee_accrued; // -> x(1,1,_)
        }
        // At least one of the CAs with c=2 has to be defined if at least one of the unconditional CAs
        // of the group is defined
        if input.fee_rate.0.is_some() {
            if input.cycle_anchor_date_of_fee.0.is_none() && input.cycle_of_fee.is_none() {
                return Err("Error while initializing terms. [4]");
            }
            terms.cycle_anchor_date_of_fee = input.cycle_anchor_date_of_fee; // -> x(1,2,_)
            terms.cycle_of_fee = input.cycle_of_fee; // -> x(1,2,_)
        }

        // Group 3
        // Business rule ‘a’ applies unconditionally
        terms.interest_calculation_base = input.interest_calculation_base; // -> x(3,0,_)

        // Business rule ‘a’ applies provided that attribute IPCB of the group takes the value NTIED
        if input.interest_calculation_base == Some(InterestCalculationBase::NTIED) {
            if input.interest_calculation_base_amount.0.is_none() {
                return Err("Error while initializing terms. [5]");
            } else {
                terms.interest_calculation_base_amount = input.interest_calculation_base_amount;
                // -> NN(3,3,_)
            }
        }

        // At least one of the terms with c=4 in this group has to be defined provided that attribute IPCB of the group takes the value NTL
        if input.interest_calculation_base == Some(InterestCalculationBase::NTL) {
            if input
                .cycle_anchor_date_of_interest_calculation_base
                .0
                .is_none()
                && input.cycle_of_interest_calculation_base.is_none()
            {
                return Err("Error while initializing terms. [6]");
            } else {
                terms.cycle_anchor_date_of_interest_calculation_base =
                    input.cycle_anchor_date_of_interest_calculation_base;
                // -> x(3,4,_)
                terms.cycle_of_interest_calculation_base = input.cycle_of_interest_calculation_base;
                // -> x(3,4,_)
            }
        }

        // Group 4
        // At least one of the terms with c=2 in this group has to be defined provided that at least one of the unconditional terms of the group is defined (if any exists)
        if input.cycle_anchor_date_of_principal_redemption.0.is_none()
            && input.cycle_of_principal_redemption.is_none()
        {
            return Err("Error while initializing terms. [7]");
        } else if input.cycle_of_principal_redemption != input.cycle_of_rate_reset {
            return Err("Error while initializing terms. [8]");
        } else {
            terms.cycle_anchor_date_of_principal_redemption =
                input.cycle_anchor_date_of_principal_redemption;
            // -> x(4,2,_)2
            terms.cycle_of_principal_redemption = input.cycle_of_principal_redemption;
            // -> x(4,2,_)2
        }

        // Group 5
        // Business rule ‘a’ applies unconditionally
        terms.purchase_date = input.purchase_date; // -> x(5,0,1)

        // Business rule ‘a’ applies if at least one of the unconditional CAs of this group is defined
        if input.purchase_date.0.is_some() {
            if input.price_at_purchase_date.0.is_none() {
                return Err("Error while initializing terms. [9]");
            } else {
                terms.price_at_purchase_date = input.price_at_purchase_date;
                // -> NN(5,1,1)
            }
        }

        // Group 6
        // Business rule ‘a’ applies unconditionally
        terms.termination_date = input.termination_date; // -> x(6,0,1)

        // Business rule ‘a’ applies if at least one of the unconditional CAs of this group is defined
        if input.termination_date.0.is_some() {
            if input.price_at_termination_date.0.is_none() {
                return Err("Error while initializing terms. [10]");
            } else {
                terms.price_at_termination_date = input.price_at_termination_date;
                // -> NN(6,1,1)
            }
        }

        // Group 7
        // Business rule ‘a’ applies unconditionally
        terms.scaling_effect = input.scaling_effect; // -> x(7,0,_)

        // Business rule ‘a’ applies if at least one of the unconditional CAs of this group is defined
        if input.scaling_effect.is_some() {
            if input.market_object_code_of_scaling_index.is_none()
                || input.scaling_index_at_status_date.0.is_none()
            {
                return Err("Error while initializing terms. [11]");
            }
            terms.market_object_code_of_scaling_index = input.market_object_code_of_scaling_index; // -> NN(7,1,_)
            terms.scaling_index_at_status_date = input.scaling_index_at_status_date;
            // -> NN(7,1,_)
        }
        // At least one of the CAs with c=2 has to be defined if at least one of the unconditional CAs
        // of the group is defined
        if input.scaling_effect.is_some() {
            if input.cycle_anchor_date_of_scaling_index.0.is_none()
                && input.cycle_of_scaling_index.is_none()
            {
                return Err("Error while initializing terms. [12]");
            }
            terms.cycle_anchor_date_of_scaling_index = input.cycle_anchor_date_of_scaling_index; // -> x(7,2,_)
            terms.cycle_of_scaling_index = input.cycle_of_scaling_index; // -> x(7,2,_)
        }

        // Group 8
        // Business rule ‘a’ applies unconditionally
        terms.prepayment_effect = input.prepayment_effect; // -> x(8,0,_)

        // Business rule ‘a’ applies if at least one of the unconditional CAs of this group is defined
        if input.prepayment_effect.is_some() {
            terms.cycle_anchor_date_of_optionality = input.cycle_anchor_date_of_optionality; // -> x(8,1,_)
            terms.cycle_of_optionality = input.cycle_of_optionality; // -> x(8,1,_)
            terms.option_exercise_end_date = input.option_exercise_end_date; // -> x(8,1,_)
            terms.penalty_rate = input.penalty_rate; // -> x(8,1,_)
            terms.penalty_type = input.penalty_type; // -> x(8,1,_)
            terms.prepayment_period = input.prepayment_period; // -> x(8,1,1)
        }

        // Group 9
        // Business rule ‘a’ applies unconditionally
        terms.cycle_anchor_date_of_rate_reset = input.cycle_anchor_date_of_rate_reset; // -> x(9,0,_)
        terms.cycle_of_rate_reset = input.cycle_of_rate_reset; // -> x(9,0,_)

        // Business rule ‘a’ applies if at least one of the unconditional CAs of this group is defined
        if input.cycle_anchor_date_of_rate_reset.0.is_some() || input.cycle_of_rate_reset.is_some()
        {
            if input.market_object_code_rate_reset.is_none() || input.rate_spread.0.is_none() {
                return Err("Error while initializing terms. [13]");
            } else {
                terms.market_object_code_rate_reset = input.market_object_code_rate_reset; // -> NN(9,1,_)
                terms.rate_spread = input.rate_spread; // -> NN(9,1,_)
            }
            terms.fixing_days = input.fixing_days; // -> x(9,1,_)
            terms.life_cap = input.life_cap; // -> x(9,1,_)
            terms.life_floor = input.life_floor; // -> x(9,1,_)
            terms.next_reset_rate = input.next_reset_rate; // -> x(9,1,_)
            terms.period_cap = input.period_cap; // -> x(9,1,_)
            terms.period_floor = input.period_floor; // -> x(9,1,_)
            terms.rate_multiplier = input.rate_multiplier; // -> x(9,1,_)
            terms.cycle_point_of_rate_reset = input.cycle_point_of_rate_reset;
            // -> x(9,1,_)1
        }

        // Checking if the terms all have allowed values
        if terms.is_valid() == false {
            return Err("Error while initializing terms. [14]");
        }

        // Creating the schedule for all the events.
        let mut schedule: Vec<Event> = Vec::new();

        // Inital exchange date event
        let event = Event::new(terms.initial_exchange_date, EventType::IED);
        schedule.push(event);

        // Principal Redemption event
        let mut s: Time = Time(None);
        if terms.cycle_anchor_date_of_principal_redemption == Time(None)
            && terms.cycle_of_principal_redemption == None
        {
            s = Time(None);
        } else if terms.cycle_anchor_date_of_principal_redemption == Time(None) {
            s = utilities::sum_cycle(
                terms.initial_exchange_date,
                terms.cycle_of_principal_redemption,
                terms.end_of_month_convention,
            );
        } else {
            s = terms.cycle_anchor_date_of_principal_redemption;
        }

        let vec = utilities::schedule(
            s,
            terms.maturity_date,
            terms.cycle_of_principal_redemption,
            terms.end_of_month_convention,
        )?;

        // Note: The last entry in vec is supposed to not enter the schedule.
        for i in 0..vec.len() - 2 {
            let event = Event::new(vec[i], EventType::PR);
            schedule.push(event);
        }

        // Maturity date event
        let event = Event::new(terms.maturity_date, EventType::MD);
        schedule.push(event);

        // Principal prepayment event
        if terms.prepayment_effect == Some(PrepaymentEffect::N) {
        } else {
            let mut s: Time = Time(None);
            if terms.cycle_anchor_date_of_optionality == Time(None)
                && terms.cycle_of_optionality == None
            {
                s = Time(None);
            } else if terms.cycle_anchor_date_of_optionality == Time(None) {
                s = utilities::sum_cycle(
                    terms.initial_exchange_date,
                    terms.cycle_of_optionality,
                    terms.end_of_month_convention,
                );
            } else {
                s = terms.cycle_anchor_date_of_optionality;
            }

            let vec = utilities::schedule(
                s,
                terms.maturity_date,
                terms.cycle_of_optionality,
                terms.end_of_month_convention,
            )?;

            for t in vec {
                let event = Event::new(t, EventType::PP);
                schedule.push(event);
            }
        }

        // Penalty payment event
        if terms.penalty_type == Some(PenaltyType::O) {
        } else {
            for e in schedule.clone() {
                if e.event_type == EventType::PP {
                    let event = Event::new(e.time, EventType::PY);
                    schedule.push(event);
                }
            }
        }

        // Fee payment event
        if terms.fee_rate == Real(None) || terms.fee_rate == Real::from(0) {
        } else {
            let mut s: Time = Time(None);
            if terms.cycle_anchor_date_of_fee == Time(None) && terms.cycle_of_fee == None {
                s = Time(None);
            } else if terms.cycle_anchor_date_of_fee == Time(None) {
                s = utilities::sum_cycle(
                    terms.initial_exchange_date,
                    terms.cycle_of_fee,
                    terms.end_of_month_convention,
                );
            } else {
                s = terms.cycle_anchor_date_of_fee;
            }

            let vec = utilities::schedule(
                s,
                terms.maturity_date,
                terms.cycle_of_fee,
                terms.end_of_month_convention,
            )?;

            for t in vec {
                let event = Event::new(t, EventType::FP);
                schedule.push(event);
            }
        }

        // Purchase date event
        let event = Event::new(terms.purchase_date, EventType::PRD);
        schedule.push(event);

        // Termination date event
        let event = Event::new(terms.termination_date, EventType::TD);
        schedule.push(event);

        // Interest payment event
        let r = if terms.capitalization_end_date != Time(None) {
            terms.capitalization_end_date
        } else if terms.cycle_anchor_date_of_interest_payment != Time(None) {
            terms.cycle_anchor_date_of_interest_payment
        } else if terms.cycle_of_interest_payment != None {
            utilities::sum_cycle(
                terms.initial_exchange_date,
                terms.cycle_of_interest_payment,
                terms.end_of_month_convention,
            )
        } else {
            Time(None)
        };

        let s = if terms.cycle_anchor_date_of_principal_redemption == Time(None) {
            utilities::sum_cycle(
                terms.initial_exchange_date,
                terms.cycle_of_principal_redemption,
                terms.end_of_month_convention,
            )
        } else {
            terms.cycle_anchor_date_of_principal_redemption
        };

        if terms.cycle_anchor_date_of_interest_payment == Time(None)
            && terms.cycle_of_interest_payment == None
        {
        } else if terms.capitalization_end_date != Time(None)
            && utilities::sum_cycle(
                terms.capitalization_end_date,
                terms.cycle_of_principal_redemption,
                terms.end_of_month_convention,
            ) >= s
        {
        } else {
            let vec = utilities::schedule(
                r,
                s,
                terms.cycle_of_interest_payment,
                terms.end_of_month_convention,
            )?;

            for t in vec {
                if utilities::sum_cycle(
                    t,
                    terms.cycle_of_principal_redemption,
                    terms.end_of_month_convention,
                ) > s
                {
                    break;
                }
                let event = Event::new(t, EventType::IP);
                schedule.push(event);
            }
        }

        let vec = utilities::schedule(
            s,
            terms.maturity_date,
            terms.cycle_of_principal_redemption,
            terms.end_of_month_convention,
        )?;

        for t in vec {
            let event = Event::new(t, EventType::IP);
            schedule.push(event);
        }

        // Interest capitalization event
        if terms.capitalization_end_date == Time(None) {
        } else {
            let mut s: Time = Time(None);
            if terms.cycle_anchor_date_of_interest_payment == Time(None)
                && terms.cycle_of_interest_payment == None
            {
                s = Time(None);
            } else if terms.cycle_anchor_date_of_interest_payment == Time(None) {
                s = utilities::sum_cycle(
                    terms.initial_exchange_date,
                    terms.cycle_of_interest_payment,
                    terms.end_of_month_convention,
                );
            } else {
                s = terms.cycle_anchor_date_of_interest_payment;
            }

            let vec = utilities::schedule(
                s,
                terms.capitalization_end_date,
                terms.cycle_of_interest_payment,
                terms.end_of_month_convention,
            )?;

            for t in vec {
                let event = Event::new(t, EventType::IPCI);
                schedule.push(event);
            }
        }

        // Interest Calculation Base Fixing event
        if terms.interest_calculation_base != Some(InterestCalculationBase::NTL) {
        } else {
            let mut s: Time = Time(None);
            if terms.cycle_anchor_date_of_interest_calculation_base == Time(None)
                && terms.cycle_of_interest_calculation_base == None
            {
                s = Time(None);
            } else if terms.cycle_anchor_date_of_interest_calculation_base == Time(None) {
                s = utilities::sum_cycle(
                    terms.initial_exchange_date,
                    terms.cycle_of_interest_calculation_base,
                    terms.end_of_month_convention,
                );
            } else {
                s = terms.cycle_anchor_date_of_interest_calculation_base;
            };

            let vec = utilities::schedule(
                s,
                terms.maturity_date,
                terms.cycle_of_interest_calculation_base,
                terms.end_of_month_convention,
            )?;

            for t in vec {
                let event = Event::new(t, EventType::IPCB);
                schedule.push(event);
            }
        }

        // Rate reset variable event
        if terms.cycle_anchor_date_of_rate_reset == Time(None) && terms.cycle_of_rate_reset == None
        {
        } else {
            let mut s: Time = Time(None);
            if terms.cycle_anchor_date_of_rate_reset == Time(None) {
                s = utilities::sum_cycle(
                    terms.initial_exchange_date,
                    terms.cycle_of_rate_reset,
                    terms.end_of_month_convention,
                );
            } else {
                s = terms.cycle_anchor_date_of_rate_reset;
            }

            let vec = utilities::schedule(
                s,
                terms.maturity_date,
                terms.cycle_of_rate_reset,
                terms.end_of_month_convention,
            )?;

            if terms.next_reset_rate != Real(None) {
                let mut t_rry = Time(None);
                for t in vec.clone() {
                    if t > terms.status_date {
                        t_rry = t;
                        break;
                    }
                }
                for t in vec {
                    if t != t_rry {
                        let event = Event::new(t, EventType::RR);
                        schedule.push(event);
                    }
                }
            } else {
                for t in vec {
                    let event = Event::new(t, EventType::RR);
                    schedule.push(event);
                }
            }
        }

        // Rate reset fixed event
        if terms.cycle_anchor_date_of_rate_reset == Time(None) && terms.cycle_of_rate_reset == None
        {
        } else {
            let mut s: Time = Time(None);
            if terms.cycle_anchor_date_of_rate_reset == Time(None) {
                s = utilities::sum_cycle(
                    terms.initial_exchange_date,
                    terms.cycle_of_rate_reset,
                    terms.end_of_month_convention,
                );
            } else {
                s = terms.cycle_anchor_date_of_rate_reset;
            }

            let vec = utilities::schedule(
                s,
                terms.maturity_date,
                terms.cycle_of_rate_reset,
                terms.end_of_month_convention,
            )?;

            for t in vec {
                if t > terms.status_date {
                    let event = Event::new(t, EventType::RRF);
                    schedule.push(event);
                    break;
                }
            }
        }

        // Scaling index revision event
        if terms.scaling_effect == Some(ScalingEffect::_000) {
        } else {
            let mut s: Time = Time(None);
            if terms.cycle_anchor_date_of_scaling_index == Time(None)
                && terms.cycle_of_scaling_index == None
            {
                s = Time(None);
            } else if terms.cycle_anchor_date_of_scaling_index == Time(None) {
                s = utilities::sum_cycle(
                    terms.initial_exchange_date,
                    terms.cycle_of_scaling_index,
                    terms.end_of_month_convention,
                );
            } else {
                s = terms.cycle_anchor_date_of_scaling_index;
            }

            let vec = utilities::schedule(
                s,
                terms.maturity_date,
                terms.cycle_of_scaling_index,
                terms.end_of_month_convention,
            )?;

            for t in vec {
                let event = Event::new(t, EventType::SC);
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

        // Initializing the contract states
        let mut states = States::new();

        // Time At Maturity Date variable
        states.time_at_maturity_date = terms.maturity_date;

        // Notional Principal variable
        if terms.initial_exchange_date > t0 {
            states.notional_principal = Real::from(0);
        } else {
            states.notional_principal =
                utilities::contract_role_sign(terms.contract_role) * terms.notional_principal;
        }

        // Nominal Interest Rate variable
        if terms.initial_exchange_date > t0 {
            states.nominal_interest_rate = Real::from(0);
        } else {
            states.nominal_interest_rate = terms.nominal_interest_rate;
        }

        // Accrued Interest variable
        if terms.nominal_interest_rate == Real(None) {
            states.accrued_interest = Real::from(0);
        } else if terms.accrued_interest != Real(None) {
            states.accrued_interest = terms.accrued_interest;
        } else {
            let mut t_minus = Time(None);
            for e in schedule.clone() {
                if e.event_type == EventType::IP {
                    if e.time >= t0 {
                        break;
                    }
                    t_minus = e.time;
                }
            }
            states.accrued_interest =
                utilities::year_fraction(t_minus, t0, terms.day_count_convention.unwrap())
                    * states.notional_principal
                    * states.nominal_interest_rate;
        }

        // Fee Accrued variable
        if terms.fee_rate == Real(None) {
            states.fee_accrued = Real::from(0);
        } else if terms.fee_accrued != Real(None) {
            states.fee_accrued = terms.fee_accrued;
        } else if terms.fee_basis == Some(FeeBasis::N) {
            let mut t_minus = Time(None);
            for e in schedule.clone() {
                if e.event_type == EventType::FP {
                    if e.time >= t0 {
                        break;
                    }
                    t_minus = e.time;
                }
            }
            states.fee_accrued =
                utilities::year_fraction(t_minus, t0, terms.day_count_convention.unwrap())
                    * states.notional_principal
                    * terms.fee_rate;
        } else {
            let mut t_minus = Time(None);
            let mut t_plus = Time(None);
            for e in schedule.clone() {
                if e.event_type == EventType::FP {
                    if e.time >= t0 {
                        t_plus = e.time;
                        break;
                    }
                    t_minus = e.time;
                }
            }
            states.fee_accrued =
                utilities::year_fraction(t_minus, t0, terms.day_count_convention.unwrap())
                    / utilities::year_fraction(
                        t_minus,
                        t_plus,
                        terms.day_count_convention.unwrap(),
                    )
                    * terms.fee_rate;
        }

        // Notional Scaling Multiplier variable
        let temp = terms.scaling_effect.unwrap_or(ScalingEffect::_000);
        if temp == ScalingEffect::_0N0 || temp == ScalingEffect::IN0 {
            states.notional_scaling_multiplier = terms.scaling_index_at_status_date;
        } else {
            states.notional_scaling_multiplier = Real::from(1);
        }

        // Interest Scaling Multiplier variable
        let temp = terms.scaling_effect.unwrap_or(ScalingEffect::_000);
        if temp == ScalingEffect::I00 || temp == ScalingEffect::IN0 {
            states.interest_scaling_multiplier = terms.scaling_index_at_status_date;
        } else {
            states.interest_scaling_multiplier = Real::from(1);
        }

        // Contract Performance variable
        states.contract_performance = terms.contract_performance;

        // Status Date variable
        states.status_date = t0;

        // Returning the initialized Contract State
        Ok(Contract {
            terms: terms,
            states: states,
            schedule: schedule,
        })
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
    fn deploy_ann_works() {
        new_test_ext().execute_with(|| {
            // TODO: Implement test cases
        });
    }
}
