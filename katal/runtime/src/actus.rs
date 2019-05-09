use parity_codec::{Decode, Encode};
use rstd::prelude::*;
use support::{decl_event, decl_module, decl_storage, dispatch::Result, StorageValue};
#[cfg(feature = "std")]
use system::ensure_signed;

/// The module's configuration trait.
pub trait Trait: system::Trait {
    type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
}

// The following enum contains all possible event types.
#[derive(Encode, Decode, Clone, PartialEq)]
#[cfg_attr(feature = "std", derive(Debug))]
pub enum EventType {
    IED,
    IPCI,
    IP,
    FP,
    PR,
    PI,
    PRF,
    PY,
    PP,
    CD,
    RRF,
    RR,
    DV,
    PRD,
    IMP,
    MP,
    TD,
    SC,
    IPCB,
    XD,
    STD,
    MD,
    AD,
}

// All the following enums are used for the contracts attributes.
#[derive(Encode, Decode, Clone, PartialEq)]
#[cfg_attr(feature = "std", derive(Debug))]
pub enum Calendar {
    NoCalendar,
    MondayToFriday,
    Calendar(u64),
}

#[derive(Encode, Decode, Clone, PartialEq)]
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

#[derive(Encode, Decode, Clone, PartialEq)]
#[cfg_attr(feature = "std", derive(Debug))]
pub enum EndOfMonthConvention {
    EOM,
    SD,
}

#[derive(Encode, Decode, Clone, PartialEq)]
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

#[derive(Encode, Decode, Clone, PartialEq)]
#[cfg_attr(feature = "std", derive(Debug))]
pub enum ContractRole {
    RPA,
    RPL,
    LG,
    ST,
    RFL,
    PFL,
    BUYER,
    SELLER,
    GUARANTOR,
    OBLIGEE,
}

#[derive(Encode, Decode, Clone, PartialEq)]
#[cfg_attr(feature = "std", derive(Debug))]
pub enum ContractStatus {
    PF,
    DL,
    DQ,
    DF,
}

#[derive(Encode, Decode, Clone, PartialEq)]
#[cfg_attr(feature = "std", derive(Debug))]
pub enum Seniority {
    S,
    J,
}

#[derive(Encode, Decode, Clone, PartialEq)]
#[cfg_attr(feature = "std", derive(Debug))]
pub enum Period {
    Days(u32),
    Weeks(u32),
    Months(u32),
    Quarters(u32),
    Halfyears(u32),
    Years(u32),
}

#[derive(Encode, Decode, Clone, PartialEq)]
#[cfg_attr(feature = "std", derive(Debug))]
pub enum GuaranteedExposure {
    NO,
    NI,
    MV,
}

#[derive(Encode, Decode, Clone, PartialEq)]
#[cfg_attr(feature = "std", derive(Debug))]
pub enum Cycle {
    Days(u32, bool),
    Weeks(u32, bool),
    Months(u32, bool),
    Quarters(u32, bool),
    Halfyears(u32, bool),
    Years(u32, bool),
}

#[derive(Encode, Decode, Clone, PartialEq)]
#[cfg_attr(feature = "std", derive(Debug))]
pub enum FeeBasis {
    A,
    N,
}

#[derive(Encode, Decode, Clone, PartialEq)]
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

#[derive(Encode, Decode, Clone, PartialEq)]
#[cfg_attr(feature = "std", derive(Debug))]
pub enum InterestCalculationBase {
    NT,
    NTIED,
    NTL,
}

#[derive(Encode, Decode, Clone, PartialEq)]
#[cfg_attr(feature = "std", derive(Debug))]
pub enum CyclePointOfInterestPayment {
    B,
    E,
}

#[derive(Encode, Decode, Clone, PartialEq)]
#[cfg_attr(feature = "std", derive(Debug))]
pub enum ClearingHouse {
    Y,
    N,
}

#[derive(Encode, Decode, Clone, PartialEq)]
#[cfg_attr(feature = "std", derive(Debug))]
pub enum IncreaseDecrease {
    INC,
    DEC,
}

#[derive(Encode, Decode, Clone, PartialEq)]
#[cfg_attr(feature = "std", derive(Debug))]
pub enum OptionExecutionType {
    E,
    B,
    A,
}

#[derive(Encode, Decode, Clone, PartialEq)]
#[cfg_attr(feature = "std", derive(Debug))]
pub enum OptionType {
    C,
    P,
    CP,
}

#[derive(Encode, Decode, Clone, PartialEq)]
#[cfg_attr(feature = "std", derive(Debug))]
pub enum PenaltyType {
    O,
    A,
    N,
    I,
}

