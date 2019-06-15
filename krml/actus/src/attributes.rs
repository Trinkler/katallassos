use super::*;

// All ACTUS contract attributes as specifed in the data dictionary
// https://www.actusfrf.org/data-dictionary
#[derive(Encode, Decode, Default, Clone, PartialEq)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct Attributes {
    AccruedInterest: Real,
    AmortizationDate: Time,
    ArrayCycleAnchorDateOfInterestPayment: Vec<Time>,
    ArrayCycleAnchorDateOfPrincipalRedemption: Vec<Time>,
    ArrayCycleAnchorDateOfRateReset: Vec<Time>,
    ArrayCycleOfInterestPayment: Option<Vec<Cycle>>,
    ArrayCycleOfPrincipalRedemption: Option<Vec<Cycle>>,
    ArrayCycleOfRateReset: Option<Vec<Cycle>>,
    ArrayFixedVariable: Option<ArrayFixedVariable>,
    ArrayIncreaseDecrease: Option<Vec<IncreaseDecrease>>,
    ArrayNextPrincipalRedemptionPayment: Vec<Real>,
    ArrayRate: Vec<Real>,
    BusinessDayConvention: Option<BusinessDayConvention>,
    Calendar: Option<Calendar>,
    CapitalizationEndDate: Time,
    ClearingHouse: Option<ClearingHouse>,
    ContractDealDate: Time,
    ContractID: u128,
    ContractRole: Option<ContractRole>,
    ContractStatus: Option<ContractStatus>,
    ContractType: Option<ContractType>,
    CoverageOfCreditEnhancement: Real,
    CoveredContracts: Vec<Option<u128>>,
    CoveredLegalEntity: Option<u128>, // Not sure about this one.
    CoveringContracts: Vec<Option<u128>>,
    Currency: Option<u128>,  // Represents an issuance object.
    Currency2: Option<u128>, // Represents an issuance object.
    CycleAnchorDateOfDividend: Time,
    CycleAnchorDateOfFee: Time,
    CycleAnchorDateOfInterestCalculationBase: Time,
    CycleAnchorDateOfInterestPayment: Time,
    CycleAnchorDateOfMargining: Time,
    CycleAnchorDateOfOptionality: Time,
    CycleAnchorDateOfPrincipalRedemption: Time,
    CycleAnchorDateOfRateReset: Time,
    CycleAnchorDateOfScalingIndex: Time,
    CycleOfDividend: Option<Period>,
    CycleOfFee: Option<Cycle>,
    CycleOfInterestCalculationBase: Option<Cycle>,
    CycleOfInterestPayment: Option<Cycle>,
    CycleOfMargining: Option<Cycle>,
    CycleOfOptionality: Option<Cycle>,
    CycleOfPrincipalRedemption: Option<Cycle>,
    CycleOfRateReset: Option<Cycle>,
    CycleOfScalingIndex: Option<Cycle>,
    CyclePointOfInterestPayment: Option<CyclePointOfInterestPayment>,
    CyclePointOfRateReset: Option<CyclePointOfRateReset>,
    DayCountConvention: Option<DayCountConvention>,
    DelinquencyPeriod: Option<Period>,
    DelinquencyRate: Real,
    DeliverySettlement: Option<DeliverySettlement>,
    EndOfMonthConvention: Option<EndOfMonthConvention>,
    ExDividendDate: Time,
    FeeAccrued: Real,
    FeeBasis: Option<FeeBasis>,
    FeeRate: Real,
    FixingDays: Option<Period>,
    FuturesPrice: Real,
    GracePeriod: Option<Period>,
    GuaranteedExposure: Option<GuaranteedExposure>,
    InitialExchangeDate: Time,
    InitialMargin: Real,
    InterestCalculationBase: Option<InterestCalculationBase>,
    InterestCalculationBaseAmount: Real,
    LegalEntityIDCounterparty: Option<u128>, //Not sure of this type
    LegalEntityIDRecordCreator: Option<u128>, //Not sure of this type
    LifeCap: Real,
    LifeFloor: Real,
    MaintenanceMarginLowerBound: Real,
    MaintenanceMarginUpperBound: Real,
    MarketObjectCodeOfScalingIndex: Option<u128>, //Not sure of this type
    MarketObjectCodeRateReset: Option<u128>,      // Not sure about this type
    MarketValueObserved: Real,
    MaturityDate: Time,
    MaximumPenaltyFreeDisbursement: Real,
    NextDividendPaymentAmount: Real,
    NextPrincipalRedemptionPayment: Real,
    NextResetRate: Real,
    NominalInterestRate: Real,
    NominalInterestRate2: Real,
    NonPerformingDate: Time,
    NotionalPrincipal: Real,
    NotionalPrincipal2: Real,
    OptionExecutionType: Option<OptionExecutionType>,
    OptionExerciseEndDate: Time,
    OptionStrike1: Real,
    OptionStrike2: Real,
    OptionType: Option<OptionType>,
    PenaltyRate: Real,
    PenaltyType: Option<PenaltyType>,
    PeriodCap: Real,
    PeriodFloor: Real,
    PremiumDiscountAtIED: Real,
    PrepaymentEffect: Option<PrepaymentEffect>,
    PrepaymentPeriod: Option<Period>,
    PriceAtPurchaseDate: Real,
    PriceAtTerminationDate: Real,
    PurchaseDate: Time,
    Quantity: Real,
    RateMultiplier: Real,
    RateSpread: Real,
    ScalingEffect: Option<ScalingEffect>,
    ScalingIndexAtStatusDate: Real,
    Seniority: Option<Seniority>,
    SettlementDate: Time,
    StatusDate: Time,
    TerminationDate: Time,
    Unit: Option<Unit>,
    VariationMargin: Real,
    XDayNotice: Option<Period>,
}

///
#[derive(Clone, Copy, Decode, Encode, PartialEq)]
#[cfg_attr(feature = "std", derive(Debug))]
enum Cycle {
    Days(u16, bool),
    Weeks(u16, bool),
    Months(u16, bool),
    Quarters(u16, bool),
    Halfyears(u16, bool),
    Years(u16, bool),
}

///
#[derive(Clone, Copy, Decode, Encode, PartialEq)]
#[cfg_attr(feature = "std", derive(Debug))]
pub enum Period {
    Days(u16),
    Weeks(u16),
    Months(u16),
    Quarters(u16),
    HalfYears(u16),
    Years(u16),
}

///
#[derive(Clone, Copy, Decode, Encode, PartialEq)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct ScalingEffect {
    x: bool,
    y: bool,
    z: bool,
}

// All the following enums are used for the contracts attributes.
#[derive(Clone, Copy, Decode, Encode, PartialEq)]
#[cfg_attr(feature = "std", derive(Debug))]
pub enum Calendar {
    // No Calendar
    NC,
    // Monday to Friday
    MTF,
    // Further calendars may need to be added here
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
    _30E360ISDA,
    _30E360,
    _30360,
    _BUS252,
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
