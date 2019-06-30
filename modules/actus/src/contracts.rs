use super::*;

pub fn initialize_pam(
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
    let mut attributes = attributes::new(contract_id);

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

    // // Mandatory on stand-alone and parent contracts only and
    // // not applicable on child contracts -> NN(_,_,1)
    // contract_deal_date: Time,
    // contract_role: Option<ContractRole>,
    // legal_entity_id_record_creator: Option<u128>,
    // status_date: Time,
    //
    // // Mandatory on stand-alone and parent contracts only and
    // // optional on child contracts -> NN(_,_,2)
    // legal_entity_id_counterparty: Option<u128>,

    // Optional in all cases -> x
    attributes.accrued_interest = accrued_interest;
    attributes.business_day_convention = business_day_convention;
    attributes.calendar = calendar;
    attributes.capitalization_end_date = capitalization_end_date;
    attributes.end_of_month_convention = end_of_month_convention;
    attributes.market_value_observed = market_value_observed;
    attributes.premium_discount_at_ied = premium_discount_at_ied;

    // // Optional on stand-alone and parent contracts only and
    // // not applicable on child contracts -> x(_,_,1)
    // contract_status: Option<ContractStatus>,
    // delinquency_period: Option<Period>,
    // delinquency_rate: Real,
    // grace_period: Option<Period>,
    // non_performing_date: Time,
    // seniority: Option<Seniority>,
    //
    // // Group 1
    // // Business rule ‘a’ applies unconditionally
    // fee_rate: Real, // -> x(1,0,_)
    // // Business rule ‘a’ applies if at least one of the unconditional CAs of this group is defined
    // fee_basis: Option<FeeBasis>, // -> NN(1,1,_)
    // fee_accrued: Real, // -> x(1,1,_)
    // // At least one of the CAs with c=2 has to be defined if at least one of the unconditional CAs
    // // of the group is defined
    // cycle_anchor_date_of_fee: Time, // -> x(1,2,_)
    // cycle_of_fee: Option<Cycle>, // -> x(1,2,_)
    //
    // // Group 2
    // // Business rule ‘a’ applies unconditionally
    // cycle_anchor_date_of_interest_payment: Time, // -> x(2,0,_)
    // cycle_of_interest_payment: Option<Cycle>, // -> x(2,0,_)
    // // Business rule ‘a’ applies if at least one of the unconditional CAs of this group is defined
    // cycle_point_of_interest_payment: Option<CyclePointOfInterestPayment>, // -> x(2,1,_)1
    //
    // // Group 5
    // // Business rule ‘a’ applies unconditionally
    // purchase_date: Time, // -> x(5,0,1)
    // // Business rule ‘a’ applies if at least one of the unconditional CAs of this group is defined
    // price_at_purchase_date: Real, // -> NN(5,1,1)
    //
    // // Group 6
    // // Business rule ‘a’ applies unconditionally
    // termination_date: Time, // -> x(6,0,1)
    // // Business rule ‘a’ applies if at least one of the unconditional CAs of this group is defined
    // price_at_termination_date: Real, // -> NN(6,1,1)
    //
    // // Group 7
    // // Business rule ‘a’ applies unconditionally
    // scaling_effect: Option<ScalingEffect>, // -> x(7,0,_)
    // // Business rule ‘a’ applies if at least one of the unconditional CAs of this group is defined
    // market_object_code_of_scaling_index: Option<u128>, // -> NN(7,1,_)
    // scaling_index_at_status_date: Real, // -> NN(7,1,_)
    // // At least one of the CAs with c=2 has to be defined if at least one of the unconditional CAs
    // // of the group is defined
    // cycle_anchor_date_of_scaling_index: Time, // -> x(7,2,_)
    // cycle_of_scaling_index: Option<Cycle>, // -> x(7,2,_)
    //
    // // Group 8
    // // Business rule ‘a’ applies unconditionally
    // prepayment_effect: Option<PrepaymentEffect>, // -> x(8,0,_)
    // // Business rule ‘a’ applies if at least one of the unconditional CAs of this group is defined
    // cycle_anchor_date_of_optionality: Time, // -> x(8,1,_)
    // cycle_of_optionality: Option<Cycle>, // -> x(8,1,_)
    // option_exercise_end_date: Time, // -> x(8,1,_)
    // penalty_rate: Real, // -> x(8,1,_)
    // penalty_type: Option<PenaltyType>, // -> x(8,1,_)
    // prepayment_period: Option<Period>, // -> x(8,1,1)
    //
    // // Group 9
    // // Business rule ‘a’ applies unconditionally
    // cycle_anchor_date_of_rate_reset: Time, // -> x(9,0,_)
    // cycle_of_rate_reset: Option<Cycle>, // -> x(9,0,_)
    // // Business rule ‘a’ applies if at least one of the unconditional CAs of this group is defined
    // market_object_code_rate_reset: Option<u128>, // -> NN(9,1,_)
    // rate_spread: Real, // -> NN(9,1,_)
    // fixing_days: Option<Period>, // -> x(9,1,_)
    // life_cap: Real, // -> x(9,1,_)
    // life_floor: Real, // -> x(9,1,_)
    // next_reset_rate: Real, // -> x(9,1,_)
    // period_cap: Real, // -> x(9,1,_)
    // period_floor: Real, // -> x(9,1,_)
    // rate_multiplier: Real, // -> x(9,1,_)
    // cycle_point_of_rate_reset: Option<CyclePointOfRateReset>, // -> x(9,1,_)1
}
