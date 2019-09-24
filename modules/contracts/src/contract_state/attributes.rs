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

use super::*;

/// All ACTUS contract attributes as specified in the data dictionary in the Github.
#[derive(Clone, Decode, Encode, Default, PartialEq)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct Attributes {
    pub accrued_interest: Real,
    pub amortization_date: Time,
    pub array_cycle_anchor_date_of_interest_payment: Vec<Time>,
    pub array_cycle_anchor_date_of_principal_redemption: Vec<Time>,
    pub array_cycle_anchor_date_of_rate_reset: Vec<Time>,
    pub array_cycle_of_interest_payment: Vec<Option<Cycle>>,
    pub array_cycle_of_principal_redemption: Vec<Option<Cycle>>,
    pub array_cycle_of_rate_reset: Vec<Option<Cycle>>,
    pub array_fixed_variable: Vec<Option<ArrayFixedVariable>>,
    pub array_increase_decrease: Vec<Option<IncreaseDecrease>>,
    pub array_next_principal_redemption_payment: Vec<Real>,
    pub array_rate: Vec<Real>,
    pub business_day_convention: Option<BusinessDayConvention>,
    pub calendar: Option<Calendar>,
    pub capitalization_end_date: Time,
    pub clearing_house: Option<ClearingHouse>,
    pub contract_deal_date: Time,
    pub contract_id: H256, // Represents an contract object.
    pub contract_performance: Option<ContractPerformance>,
    pub contract_role: Option<ContractRole>,
    pub contract_structure: Vec<Option<ContractReference>>,
    pub contract_type: Option<ContractType>,
    pub counterparty_id: Option<H256>, // Represents an account object.
    pub coverage_of_credit_enhancement: Real,
    pub creator_id: Option<H256>, // Represents an account object.
    pub credit_line_amount: Real,
    pub currency: Option<u32>,   // Represents an asset object.
    pub currency_2: Option<u32>, // Represents an asset object.
    pub cycle_anchor_date_of_dividend: Time,
    pub cycle_anchor_date_of_fee: Time,
    pub cycle_anchor_date_of_interest_calculation_base: Time,
    pub cycle_anchor_date_of_interest_payment: Time,
    pub cycle_anchor_date_of_margining: Time,
    pub cycle_anchor_date_of_optionality: Time,
    pub cycle_anchor_date_of_principal_redemption: Time,
    pub cycle_anchor_date_of_rate_reset: Time,
    pub cycle_anchor_date_of_scaling_index: Time,
    pub cycle_of_dividend: Option<Period>,
    pub cycle_of_fee: Option<Cycle>,
    pub cycle_of_interest_calculation_base: Option<Cycle>,
    pub cycle_of_interest_payment: Option<Cycle>,
    pub cycle_of_margining: Option<Cycle>,
    pub cycle_of_optionality: Option<Cycle>,
    pub cycle_of_principal_redemption: Option<Cycle>,
    pub cycle_of_rate_reset: Option<Cycle>,
    pub cycle_of_scaling_index: Option<Cycle>,
    pub cycle_point_of_interest_payment: Option<CyclePointOfInterestPayment>,
    pub cycle_point_of_rate_reset: Option<CyclePointOfRateReset>,
    pub day_count_convention: Option<DayCountConvention>,
    pub delinquency_period: Option<Period>,
    pub delinquency_rate: Real,
    pub delivery_settlement: Option<DeliverySettlement>,
    pub end_of_month_convention: Option<EndOfMonthConvention>,
    pub ex_dividend_date: Time,
    pub fee_accrued: Real,
    pub fee_basis: Option<FeeBasis>,
    pub fee_rate: Real,
    pub fixing_days: Option<Period>,
    pub futures_price: Real,
    pub grace_period: Option<Period>,
    pub guaranteed_exposure: Option<GuaranteedExposure>,
    pub initial_exchange_date: Time,
    pub initial_margin: Real,
    pub interest_calculation_base: Option<InterestCalculationBase>,
    pub interest_calculation_base_amount: Real,
    pub life_cap: Real,
    pub life_floor: Real,
    pub maintenance_margin_lower_bound: Real,
    pub maintenance_margin_upper_bound: Real,
    pub market_object_code: Option<H256>, // Represents an oracle object.
    pub market_object_code_of_scaling_index: Option<H256>, // Represents an oracle object.
    pub market_object_code_rate_reset: Option<H256>, // Represents an oracle object.
    pub market_value_observed: Real,
    pub maturity_date: Time,
    pub maximum_penalty_free_disbursement: Real,
    pub next_dividend_payment_amount: Real,
    pub next_principal_redemption_payment: Real,
    pub next_reset_rate: Real,
    pub nominal_interest_rate: Real,
    pub nominal_interest_rate_2: Real,
    pub non_performing_date: Time,
    pub notional_principal: Real,
    pub notional_principal_2: Real,
    pub option_execution_type: Option<OptionExecutionType>,
    pub option_exercise_end_date: Time,
    pub option_strike_1: Real,
    pub option_strike_2: Real,
    pub option_type: Option<OptionType>,
    pub penalty_rate: Real,
    pub penalty_type: Option<PenaltyType>,
    pub period_cap: Real,
    pub period_floor: Real,
    pub premium_discount_at_ied: Real,
    pub prepayment_effect: Option<PrepaymentEffect>,
    pub prepayment_period: Option<Period>,
    pub price_at_purchase_date: Real,
    pub price_at_termination_date: Real,
    pub purchase_date: Time,
    pub quantity: Real,
    pub rate_multiplier: Real,
    pub rate_spread: Real,
    pub scaling_effect: Option<ScalingEffect>,
    pub scaling_index_at_status_date: Real,
    pub seniority: Option<Seniority>,
    pub settlement_date: Time,
    pub status_date: Time,
    pub termination_date: Time,
    pub unit: Option<Unit>,
    pub variation_margin: Real,
    pub x_day_notice: Option<Period>,
}

