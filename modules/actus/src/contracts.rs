use super::*;

pub fn initialize_pam(
    t0: Time,
    accrued_interest: Real,
    business_day_convention: Option<BusinessDayConvention>,
    calendar: Option<Calendar>,
    capitalization_end_date: Time,
    contract_deal_date: Time,
    contract_id: u128,
    contract_role: Option<ContractRole>,
    contract_status: Option<ContractStatus>,
    contract_type: Option<ContractType>,
    currency: Option<u128>,
    cycle_anchor_date_of_fee: Time,
    cycle_anchor_date_of_interest_payment: Time,
    cycle_anchor_date_of_optionality: Time,
    cycle_anchor_date_of_rate_reset: Time,
    cycle_anchor_date_of_scaling_index: Time,
    cycle_of_fee: Option<Cycle>,
    cycle_of_interest_payment: Option<Cycle>,
    cycle_of_optionality: Option<Cycle>,
    cycle_of_rate_reset: Option<Cycle>,
    cycle_of_scaling_index: Option<Cycle>,
    cycle_point_of_interest_payment: Option<CyclePointOfInterestPayment>,
    cycle_point_of_rate_reset: Option<CyclePointOfRateReset>,
    day_count_convention: Option<DayCountConvention>,
    delinquency_period: Option<Period>,
    delinquency_rate: Real,
    end_of_month_convention: Option<EndOfMonthConvention>,
    ex_dividend_date: Time,
    fee_accrued: Real,
    fee_basis: Option<FeeBasis>,
    fee_rate: Real,
    fixing_days: Option<Period>,
    grace_period: Option<Period>,
    initial_exchange_date: Time,
    legal_entity_id_counterparty: Option<u128>,
    legal_entity_id_record_creator: Option<u128>,
    life_cap: Real,
    life_floor: Real,
    market_object_code_of_scaling_index: Option<u128>,
    market_object_code_rate_reset: Option<u128>,
    market_value_observed: Real,
    maturity_date: Time,
    next_reset_rate: Real,
    nominal_interest_rate: Real,
    non_performing_date: Time,
    notional_principal: Real,
    option_exercise_end_date: Time,
    penalty_rate: Real,
    penalty_type: Option<PenaltyType>,
    period_cap: Real,
    period_floor: Real,
    premium_discount_at_ied: Real,
    prepayment_effect: Option<PrepaymentEffect>,
    prepayment_period: Option<Period>,
    price_at_purchase_date: Real,
    price_at_termination_date: Real,
    purchase_date: Time,
    rate_multiplier: Real,
    rate_spread: Real,
    scaling_effect: Option<ScalingEffect>,
    scaling_index_at_status_date: Real,
    seniority: Option<Seniority>,
    status_date: Time,
    termination_date: Time,
) -> MyResult<ContractState> {
    // The ContractID, necessary to create any contract
    let mut attributes = Attributes::new(contract_id);

    // Mandatory in all cases -> NN
    if contract_type.is_none()
        || currency.is_none()
        || day_count_convention.is_none()
        || initial_exchange_date.0.is_none()
        || maturity_date.0.is_none()
        || nominal_interest_rate.0.is_none()
        || notional_principal.0.is_none()
    {
        return Err("Error while initializing attributes");
    } else {
        attributes.contract_type = contract_type;
        attributes.currency = currency;
        attributes.day_count_convention = day_count_convention;
        attributes.initial_exchange_date = initial_exchange_date;
        attributes.maturity_date = maturity_date;
        attributes.nominal_interest_rate = nominal_interest_rate;
        attributes.notional_principal = notional_principal;
    }

    // Mandatory on stand-alone and parent contracts only and
    // not applicable on child contracts -> NN(_,_,1)
    // TODO: check for parent/child relationship
    if contract_deal_date.0.is_none()
        || contract_role.is_none()
        || legal_entity_id_record_creator.is_none()
        || status_date.0.is_none()
    {
        return Err("Error while initializing attributes");
    } else {
        attributes.contract_deal_date = contract_deal_date;
        attributes.contract_role = contract_role;
        attributes.legal_entity_id_record_creator = legal_entity_id_record_creator;
        attributes.status_date = status_date; // What's the relation of this to t0?
    }

    // Mandatory on stand-alone and parent contracts only and
    // optional on child contracts -> NN(_,_,2)
    // TODO: check for parent/child relationship
    if legal_entity_id_counterparty.is_none() {
        return Err("Error while initializing attributes");
    } else {
        attributes.legal_entity_id_counterparty = legal_entity_id_counterparty;
    }

    // Optional in all cases -> x
    attributes.accrued_interest = accrued_interest;
    attributes.business_day_convention = business_day_convention;
    attributes.calendar = calendar;
    attributes.capitalization_end_date = capitalization_end_date;
    attributes.end_of_month_convention = end_of_month_convention;
    attributes.market_value_observed = market_value_observed;
    attributes.premium_discount_at_ied = premium_discount_at_ied;

    // Optional on stand-alone and parent contracts only and
    // not applicable on child contracts -> x(_,_,1)
    // TODO: check for parent/child relationship
    attributes.contract_status = contract_status;
    attributes.delinquency_period = delinquency_period;
    attributes.delinquency_rate = delinquency_rate;
    attributes.grace_period = grace_period;
    attributes.non_performing_date = non_performing_date;
    attributes.seniority = seniority;

    // Group 1
    // Business rule ‘a’ applies unconditionally
    attributes.fee_rate = fee_rate; // -> x(1,0,_)

    // Business rule ‘a’ applies if at least one of the unconditional CAs of this group is defined
    if fee_rate.0.is_some() {
        if fee_basis.is_none() {
            return Err("Error while initializing attributes");
        }
        attributes.fee_basis = fee_basis; // -> NN(1,1,_)
        attributes.fee_accrued = fee_accrued; // -> x(1,1,_)
    }
    // At least one of the CAs with c=2 has to be defined if at least one of the unconditional CAs
    // of the group is defined
    if fee_rate.0.is_some() {
        if cycle_anchor_date_of_fee.0.is_none() && cycle_of_fee.is_none() {
            return Err("Error while initializing attributes");
        }
        attributes.cycle_anchor_date_of_fee = cycle_anchor_date_of_fee; // -> x(1,2,_)
        attributes.cycle_of_fee = cycle_of_fee; // -> x(1,2,_)
    }

    // Group 2
    // Business rule ‘a’ applies unconditionally
    attributes.cycle_anchor_date_of_interest_payment = cycle_anchor_date_of_interest_payment; // -> x(2,0,_)
    attributes.cycle_of_interest_payment = cycle_of_interest_payment; // -> x(2,0,_)

    // Business rule ‘a’ applies if at least one of the unconditional CAs of this group is defined
    if cycle_anchor_date_of_interest_payment.0.is_some() || cycle_of_interest_payment.is_some() {
        if cycle_point_of_interest_payment == Some(CyclePointOfInterestPayment::B)
            && cycle_point_of_rate_reset != Some(CyclePointOfRateReset::B)
        {
            return Err("Error while initializing attributes");
        }
        attributes.cycle_point_of_interest_payment = cycle_point_of_interest_payment; // -> x(2,1,_)1
    }

    // Group 5
    // Business rule ‘a’ applies unconditionally
    // TODO: check for parent/child relationship
    attributes.purchase_date = purchase_date; // -> x(5,0,1)

    // Business rule ‘a’ applies if at least one of the unconditional CAs of this group is defined
    if purchase_date.0.is_some() {
        if price_at_purchase_date.0.is_none() {
            return Err("Error while initializing attributes");
        } else {
            // TODO: check for parent/child relationship
            attributes.price_at_purchase_date = price_at_purchase_date; // -> NN(5,1,1)
        }
    }

    // Group 6
    // Business rule ‘a’ applies unconditionally
    // TODO: check for parent/child relationship
    attributes.termination_date = termination_date; // -> x(6,0,1)

    // Business rule ‘a’ applies if at least one of the unconditional CAs of this group is defined
    if termination_date.0.is_some() {
        if price_at_termination_date.0.is_none() {
            return Err("Error while initializing attributes");
        } else {
            // TODO: check for parent/child relationship
            attributes.price_at_termination_date = price_at_termination_date; // -> NN(6,1,1)
        }
    }

    // Group 7
    // Business rule ‘a’ applies unconditionally
    attributes.scaling_effect = scaling_effect; // -> x(7,0,_)

    // Business rule ‘a’ applies if at least one of the unconditional CAs of this group is defined
    if scaling_effect.is_some() {
        if market_object_code_of_scaling_index.is_none() || scaling_index_at_status_date.0.is_none()
        {
            return Err("Error while initializing attributes");
        }
        attributes.market_object_code_of_scaling_index = market_object_code_of_scaling_index; // -> NN(7,1,_)
        attributes.scaling_index_at_status_date = scaling_index_at_status_date; // -> NN(7,1,_)
    }
    // At least one of the CAs with c=2 has to be defined if at least one of the unconditional CAs
    // of the group is defined
    if scaling_effect.is_some() {
        if cycle_anchor_date_of_scaling_index.0.is_none() && cycle_of_scaling_index.is_none() {
            return Err("Error while initializing attributes");
        }
        attributes.cycle_anchor_date_of_scaling_index = cycle_anchor_date_of_scaling_index; // -> x(7,2,_)
        attributes.cycle_of_scaling_index = cycle_of_scaling_index; // -> x(7,2,_)
    }

    // Group 8
    // Business rule ‘a’ applies unconditionally
    attributes.prepayment_effect = prepayment_effect; // -> x(8,0,_)

    // Business rule ‘a’ applies if at least one of the unconditional CAs of this group is defined
    if prepayment_effect.is_some() {
        attributes.cycle_anchor_date_of_optionality = cycle_anchor_date_of_optionality; // -> x(8,1,_)
        attributes.cycle_of_optionality = cycle_of_optionality; // -> x(8,1,_)
        attributes.option_exercise_end_date = option_exercise_end_date; // -> x(8,1,_)
        attributes.penalty_rate = penalty_rate; // -> x(8,1,_)
        attributes.penalty_type = penalty_type; // -> x(8,1,_)

        // TODO: check for parent/child relationship
        attributes.prepayment_period = prepayment_period; // -> x(8,1,1)
    }

    // Group 9
    // Business rule ‘a’ applies unconditionally
    attributes.cycle_anchor_date_of_rate_reset = cycle_anchor_date_of_rate_reset; // -> x(9,0,_)
    attributes.cycle_of_rate_reset = cycle_of_rate_reset; // -> x(9,0,_)

    // Business rule ‘a’ applies if at least one of the unconditional CAs of this group is defined
    if cycle_anchor_date_of_rate_reset.0.is_some() || cycle_of_rate_reset.is_some() {
        if market_object_code_rate_reset.is_none() || rate_spread.0.is_none() {
            return Err("Error while initializing attributes");
        } else {
            attributes.market_object_code_rate_reset = market_object_code_rate_reset; // -> NN(9,1,_)
            attributes.rate_spread = rate_spread; // -> NN(9,1,_)
        }
        attributes.fixing_days = fixing_days; // -> x(9,1,_)
        attributes.life_cap = life_cap; // -> x(9,1,_)
        attributes.life_floor = life_floor; // -> x(9,1,_)
        attributes.next_reset_rate = next_reset_rate; // -> x(9,1,_)
        attributes.period_cap = period_cap; // -> x(9,1,_)
        attributes.period_floor = period_floor; // -> x(9,1,_)
        attributes.rate_multiplier = rate_multiplier; // -> x(9,1,_)
        attributes.cycle_point_of_rate_reset = cycle_point_of_rate_reset; // -> x(9,1,_)1
    }

    // Creating the schedule for all the events.
    let mut schedule: Vec<ContractEvent> = Vec::new();

    // Inital exchange date event
    let event = ContractEvent::new(initial_exchange_date, ContractEventType::IED);
    schedule.push(event);

    // Principal redemption event
    let event = ContractEvent::new(maturity_date, ContractEventType::PR);
    schedule.push(event);

    // Principal prepayment event
    // TODO: It can't be properly implemented now. Tech specs are ambiguous.
    if prepayment_effect == Some(PrepaymentEffect::N) {
    } else {
        let mut s: Time = Time(None);
        if cycle_anchor_date_of_optionality == Time(None) && cycle_of_optionality == None {
            s = Time(None);
        } else if cycle_anchor_date_of_optionality == Time(None) {
            s = utilities::sum_cycle(
                initial_exchange_date,
                cycle_of_optionality,
                end_of_month_convention,
            );
        } else {
            s = cycle_anchor_date_of_optionality;
        }

        let vec = utilities::schedule(
            s,
            maturity_date,
            cycle_of_optionality,
            end_of_month_convention,
        )?;

        for t in vec {
            let event = ContractEvent::new(t, ContractEventType::PP);
            schedule.push(event);
        }
    }

    // Penalty payment event
    if penalty_type == Some(PenaltyType::O) {
    } else {
        for e in schedule.clone() {
            if e.event_type == ContractEventType::PP {
                let event = ContractEvent::new(e.time, ContractEventType::PY);
                schedule.push(event);
            }
        }
    }

    // Fee payment event
    // TODO: Need to check if attributes are correct.
    if fee_rate == Real(None) || fee_rate == Real::from(0) {
    } else {
        let mut s: Time = Time(None);
        if cycle_anchor_date_of_fee == Time(None) && cycle_of_fee == None {
            s = Time(None);
        } else if cycle_anchor_date_of_fee == Time(None) {
            s = utilities::sum_cycle(initial_exchange_date, cycle_of_fee, end_of_month_convention);
        } else {
            s = cycle_anchor_date_of_fee;
        }

        let vec = utilities::schedule(s, maturity_date, cycle_of_fee, end_of_month_convention)?;

        for t in vec {
            let event = ContractEvent::new(t, ContractEventType::FP);
            schedule.push(event);
        }
    }

    // Purchase date event
    let event = ContractEvent::new(purchase_date, ContractEventType::PRD);
    schedule.push(event);

    // Termination date event
    let event = ContractEvent::new(termination_date, ContractEventType::TD);
    schedule.push(event);

    // Interest payment event
    // TODO: Check expression.
    if nominal_interest_rate == Real::from(0) {
    } else {
        let mut s: Time = Time(None);
        if cycle_anchor_date_of_interest_payment == Time(None) && cycle_of_interest_payment == None
        {
            s = Time(None);
        } else if capitalization_end_date != Time(None) {
            s = capitalization_end_date;
        } else if cycle_anchor_date_of_interest_payment == Time(None) {
            s = utilities::sum_cycle(
                initial_exchange_date,
                cycle_of_interest_payment,
                end_of_month_convention,
            );
        } else {
            s = cycle_anchor_date_of_interest_payment;
        }

        let vec = utilities::schedule(
            s,
            maturity_date,
            cycle_of_interest_payment,
            end_of_month_convention,
        )?;

        for t in vec {
            let event = ContractEvent::new(t, ContractEventType::IP);
            schedule.push(event);
        }
    }

    // Interest capitalization event
    if capitalization_end_date == Time(None) {
    } else {
        let mut s: Time = Time(None);
        if cycle_anchor_date_of_interest_payment == Time(None) && cycle_of_interest_payment == None
        {
            s = Time(None);
        } else if cycle_anchor_date_of_interest_payment == Time(None) {
            s = utilities::sum_cycle(
                initial_exchange_date,
                cycle_of_interest_payment,
                end_of_month_convention,
            );
        } else {
            s = cycle_anchor_date_of_interest_payment;
        }

        let vec = utilities::schedule(
            s,
            capitalization_end_date,
            cycle_of_interest_payment,
            end_of_month_convention,
        )?;

        for t in vec {
            let event = ContractEvent::new(t, ContractEventType::IPCI);
            schedule.push(event);
        }
    }

    // Rate reset variable event
    // TODO:Not sure what status date is.
    if cycle_anchor_date_of_rate_reset == Time(None) && cycle_of_rate_reset == None {
    } else {
        let mut s: Time = Time(None);
        if cycle_anchor_date_of_rate_reset == Time(None) {
            s = utilities::sum_cycle(
                initial_exchange_date,
                cycle_of_rate_reset,
                end_of_month_convention,
            );
        } else {
            s = cycle_anchor_date_of_rate_reset;
        }

        let vec = utilities::schedule(
            s,
            maturity_date,
            cycle_of_rate_reset,
            end_of_month_convention,
        )?;

        if next_reset_rate != Real(None) {
            let mut t_rry = Time(None);
            for t in vec.clone() {
                if t > status_date {
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
    // TODO:Not sure what status date is.
    if cycle_anchor_date_of_rate_reset == Time(None) && cycle_of_rate_reset == None {
    } else {
        let mut s: Time = Time(None);
        if cycle_anchor_date_of_rate_reset == Time(None) {
            s = utilities::sum_cycle(
                initial_exchange_date,
                cycle_of_rate_reset,
                end_of_month_convention,
            );
        } else {
            s = cycle_anchor_date_of_rate_reset;
        }

        let vec = utilities::schedule(
            s,
            maturity_date,
            cycle_of_rate_reset,
            end_of_month_convention,
        )?;

        for t in vec {
            if t > status_date {
                let event = ContractEvent::new(t, ContractEventType::RRF);
                schedule.push(event);
                break;
            }
        }
    }

    // Scaling index revision event
    if scaling_effect
        == Some(ScalingEffect {
            x: false,
            y: false,
            z: false,
        })
    {
    } else {
        let mut s: Time = Time(None);
        if cycle_anchor_date_of_scaling_index == Time(None) && cycle_of_scaling_index == None {
            s = Time(None);
        } else if cycle_anchor_date_of_scaling_index == Time(None) {
            s = utilities::sum_cycle(
                initial_exchange_date,
                cycle_of_scaling_index,
                end_of_month_convention,
            );
        } else {
            s = cycle_anchor_date_of_scaling_index;
        }

        let vec = utilities::schedule(
            s,
            maturity_date,
            cycle_of_scaling_index,
            end_of_month_convention,
        )?;

        for t in vec {
            let event = ContractEvent::new(t, ContractEventType::SC);
            schedule.push(event);
        }
    }

    // Credit default event
    // TODO: Seems to be user-initiated, so no need to appear in the schedule?

    // Ordering the schedule
    schedule.sort_unstable();

    // Initializing the variables
    let mut variables = Variables::new();

    // Temporary, remove this!
    Err("Exterminate!")
}