#[derive(Encode, Decode, Clone, PartialEq)]
#[cfg_attr(feature = "std", derive(Debug))]
pub enum PrepaymentEffect {
    N,
    A,
    M,
}

#[derive(Encode, Decode, Clone, PartialEq)]
#[cfg_attr(feature = "std", derive(Debug))]
pub enum ArrayFixedVariable {
    F,
    V,
}

#[derive(Encode, Decode, Clone, PartialEq)]
#[cfg_attr(feature = "std", derive(Debug))]
pub enum CyclePointOfRateReset {
    B,
    E,
}

#[derive(Encode, Decode, Clone, PartialEq)]
#[cfg_attr(feature = "std", derive(Debug))]
pub enum DeliverySettlement {
    S,
    D,
}

// All ACTUS contract attributes as specifed in the data dictionary
// https://www.actusfrf.org/data-dictionary
#[derive(Encode, Decode, Clone, PartialEq)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct Attributes {
    Calendar: Option<Calendar>,
    BusinessDayConvention: Option<BusinessDayConvention>,
    EndOfMonthConvention: Option<EndOfMonthConvention>,
    ContractType: Option<ContractType>,
    StatusDate: Option<u64>,
    ContractRole: Option<ContractRole>,
    LegalEntityIDRecordCreator: Option<u64>,
    ContractID: Option<u64>,
    LegalEntityIDCounterparty: Option<u64>,
    ContractStatus: Option<ContractStatus>,
    Seniority: Option<Seniority>,
    NonPerformingDate: Option<u64>,
    PrepaymentPeriod: Option<Period>,
    GracePeriod: Option<Period>,
    DelinquencyPeriod: Option<Period>,
    DelinquencyRate: Option<u64>,
    GuaranteedExposure: Option<GuaranteedExposure>,
    CoverageOfCreditEnhancement: Option<u64>,
    CoveredContracts: Option<Vec<u64>>,
    CoveringContracts: Option<Vec<u64>>,
    CoveredLegalEntity: Option<u64>,
    CycleAnchorDateOfDividend: Option<u64>,
    CycleOfDividend: Option<Period>,
    NextDividendPaymentAmount: Option<u64>,
    ExDividendPayment: Option<u64>,
    CycleAnchorDateOfFee: Option<u64>,
    CycleOfFee: Option<Cycle>,
    FeeBasis: Option<FeeBasis>,
    FeeRate: Option<i64>,
    FeeAccrued: Option<i64>,
    CycleAnchorDateOfInterestPayment: Option<u64>,
    ArrayCycleAnchorDateOfInterestPayment: Option<Vec<u64>>,
    CycleOfInterestPayment: Option<Cycle>,
    ArrayCycleOfInterestPayment: Option<Vec<Cycle>>,
    NominalInterestRate: Option<i64>,
    NominalInterestRate2: Option<i64>,
    DayCountConvention: Option<DayCountConvention>,
    AccruedInterest: Option<i64>,
    CapitalizationEndDate: Option<u64>,
    CycleAnchorDateOfInterestCalculationBase: Option<u64>,
    CycleOfInterestCalculationBase: Option<Cycle>,
    InterestCalculationBase: Option<InterestCalculationBase>,
    InterestCalculationBaseAmount: Option<u64>,
    CyclePointOfInterestPayment: Option<CyclePointOfInterestPayment>,
    ClearingHouse: Option<ClearingHouse>,
    InitialMargin: Option<u64>,
    MaintenanceMarginLowerBound: Option<u64>,
    MaintenanceMarginUpperBound: Option<u64>,
    CycleAnchorDateOfMargining: Option<u64>,
    CycleOfMargining: Option<Cycle>,
    VariationMargin: Option<u64>,
    Currency: Option<u64>,
    Currency2: Option<u64>,
    AmortizationDate: Option<u64>,
    ContractDealDate: Option<u64>,
    InitialExchangeDate: Option<u64>,
    PremiumDiscountAtIED: Option<i64>,
    MaturityDate: Option<u64>,
    NotionalPrincipal: Option<u64>,
    NotionalPrincipal2: Option<u64>,
    Quantity: Option<u64>,
    Unit: Option<Vec<u8>>,
    CycleAnchorDateOfPrincipalRedemption: Option<u64>,
    ArrayCycleAnchorDateOfPrincipalRedemption: Option<Vec<u64>>,
    CycleOfPrincipalRedemption: Option<Cycle>,
    ArrayCycleOfPrincipalRedemption: Option<Vec<Cycle>>,
    NextPrincipalRedemptionPayment: Option<u64>,
    ArrayNextPrincipalRedemptionPayment: Option<Vec<u64>>,
    ArrayIncreaseDecrease: Option<Vec<IncreaseDecrease>>,
    PurchaseDate: Option<u64>,
    PriceAtPurchaseDate: Option<i64>,
    TerminationDate: Option<u64>,
    PriceAtTerminationDate: Option<u64>,
    XDayNotice: Option<Period>,
    MarketObjectCodeOfScalingIndex: Option<u64>,
    ScalingIndexAtStatusDate: Option<u64>,
    CycleAnchorDateOfScalingIndex: Option<u64>,
    CycleOfScalingIndex: Option<Cycle>,
    ScalingEffect: Option<(bool, bool, bool)>,
    MarketValueObserved: Option<i64>,
    OptionExecutionType: Option<OptionExecutionType>,
    OptionExerciseEndDate: Option<u64>,
    OptionStrike1: Option<u64>,
    OptionStrike2: Option<u64>,
    OptionType: Option<OptionType>,
    CycleAnchorDateOfOptionality: Option<u64>,
    CycleOfOptionality: Option<Cycle>,
    PenaltyType: Option<PenaltyType>,
    PenaltyRate: Option<u64>,
    PrepaymentEffect: Option<PrepaymentEffect>,
    MaximumPenaltyFreeDisbursement: Option<u64>,
    CycleAnchorDateOfRateReset: Option<u64>,
    ArrayCycleAnchorDateOfRateReset: Option<Vec<u64>>,
    CycleOfRateReset: Option<Cycle>,
    ArrayCycleOfRateReset: Option<Vec<Cycle>>,
    RateSpread: Option<i64>,
    ArrayRate: Option<Vec<i64>>,
    ArrayFixedVariable: Option<ArrayFixedVariable>,
    MarketObjectCodeRateReset: Option<u64>,
    LifeCap: Option<i64>,
    LifeFloor: Option<i64>,
    PeriodCap: Option<u64>,
    PeriodFloor: Option<u64>,
    CyclePointOfRateReset: Option<CyclePointOfRateReset>,
    FixingDays: Option<Period>,
    NextResetRate: Option<i64>,
    RateMultiplier: Option<i64>,
    SettlementDate: Option<u64>,
    DeliverySettlement: Option<DeliverySettlement>,
    FuturesPrice: Option<i64>,
}