#[derive(Clone, Copy, Decode, Encode, PartialEq)]
#[cfg_attr(feature = "std", derive(Debug))]
pub enum ArrayFixedVariable {
    F,
    V,
}

#[derive(Clone, Copy, Decode, Encode, PartialEq)]
#[cfg_attr(feature = "std", derive(Debug))]
pub enum BusinessDayConvention {
    SCF,
    SCMF,
    CSF,
    CSMF,
    SCP,
    SCMP,
    CSP,
    CSMP,
}

#[derive(Clone, Copy, Decode, Encode, PartialEq)]
#[cfg_attr(feature = "std", derive(Debug))]
pub enum Calendar {
    NC,
    MTF,
}

#[derive(Clone, Copy, Decode, Encode, PartialEq)]
#[cfg_attr(feature = "std", derive(Debug))]
pub enum ClearingHouse {
    Y,
    N,
}

#[derive(Clone, Copy, Decode, Encode, PartialEq)]
#[cfg_attr(feature = "std", derive(Debug))]
pub enum ContractPerformance {
    PF,
    DL,
    DQ,
    DF,
}

// The underscore is necessary because 'type' is a reserved word.
#[derive(Clone, Copy, Decode, Encode, PartialEq)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct ContractReference {
    pub _object: H256,
    pub _type: ContractReferenceType,
    pub _role: ContractReferenceRole,
}

#[derive(Clone, Copy, Decode, Encode, PartialEq)]
#[cfg_attr(feature = "std", derive(Debug))]
pub enum ContractReferenceRole {
    Underlying,
    FirstLeg,
    SecondLeg,
    CoveredContract,
    CoveringContract,
}

#[derive(Clone, Copy, Decode, Encode, PartialEq)]
#[cfg_attr(feature = "std", derive(Debug))]
pub enum ContractReferenceType {
    Contract,
    ContractIdentifier,
    MarketObjectIdentifier,
    LegalEntityIdentifier,
    ContractStructure,
}

#[derive(Clone, Copy, Decode, Encode, PartialEq)]
#[cfg_attr(feature = "std", derive(Debug))]
pub enum ContractRole {
    RPA,
    RPL,
    LG,
    ST,
    BUY,
    SEL,
    RFL,
    PFL,
    GUA,
    OBL,
}

