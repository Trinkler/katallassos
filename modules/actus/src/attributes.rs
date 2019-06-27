use super::*;

/// All ACTUS contract attributes as specified in the data dictionary.
#[derive(Clone, Decode, Encode, Default, PartialEq)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct Attributes {
    accrued_interest: Real,
    amortization_date: Time,
    array_cycle_anchor_date_of_interest_payment: Vec<Time>,
    array_cycle_anchor_date_of_principal_redemption: Vec<Time>,
    array_cycle_anchor_date_of_rate_reset: Vec<Time>,
    array_cycle_of_interest_payment: Vec<Option<Cycle>>,
    array_cycle_of_principal_redemption: Vec<Option<Cycle>>,
    array_cycle_of_rate_reset: Vec<Option<Cycle>>,
    array_fixed_variable: Option<ArrayFixedVariable>,
    array_increase_decrease: Vec<Option<IncreaseDecrease>>,
    array_next_principal_redemption_payment: Vec<Real>,
    array_rate: Vec<Real>,
    business_day_convention: Option<BusinessDayConvention>,
    calendar: Option<Calendar>,
    capitalization_end_date: Time,
    clearing_house: Option<ClearingHouse>,
    contract_deal_date: Time,
    contract_id: u128,
    contract_role: Option<ContractRole>,
    contract_status: Option<ContractStatus>,
    contract_type: Option<ContractType>,
    coverage_of_credit_enhancement: Real,
    covered_contracts: Vec<Option<u128>>,
    covered_legal_entity: Option<u128>, // Not sure about this one.
    covering_contracts: Vec<Option<u128>>,
    currency: Option<u128>,   // Represents an issuance object.
    currency_2: Option<u128>, // Represents an issuance object.
    cycle_anchor_date_of_dividend: Time,
    cycle_anchor_date_of_fee: Time,
    cycle_anchor_date_of_interest_calculation_base: Time,
    cycle_anchor_date_of_interest_payment: Time,
    cycle_anchor_date_of_margining: Time,
    cycle_anchor_date_of_optionality: Time,
    cycle_anchor_date_of_principal_redemption: Time,
    cycle_anchor_date_of_rate_reset: Time,
    cycle_anchor_date_of_scaling_index: Time,
    cycle_of_dividend: Option<Period>,
    cycle_of_fee: Option<Cycle>,
    cycle_of_interest_calculation_base: Option<Cycle>,
    cycle_of_nterest_payment: Option<Cycle>,
    cycle_of_margining: Option<Cycle>,
    cycle_of_optionality: Option<Cycle>,
    cycle_of_principal_redemption: Option<Cycle>,
    cycle_of_rate_reset: Option<Cycle>,
    cycle_of_scaling_index: Option<Cycle>,
    cycle_point_of_interest_payment: Option<CyclePointOfInterestPayment>,
    cycle_point_of_rate_reset: Option<CyclePointOfRateReset>,
    day_count_convention: Option<DayCountConvention>,
    delinquency_period: Option<Period>,
    delinquency_rate: Real,
    delivery_settlement: Option<DeliverySettlement>,
    end_of_month_convention: Option<EndOfMonthConvention>,
    ex_dividend_date: Time,
    fee_accrued: Real,
    fee_basis: Option<FeeBasis>,
    fee_rate: Real,
    fixing_days: Option<Period>,
    futures_price: Real,
    grace_period: Option<Period>,
    guaranteed_exposure: Option<GuaranteedExposure>,
    initial_exchange_date: Time,
    initial_margin: Real,
    interest_calculation_base: Option<InterestCalculationBase>,
    interest_calculation_base_amount: Real,
    legal_entity_id_counterparty: Option<u128>,
    legal_entity_id_record_creator: Option<u128>,
    life_cap: Real,
    life_floor: Real,
    maintenance_margin_lower_bound: Real,
    maintenance_margin_upper_bound: Real,
    market_object_code_of_scaling_index: Option<u128>, //Not sure of this type
    market_object_code_rate_reset: Option<u128>,       // Not sure about this type
    market_value_observed: Real,
    maturity_date: Time,
    maximum_penalty_free_disbursement: Real,
    next_dividend_payment_amount: Real,
    next_principal_redemption_payment: Real,
    next_reset_rate: Real,
    nominal_interest_rate: Real,
    nominal_interest_rate_2: Real,
    non_performing_date: Time,
    notional_principal: Real,
    notional_principal_2: Real,
    option_execution_type: Option<OptionExecutionType>,
    option_exercise_end_date: Time,
    option_strike_1: Real,
    option_strike_2: Real,
    option_type: Option<OptionType>,
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
    quantity: Real,
    rate_multiplier: Real,
    rate_spread: Real,
    scaling_effect: Option<ScalingEffect>,
    scaling_index_at_status_date: Real,
    seniority: Option<Seniority>,
    settlement_date: Time,
    status_date: Time,
    termination_date: Time,
    unit: Option<Unit>,
    variation_margin: Real,
    x_day_notice: Option<Period>,
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
    x: bool,
    y: bool,
    z: bool,
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
