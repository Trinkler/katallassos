use super::*;

// All ACTUS contract attributes as specifed in the data dictionary
// https://www.actusfrf.org/data-dictionary
#[derive(Encode, Decode, Default, Clone, PartialEq)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct Attributes {
    Calendar: Option<Calendar>,
    BusinessDayConvention: Option<BusinessDayConvention>,
    EndOfMonthConvention: Option<EndOfMonthConvention>,
    ContractType: Option<ContractType>,
    StatusDate: Real,
    ContractRole: Option<ContractRole>,
    LegalEntityIDRecordCreator: Option<i64>,
    ContractID: u128,
    LegalEntityIDCounterparty: Option<u128>,
    ContractStatus: Option<ContractStatus>,
    Seniority: Option<Seniority>,
    NonPerformingDate: Real,
    PrepaymentPeriod: Option<Period>,
    GracePeriod: Option<Period>,
    DelinquencyPeriod: Option<Period>,
    DelinquencyRate: Real,
    GuaranteedExposure: Option<GuaranteedExposure>,
    CoverageOfCreditEnhancement: Real,
    CoveredContracts: Vec<Option<u128>>,
    CoveringContracts: Vec<Option<u128>>,
    CoveredLegalEntity: Option<u128>,
    CycleAnchorDateOfDividend: Real,
    CycleOfDividend: Option<Period>,
    NextDividendPaymentAmount: Real,
    ExDividendPayment: Real,
    CycleAnchorDateOfFee: Real,
    CycleOfFee: Option<Cycle>,
    FeeBasis: Option<FeeBasis>,
    FeeRate: Real,
    FeeAccrued: Real,
    CycleAnchorDateOfInterestPayment: Real,
    ArrayCycleAnchorDateOfInterestPayment: Vec<Real>,
    CycleOfInterestPayment: Option<Cycle>,
    ArrayCycleOfInterestPayment: Option<Vec<Cycle>>,
    NominalInterestRate: Real,
    NominalInterestRate2: Real,
    DayCountConvention: Option<DayCountConvention>,
    AccruedInterest: Real,
    CapitalizationEndDate: Real,
    CycleAnchorDateOfInterestCalculationBase: Real,
    CycleOfInterestCalculationBase: Option<Cycle>,
    InterestCalculationBase: Option<InterestCalculationBase>,
    InterestCalculationBaseAmount: Real,
    CyclePointOfInterestPayment: Option<CyclePointOfInterestPayment>,
    ClearingHouse: Option<ClearingHouse>,
    InitialMargin: Real,
    MaintenanceMarginLowerBound: Real,
    MaintenanceMarginUpperBound: Real,
    CycleAnchorDateOfMargining: Real,
    CycleOfMargining: Option<Cycle>,
    VariationMargin: Real,
    Currency: Option<u128>,
    Currency2: Option<u128>,
    AmortizationDate: Real,
    ContractDealDate: Real,
    InitialExchangeDate: Real,
    PremiumDiscountAtIED: Real,
    MaturityDate: Real,
    NotionalPrincipal: Real,
    NotionalPrincipal2: Real,
    Quantity: Real,
    Unit: Option<Unit>,
    CycleAnchorDateOfPrincipalRedemption: Real,
    ArrayCycleAnchorDateOfPrincipalRedemption: Vec<Real>,
    CycleOfPrincipalRedemption: Option<Cycle>,
    ArrayCycleOfPrincipalRedemption: Option<Vec<Cycle>>,
    NextPrincipalRedemptionPayment: Real,
    ArrayNextPrincipalRedemptionPayment: Vec<Real>,
    ArrayIncreaseDecrease: Option<Vec<IncreaseDecrease>>,
    PurchaseDate: Real,
    PriceAtPurchaseDate: Real,
    TerminationDate: Real,
    PriceAtTerminationDate: Real,
    XDayNotice: Option<Period>,
    MarketObjectCodeOfScalingIndex: Option<u128>, //Not sure of this type
    ScalingIndexAtStatusDate: Real,
    CycleAnchorDateOfScalingIndex: Real,
    CycleOfScalingIndex: Option<Cycle>,
    ScalingEffect: Option<(bool, bool, bool)>,
    MarketValueObserved: Real,
    OptionExecutionType: Option<OptionExecutionType>,
    OptionExerciseEndDate: Real,
    OptionStrike1: Real,
    OptionStrike2: Real,
    OptionType: Option<OptionType>,
    CycleAnchorDateOfOptionality: Real,
    CycleOfOptionality: Option<Cycle>,
    PenaltyType: Option<PenaltyType>,
    PenaltyRate: Real,
    PrepaymentEffect: Option<PrepaymentEffect>,
    MaximumPenaltyFreeDisbursement: Real,
    CycleAnchorDateOfRateReset: Real,
    ArrayCycleAnchorDateOfRateReset: Vec<Real>,
    CycleOfRateReset: Option<Cycle>,
    ArrayCycleOfRateReset: Option<Vec<Cycle>>,
    RateSpread: Real,
    ArrayRate: Vec<Real>,
    ArrayFixedVariable: Option<ArrayFixedVariable>,
    MarketObjectCodeRateReset: Option<u128>, // Not sure about this type
    LifeCap: Real,
    LifeFloor: Real,
    PeriodCap: Real,
    PeriodFloor: Real,
    CyclePointOfRateReset: Option<CyclePointOfRateReset>,
    FixingDays: Option<Period>,
    NextResetRate: Real,
    RateMultiplier: Real,
    SettlementDate: Real,
    DeliverySettlement: Option<DeliverySettlement>,
    FuturesPrice: Real,
}

///
#[derive(Clone, Copy, Decode, Encode, PartialEq)]
#[cfg_attr(feature = "std", derive(Debug))]
enum Cycle {
    Days(u32, bool),
    Weeks(u32, bool),
    Months(u32, bool),
    Quarters(u32, bool),
    Halfyears(u32, bool),
    Years(u32, bool),
}

///
#[derive(Clone, Copy, Decode, Encode, PartialEq)]
#[cfg_attr(feature = "std", derive(Debug))]
pub enum Period {
    Days(u32),
    Weeks(u32),
    Months(u32),
    Quarters(u32),
    HalfYears(u32),
    Years(u32),
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