#[derive(Clone, Copy, Decode, Encode, PartialEq)]
#[cfg_attr(feature = "std", derive(Debug))]
pub enum ContractType {
    PAM,
    ANN,
    NAM,
    LAM,
    LAX,
    CLM,
    UMP,
    CSH,
    STK,
    COM,
    SWAPS,
    SWPPV,
    FXOUT,
    CAPFL,
    FUTUR,
    OPTNS,
    CEG,
    CEC,
    MRGNG,
}

// The boolean represents the stub, true = long stub, false = short stub.
#[derive(Clone, Copy, Decode, Encode, PartialEq)]
#[cfg_attr(feature = "std", derive(Debug))]
pub enum Cycle {
    Days(u16, bool),
    // Weeks(u16, bool),
    Months(u16, bool),
    // Quarters(u16, bool),
    // Halfyears(u16, bool),
    Years(u16, bool),
}

#[derive(Clone, Copy, Decode, Encode, PartialEq)]
#[cfg_attr(feature = "std", derive(Debug))]
pub enum CyclePointOfInterestPayment {
    B,
    E,
}

#[derive(Clone, Copy, Decode, Encode, PartialEq)]
#[cfg_attr(feature = "std", derive(Debug))]
pub enum CyclePointOfRateReset {
    B,
    E,
}

#[derive(Clone, Copy, Decode, Encode, PartialEq)]
#[cfg_attr(feature = "std", derive(Debug))]
pub enum DayCountConvention {
    _AAISDA,
    _A360,
    _A365,
    // _30E360ISDA,
    _30E360,
    _30360, // This one does not appear in the data dictionary?...
            // _BUS252,
}

#[derive(Clone, Copy, Decode, Encode, PartialEq)]
#[cfg_attr(feature = "std", derive(Debug))]
pub enum DeliverySettlement {
    S,
    D,
}

#[derive(Clone, Copy, Decode, Encode, PartialEq)]
#[cfg_attr(feature = "std", derive(Debug))]
pub enum EndOfMonthConvention {
    EOM,
    SD,
}

#[derive(Clone, Copy, Decode, Encode, PartialEq)]
#[cfg_attr(feature = "std", derive(Debug))]
pub enum FeeBasis {
    A,
    N,
}

#[derive(Clone, Copy, Decode, Encode, PartialEq)]
#[cfg_attr(feature = "std", derive(Debug))]
pub enum GuaranteedExposure {
    NO,
    NI,
    MV,
}

#[derive(Clone, Copy, Decode, Encode, PartialEq)]
#[cfg_attr(feature = "std", derive(Debug))]
pub enum IncreaseDecrease {
    INC,
    DEC,
}

#[derive(Clone, Copy, Decode, Encode, PartialEq)]
#[cfg_attr(feature = "std", derive(Debug))]
pub enum InterestCalculationBase {
    NT,
    NTIED,
    NTL,
}

#[derive(Clone, Copy, Decode, Encode, PartialEq)]
#[cfg_attr(feature = "std", derive(Debug))]
pub enum OptionExecutionType {
    E,
    B,
    A,
}

#[derive(Clone, Copy, Decode, Encode, PartialEq)]
#[cfg_attr(feature = "std", derive(Debug))]
pub enum OptionType {
    C,
    P,
    CP,
}

#[derive(Clone, Copy, Decode, Encode, PartialEq)]
#[cfg_attr(feature = "std", derive(Debug))]
pub enum PenaltyType {
    O,
    A,
    N,
    I,
}

#[derive(Clone, Copy, Decode, Encode, PartialEq)]
#[cfg_attr(feature = "std", derive(Debug))]
pub enum Period {
    Days(u16),
    // Weeks(u16),
    Months(u16),
    // Quarters(u16),
    // HalfYears(u16),
    Years(u16),
}

#[derive(Clone, Copy, Decode, Encode, PartialEq)]
#[cfg_attr(feature = "std", derive(Debug))]
pub enum PrepaymentEffect {
    N,
    A,
    M,
}

#[derive(Clone, Copy, Decode, Encode, PartialEq)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct ScalingEffect {
    pub x: bool,
    pub y: bool,
    pub z: bool,
}

