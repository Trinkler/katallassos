use super::*;

/// All ACTUS contract attributes as specified in the data dictionary.
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
    pub array_fixed_variable: Option<ArrayFixedVariable>,
    pub array_increase_decrease: Vec<Option<IncreaseDecrease>>,
    pub array_next_principal_redemption_payment: Vec<Real>,
    pub array_rate: Vec<Real>,
    pub business_day_convention: Option<BusinessDayConvention>,
    pub calendar: Option<Calendar>,
    pub capitalization_end_date: Time,
    pub clearing_house: Option<ClearingHouse>,
    pub contract_deal_date: Time,
    pub contract_id: u128,
    pub contract_role: Option<ContractRole>,
    pub contract_status: Option<ContractStatus>,
    pub contract_type: Option<ContractType>,
    pub coverage_of_credit_enhancement: Real,
    pub covered_contracts: Vec<Option<u128>>,
    pub covered_legal_entity: Option<u128>, // Not sure about this one.
    pub covering_contracts: Vec<Option<u128>>,
    pub currency: Option<u128>,   // Represents an issuance object.
    pub currency_2: Option<u128>, // Represents an issuance object.
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
    pub legal_entity_id_counterparty: Option<u128>,
    pub legal_entity_id_record_creator: Option<u128>,
    pub life_cap: Real,
    pub life_floor: Real,
    pub maintenance_margin_lower_bound: Real,
    pub maintenance_margin_upper_bound: Real,
    pub market_object_code_of_scaling_index: Option<u128>, //Not sure of this type
    pub market_object_code_rate_reset: Option<u128>,       // Not sure about this type
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
pub struct ScalingEffect {
    pub x: bool,
    pub y: bool,
    pub z: bool,
}

#[derive(Clone, Copy, Decode, Encode, PartialEq)]
#[cfg_attr(feature = "std", derive(Debug))]
pub enum Calendar {
    // No Calendar
    NC,
    // Monday to Friday
    // MTF,
    // Further calendars may need to be added here
}

#[derive(Clone, Copy, Decode, Encode, PartialEq)]
#[cfg_attr(feature = "std", derive(Debug))]
pub enum BusinessDayConvention {
    NULL,
    // SCF,
    // SCMF,
    // CSF,
    // CSMF,
    // SCP,
    // SCMP,
    // CSP,
    // CSMP,
}

#[derive(Clone, Copy, Decode, Encode, PartialEq)]
#[cfg_attr(feature = "std", derive(Debug))]
pub enum EndOfMonthConvention {
    EOM,
    SD,
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

// This specific attribute is according to the ACTUS paper and not the Data Dictionary.
#[derive(Clone, Copy, Decode, Encode, PartialEq)]
#[cfg_attr(feature = "std", derive(Debug))]
pub enum ContractRole {
    RPA,
    RPL,
    CLO,
    CNO,
    COL,
    LG,
    ST,
    BUY,
    SEL,
    RFL,
    PFL,
    RF,
    PF,
}

#[derive(Clone, Copy, Decode, Encode, PartialEq)]
#[cfg_attr(feature = "std", derive(Debug))]
pub enum ContractStatus {
    PF,
    DL,
    DQ,
    DF,
}

#[derive(Clone, Copy, Decode, Encode, PartialEq)]
#[cfg_attr(feature = "std", derive(Debug))]
pub enum Seniority {
    S,
    J,
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
pub enum FeeBasis {
    A,
    N,
}

#[derive(Clone, Copy, Decode, Encode, PartialEq)]
#[cfg_attr(feature = "std", derive(Debug))]
pub enum DayCountConvention {
    _AAISDA,
    _A360,
    _A365,
    // _30E360ISDA,
    _30E360,
    _30360,
    // _BUS252,
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
pub enum CyclePointOfInterestPayment {
    B,
    E,
}

#[derive(Clone, Copy, Decode, Encode, PartialEq)]
#[cfg_attr(feature = "std", derive(Debug))]
pub enum ClearingHouse {
    Y,
    N,
}

#[derive(Clone, Copy, Decode, Encode, PartialEq)]
#[cfg_attr(feature = "std", derive(Debug))]
pub enum IncreaseDecrease {
    INC,
    DEC,
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
pub enum PrepaymentEffect {
    N,
    A,
    M,
}

#[derive(Clone, Copy, Decode, Encode, PartialEq)]
#[cfg_attr(feature = "std", derive(Debug))]
pub enum ArrayFixedVariable {
    F,
    V,
}

#[derive(Clone, Copy, Decode, Encode, PartialEq)]
#[cfg_attr(feature = "std", derive(Debug))]
pub enum CyclePointOfRateReset {
    B,
    E,
}

#[derive(Clone, Copy, Decode, Encode, PartialEq)]
#[cfg_attr(feature = "std", derive(Debug))]
pub enum DeliverySettlement {
    S,
    D,
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
    pub fn new(contract_id: u128) -> Attributes {
        Attributes {
            accrued_interest: Real(None),
            amortization_date: Time(None),
            array_cycle_anchor_date_of_interest_payment: Vec::new(),
            array_cycle_anchor_date_of_principal_redemption: Vec::new(),
            array_cycle_anchor_date_of_rate_reset: Vec::new(),
            array_cycle_of_interest_payment: Vec::new(),
            array_cycle_of_principal_redemption: Vec::new(),
            array_cycle_of_rate_reset: Vec::new(),
            array_fixed_variable: None,
            array_increase_decrease: Vec::new(),
            array_next_principal_redemption_payment: Vec::new(),
            array_rate: Vec::new(),
            business_day_convention: Some(BusinessDayConvention::NULL),
            calendar: Some(Calendar::NC),
            capitalization_end_date: Time(None),
            clearing_house: None,
            contract_deal_date: Time(None),
            contract_id: contract_id,
            contract_role: None,
            contract_status: Some(ContractStatus::PF),
            contract_type: None,
            coverage_of_credit_enhancement: Real::from(1),
            covered_contracts: Vec::new(),
            covered_legal_entity: None,
            covering_contracts: Vec::new(),
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
            delivery_settlement: None,
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
            initial_margin: Real(None),
            interest_calculation_base: Some(InterestCalculationBase::NT),
            interest_calculation_base_amount: Real(None),
            legal_entity_id_counterparty: None,
            legal_entity_id_record_creator: None,
            life_cap: Real(None),
            life_floor: Real(None),
            maintenance_margin_lower_bound: Real(None),
            maintenance_margin_upper_bound: Real(None),
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
}