// All ACTUS contract variables as specifed in the ACTUS paper.
#[derive(Encode, Decode, Clone, PartialEq)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct Variables {
    Performance: Option<i64>,
    LastEventDate: Option<u64>,
    NominalValue1: Option<u64>,
    NominalValue2: Option<u64>,
    NominalRate: Option<i64>,
    NominalAccrued: Option<i64>,
    InterestCalculationBase: Option<u64>,
    NotionalScalingMultiplier: Option<i64>,
    InterestScalingMultiplier: Option<i64>,
    NextPrincipalRedemptionPayment: Option<u64>,
    PayoffAtSettlement: Option<i64>,
    // Variables that are missing from the variables list. Awaiting for the full names and types.
    Tmd: Option<u64>,
    Fac: Option<u64>,
    Npr: Option<u64>,
    Nac1: Option<u64>,
    Nac2: Option<u64>,
}

// Contract Metadata, necessary for operation of the contract.
#[derive(Encode, Decode, Clone, PartialEq)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct MetaData {
    OracleObjectID: Option<u64>,
    GovernanceObjectID: Option<u64>,
    // If necessary we can add more fields.
}

// This struct contains all the information that defines a contract state.
#[derive(Encode, Decode, Clone, PartialEq)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct ContractState {
    MetaData: MetaData,
    Attributes: Attributes,
    Variables: Variables,
}

// This module's storage items.
decl_storage! {
    trait Store for Module<T: Trait> as ACTUS {
    }
}

// The module's dispatchable functions.
decl_module! {
    /// The module declaration.
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        // Initializing events
        // this is needed only if you are using events in your module
        fn deposit_event<T>() = default;

    }
}

decl_event!(
    pub enum Event<T>
    where
        AccountId = <T as system::Trait>::AccountId,
    {
        Nothing(AccountId),
    }
);

/// tests for this module
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

    // For testing the module, we construct most of a mock runtime. This means
    // first constructing a configuration type (`Test`) which `impl`s each of the
    // configuration traits of modules we want to use.
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
    impl Trait for Test {
        type Event = ();
    }
    type ACTUS = Module<Test>;

    // This function basically just builds a genesis storage key/value store according to
    // our desired mockup.
    fn new_test_ext() -> runtime_io::TestExternalities<Blake2Hasher> {
        system::GenesisConfig::<Test>::default()
            .build_storage()
            .unwrap()
            .0
            .into()
    }

    #[test]
    fn it_works_for_default_value() {
        with_externalities(&mut new_test_ext(), || {});
    }
}