#[derive(Clone, Copy, Decode, Encode, PartialEq)]
#[cfg_attr(feature = "std", derive(Debug))]
pub enum Seniority {
    S,
    J,
}

#[derive(Clone, Copy, Decode, Encode, PartialEq)]
#[cfg_attr(feature = "std", derive(Debug))]
pub enum Unit {
    BRL,
    BSH,
    GLN,
    CUU,
    MWH,
    PND,
    STN,
    TON,
    TRO,
}

impl Attributes {
    // Creates a new Attributes instance with every field set to the default value (as defined by
    // the ACTUS data dictionary).
    pub fn new(contract_id: H256) -> Attributes {
        Attributes {
            accrued_interest: Real(None),
            amortization_date: Time(None),
            array_cycle_anchor_date_of_interest_payment: Vec::new(),
            array_cycle_anchor_date_of_principal_redemption: Vec::new(),
            array_cycle_anchor_date_of_rate_reset: Vec::new(),
            array_cycle_of_interest_payment: Vec::new(),
            array_cycle_of_principal_redemption: Vec::new(),
            array_cycle_of_rate_reset: Vec::new(),
            array_fixed_variable: Vec::new(),
            array_increase_decrease: Vec::new(),
            array_next_principal_redemption_payment: Vec::new(),
            array_rate: Vec::new(),
            business_day_convention: Some(BusinessDayConvention::SCF),
            calendar: Some(Calendar::NC),
            capitalization_end_date: Time(None),
            clearing_house: None,
            contract_deal_date: Time(None),
            contract_id: contract_id,
            contract_performance: Some(ContractPerformance::PF),
            contract_role: None,
            contract_structure: Vec::new(),
            contract_type: None,
            counterparty_id: None,
            coverage_of_credit_enhancement: Real::from(1),
            creator_id: None,
            credit_line_amount: Real(None),
            currency: None,
            currency_2: None,
            cycle_anchor_date_of_dividend: Time(None),
            cycle_anchor_date_of_fee: Time(None),
            cycle_anchor_date_of_interest_calculation_base: Time(None),
            cycle_anchor_date_of_interest_payment: Time(None),
            cycle_anchor_date_of_margining: Time(None),
            cycle_anchor_date_of_optionality: Time(None),
            cycle_anchor_date_of_principal_redemption: Time(None),
            cycle_anchor_date_of_rate_reset: Time(None),
            cycle_anchor_date_of_scaling_index: Time(None),
            cycle_of_dividend: None,
            cycle_of_fee: None,
            cycle_of_interest_calculation_base: None,
            cycle_of_interest_payment: None,
            cycle_of_margining: None,
            cycle_of_optionality: None,
            cycle_of_principal_redemption: None,
            cycle_of_rate_reset: None,
            cycle_of_scaling_index: None,
            cycle_point_of_interest_payment: Some(CyclePointOfInterestPayment::E),
            cycle_point_of_rate_reset: Some(CyclePointOfRateReset::B),
            day_count_convention: None,
            delinquency_period: Some(Period::Days(0)),
            delinquency_rate: Real::from(0),
            delivery_settlement: Some(DeliverySettlement::D),
            end_of_month_convention: Some(EndOfMonthConvention::SD),
            ex_dividend_date: Time(None),
            fee_accrued: Real(None),
            fee_basis: None,
            fee_rate: Real(None),
            fixing_days: Some(Period::Days(0)),
            futures_price: Real(None),
            grace_period: Some(Period::Days(0)),
            guaranteed_exposure: None,
            initial_exchange_date: Time(None),
            initial_margin: Real::from(0),
            interest_calculation_base: Some(InterestCalculationBase::NT),
            interest_calculation_base_amount: Real(None),
            life_cap: Real(None),
            life_floor: Real(None),
            maintenance_margin_lower_bound: Real(None),
            maintenance_margin_upper_bound: Real(None),
            market_object_code: None,
            market_object_code_of_scaling_index: None,
            market_object_code_rate_reset: None,
            market_value_observed: Real(None),
            maturity_date: Time(None),
            maximum_penalty_free_disbursement: Real(None),
            next_dividend_payment_amount: Real::from(0),
            next_principal_redemption_payment: Real(None),
            next_reset_rate: Real(None),
            nominal_interest_rate: Real(None),
            nominal_interest_rate_2: Real(None),
            non_performing_date: Time(None),
            notional_principal: Real(None),
            notional_principal_2: Real(None),
            option_execution_type: None,
            option_exercise_end_date: Time(None),
            option_strike_1: Real(None),
            option_strike_2: Real(None),
            option_type: None,
            penalty_rate: Real::from(0),
            penalty_type: Some(PenaltyType::O),
            period_cap: Real(None),
            period_floor: Real(None),
            premium_discount_at_ied: Real::from(0),
            prepayment_effect: Some(PrepaymentEffect::N),
            prepayment_period: Some(Period::Days(0)),
            price_at_purchase_date: Real(None),
            price_at_termination_date: Real(None),
            purchase_date: Time(None),
            quantity: Real::from(1),
            rate_multiplier: Real::from(1),
            rate_spread: Real::from(0),
            scaling_effect: Some(ScalingEffect {
                x: false,
                y: false,
                z: false,
            }),
            scaling_index_at_status_date: Real(None),
            seniority: None,
            settlement_date: Time(None),
            status_date: Time(None),
            termination_date: Time(None),
            unit: None,
            variation_margin: Real(None),
            x_day_notice: None,
        }
    }

    /// Checks if an Attribute instance is valid.
    pub fn is_valid(&self) -> bool {
        // Guaranteeing that each field has a value that is in fact
        // allowed (as defined in the ACTUS dictionary as "allowed values").
        if !((self.coverage_of_credit_enhancement >= Real::from(0)
            && self.coverage_of_credit_enhancement <= Real::from(1))
            || self.coverage_of_credit_enhancement == Real(None))
            && (self.credit_line_amount >= Real::from(0) || self.credit_line_amount == Real(None))
            && (self.delinquency_rate >= Real::from(0) || self.delinquency_rate == Real(None))
            && (self.initial_margin >= Real::from(0) || self.initial_margin == Real(None))
            && (self.interest_calculation_base_amount >= Real::from(0)
                || self.interest_calculation_base_amount == Real(None))
            && (self.maintenance_margin_lower_bound >= Real::from(0)
                || self.maintenance_margin_lower_bound == Real(None))
            && (self.maintenance_margin_upper_bound >= Real::from(0)
                || self.maintenance_margin_upper_bound == Real(None))
            && (self.maximum_penalty_free_disbursement >= Real::from(0)
                || self.maximum_penalty_free_disbursement == Real(None))
            && (self.next_dividend_payment_amount >= Real::from(0)
                || self.next_dividend_payment_amount == Real(None))
            && (self.next_principal_redemption_payment >= Real::from(0)
                || self.next_principal_redemption_payment == Real(None))
            && (self.notional_principal >= Real::from(0) || self.notional_principal == Real(None))
            && (self.notional_principal_2 >= Real::from(0)
                || self.notional_principal_2 == Real(None))
            && (self.option_strike_1 >= Real::from(0) || self.option_strike_1 == Real(None))
            && (self.option_strike_2 >= Real::from(0) || self.option_strike_2 == Real(None))
            && (self.penalty_rate >= Real::from(0) || self.penalty_rate == Real(None))
            && (self.period_cap >= Real::from(0) || self.period_cap == Real(None))
            && (self.period_floor >= Real::from(0) || self.period_floor == Real(None))
            && (self.quantity >= Real::from(0) || self.quantity == Real(None))
            && (self.scaling_index_at_status_date >= Real::from(0)
                || self.scaling_index_at_status_date == Real(None))
            && (self.variation_margin >= Real::from(0) || self.variation_margin == Real(None))
        {
            return false;
        }
        // Verifying the Time Consistency Business Rules defined in ACTUS
        // TODO: Verify this implementation with Nils
        // Rule 1
        if !((self.contract_deal_date <= self.initial_exchange_date
            || self.initial_exchange_date == Time(None))
            && (self.initial_exchange_date <= self.capitalization_end_date
                || self.capitalization_end_date == Time(None))
            && (self.initial_exchange_date <= self.purchase_date
                || self.purchase_date == Time(None))
            && (self.initial_exchange_date <= self.termination_date
                || self.termination_date == Time(None))
            && (self.initial_exchange_date <= self.cycle_anchor_date_of_dividend
                || self.cycle_anchor_date_of_dividend == Time(None))
            && (self.initial_exchange_date <= self.cycle_anchor_date_of_fee
                || self.cycle_anchor_date_of_fee == Time(None))
            && (self.initial_exchange_date <= self.cycle_anchor_date_of_interest_calculation_base
                || self.cycle_anchor_date_of_interest_calculation_base == Time(None))
            && (self.initial_exchange_date <= self.cycle_anchor_date_of_margining
                || self.cycle_anchor_date_of_margining == Time(None))
            && (self.initial_exchange_date <= self.cycle_anchor_date_of_optionality
                || self.cycle_anchor_date_of_optionality == Time(None))
            && (self.initial_exchange_date <= self.cycle_anchor_date_of_principal_redemption
                || self.cycle_anchor_date_of_principal_redemption == Time(None))
            && (self.initial_exchange_date <= self.cycle_anchor_date_of_rate_reset
                || self.cycle_anchor_date_of_rate_reset == Time(None))
            && (self.initial_exchange_date <= self.cycle_anchor_date_of_scaling_index
                || self.cycle_anchor_date_of_scaling_index == Time(None))
            && (self.capitalization_end_date <= self.option_exercise_end_date
                || self.option_exercise_end_date == Time(None))
            && (self.purchase_date <= self.option_exercise_end_date
                || self.option_exercise_end_date == Time(None))
            && (self.termination_date <= self.option_exercise_end_date
                || self.option_exercise_end_date == Time(None))
            && (self.cycle_anchor_date_of_dividend <= self.option_exercise_end_date
                || self.option_exercise_end_date == Time(None))
            && (self.cycle_anchor_date_of_fee <= self.option_exercise_end_date
                || self.option_exercise_end_date == Time(None))
            && (self.cycle_anchor_date_of_interest_calculation_base
                <= self.option_exercise_end_date
                || self.option_exercise_end_date == Time(None))
            && (self.cycle_anchor_date_of_margining <= self.option_exercise_end_date
                || self.option_exercise_end_date == Time(None))
            && (self.cycle_anchor_date_of_optionality <= self.option_exercise_end_date
                || self.option_exercise_end_date == Time(None))
            && (self.cycle_anchor_date_of_principal_redemption <= self.option_exercise_end_date
                || self.option_exercise_end_date == Time(None))
            && (self.cycle_anchor_date_of_rate_reset <= self.option_exercise_end_date
                || self.option_exercise_end_date == Time(None))
            && (self.cycle_anchor_date_of_scaling_index <= self.option_exercise_end_date
                || self.option_exercise_end_date == Time(None))
            && (self.option_exercise_end_date <= self.maturity_date
                || self.maturity_date == Time(None))
            && (self.maturity_date <= self.amortization_date
                || self.amortization_date == Time(None))
            && (self.amortization_date <= self.settlement_date
                || self.settlement_date == Time(None)))
        {
            return false;
        }
        // Rule 2
        if !((self.cycle_anchor_date_of_interest_payment < self.maturity_date
            || self.maturity_date == Time(None))
            && (self.maturity_date <= self.amortization_date
                || self.amortization_date == Time(None)))
        {
            return false;
        }
        // Rule 3
        if !((self.contract_deal_date <= self.status_date || self.status_date == Time(None))
            && (self.status_date <= self.maturity_date || self.maturity_date == Time(None))
            && (self.status_date <= self.settlement_date || self.settlement_date == Time(None))
            && (self.status_date <= self.option_exercise_end_date
                || self.option_exercise_end_date == Time(None))
            && (self.status_date <= self.termination_date || self.termination_date == Time(None)))
        {
            return false;
        }
        // Rule 4
        if self.next_dividend_payment_amount.0.is_some() && self.cycle_of_dividend.is_none() {
            if !(self.status_date < self.cycle_anchor_date_of_dividend
                || self.cycle_anchor_date_of_dividend == Time(None))
            {
                return false;
            }
        }
        true
    }
}
