use parity_codec::{Decode, Encode};
use rstd::prelude::*;
use runtime_primitives::traits::As;
use support::{decl_event, decl_module, decl_storage, dispatch::Result, ensure, StorageMap};
use system::ensure_signed;
use timestamp;

/// The module's configuration trait.
pub trait Trait: system::Trait + timestamp::Trait {
    type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
}

decl_event!(
    pub enum Event<T>
    where
        AccountId = <T as system::Trait>::AccountId,
    {
        Nothing(AccountId),
    }
);

// The following enum contains all possible contract event types.
#[derive(Clone, Copy, Decode, Encode, PartialEq)]
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
#[derive(Clone, Copy, Decode, Encode, PartialEq)]
#[cfg_attr(feature = "std", derive(Debug))]
pub enum Calendar {
    NoCalendar,
    MondayToFriday,
    Calendar(i64),
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

#[derive(Clone, Copy, Decode, Encode, PartialEq)]
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
pub enum Period {
    Days(u32),
    Weeks(u32),
    Months(u32),
    Quarters(u32),
    Halfyears(u32),
    Years(u32),
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
pub enum Cycle {
    Days(u32, bool),
    Weeks(u32, bool),
    Months(u32, bool),
    Quarters(u32, bool),
    Halfyears(u32, bool),
    Years(u32, bool),
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

// All ACTUS contract attributes as specifed in the data dictionary
// https://www.actusfrf.org/data-dictionary
#[derive(Encode, Decode, Default, Clone, PartialEq)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct Attributes {
    Calendar: Option<Calendar>,
    BusinessDayConvention: Option<BusinessDayConvention>,
    EndOfMonthConvention: Option<EndOfMonthConvention>,
    ContractType: Option<ContractType>,
    StatusDate: Option<i64>,
    ContractRole: Option<ContractRole>,
    LegalEntityIDRecordCreator: Option<i64>,
    ContractID: Option<i64>,
    LegalEntityIDCounterparty: Option<i64>,
    ContractStatus: Option<ContractStatus>,
    Seniority: Option<Seniority>,
    NonPerformingDate: Option<i64>,
    PrepaymentPeriod: Option<Period>,
    GracePeriod: Option<Period>,
    DelinquencyPeriod: Option<Period>,
    DelinquencyRate: Option<i64>,
    GuaranteedExposure: Option<GuaranteedExposure>,
    CoverageOfCreditEnhancement: Option<i64>,
    CoveredContracts: Option<Vec<i64>>,
    CoveringContracts: Option<Vec<i64>>,
    CoveredLegalEntity: Option<i64>,
    CycleAnchorDateOfDividend: Option<i64>,
    CycleOfDividend: Option<Period>,
    NextDividendPaymentAmount: Option<i64>,
    ExDividendPayment: Option<i64>,
    CycleAnchorDateOfFee: Option<i64>,
    CycleOfFee: Option<Cycle>,
    FeeBasis: Option<FeeBasis>,
    FeeRate: Option<i64>,
    FeeAccrued: Option<i64>,
    CycleAnchorDateOfInterestPayment: Option<i64>,
    ArrayCycleAnchorDateOfInterestPayment: Option<Vec<i64>>,
    CycleOfInterestPayment: Option<Cycle>,
    ArrayCycleOfInterestPayment: Option<Vec<Cycle>>,
    NominalInterestRate: Option<i64>,
    NominalInterestRate2: Option<i64>,
    DayCountConvention: Option<DayCountConvention>,
    AccruedInterest: Option<i64>,
    CapitalizationEndDate: Option<i64>,
    CycleAnchorDateOfInterestCalculationBase: Option<i64>,
    CycleOfInterestCalculationBase: Option<Cycle>,
    InterestCalculationBase: Option<InterestCalculationBase>,
    InterestCalculationBaseAmount: Option<i64>,
    CyclePointOfInterestPayment: Option<CyclePointOfInterestPayment>,
    ClearingHouse: Option<ClearingHouse>,
    InitialMargin: Option<i64>,
    MaintenanceMarginLowerBound: Option<i64>,
    MaintenanceMarginUpperBound: Option<i64>,
    CycleAnchorDateOfMargining: Option<i64>,
    CycleOfMargining: Option<Cycle>,
    VariationMargin: Option<i64>,
    Currency: Option<i64>,
    Currency2: Option<i64>,
    AmortizationDate: Option<i64>,
    ContractDealDate: Option<i64>,
    InitialExchangeDate: Option<i64>,
    PremiumDiscountAtIED: Option<i64>,
    MaturityDate: Option<i64>,
    NotionalPrincipal: Option<i64>,
    NotionalPrincipal2: Option<i64>,
    Quantity: Option<i64>,
    Unit: Option<Vec<u8>>,
    CycleAnchorDateOfPrincipalRedemption: Option<i64>,
    ArrayCycleAnchorDateOfPrincipalRedemption: Option<Vec<i64>>,
    CycleOfPrincipalRedemption: Option<Cycle>,
    ArrayCycleOfPrincipalRedemption: Option<Vec<Cycle>>,
    NextPrincipalRedemptionPayment: Option<i64>,
    ArrayNextPrincipalRedemptionPayment: Option<Vec<i64>>,
    ArrayIncreaseDecrease: Option<Vec<IncreaseDecrease>>,
    PurchaseDate: Option<i64>,
    PriceAtPurchaseDate: Option<i64>,
    TerminationDate: Option<i64>,
    PriceAtTerminationDate: Option<i64>,
    XDayNotice: Option<Period>,
    MarketObjectCodeOfScalingIndex: Option<i64>,
    ScalingIndexAtStatusDate: Option<i64>,
    CycleAnchorDateOfScalingIndex: Option<i64>,
    CycleOfScalingIndex: Option<Cycle>,
    ScalingEffect: Option<(bool, bool, bool)>,
    MarketValueObserved: Option<i64>,
    OptionExecutionType: Option<OptionExecutionType>,
    OptionExerciseEndDate: Option<i64>,
    OptionStrike1: Option<i64>,
    OptionStrike2: Option<i64>,
    OptionType: Option<OptionType>,
    CycleAnchorDateOfOptionality: Option<i64>,
    CycleOfOptionality: Option<Cycle>,
    PenaltyType: Option<PenaltyType>,
    PenaltyRate: Option<i64>,
    PrepaymentEffect: Option<PrepaymentEffect>,
    MaximumPenaltyFreeDisbursement: Option<i64>,
    CycleAnchorDateOfRateReset: Option<i64>,
    ArrayCycleAnchorDateOfRateReset: Option<Vec<i64>>,
    CycleOfRateReset: Option<Cycle>,
    ArrayCycleOfRateReset: Option<Vec<Cycle>>,
    RateSpread: Option<i64>,
    ArrayRate: Option<Vec<i64>>,
    ArrayFixedVariable: Option<ArrayFixedVariable>,
    MarketObjectCodeRateReset: Option<i64>,
    LifeCap: Option<i64>,
    LifeFloor: Option<i64>,
    PeriodCap: Option<i64>,
    PeriodFloor: Option<i64>,
    CyclePointOfRateReset: Option<CyclePointOfRateReset>,
    FixingDays: Option<Period>,
    NextResetRate: Option<i64>,
    RateMultiplier: Option<i64>,
    SettlementDate: Option<i64>,
    DeliverySettlement: Option<DeliverySettlement>,
    FuturesPrice: Option<i64>,
}

// All ACTUS contract variables as specifed in the ACTUS paper.
#[derive(Encode, Decode, Default, Clone, PartialEq)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct Variables {
    Performance: Option<i64>,
    LastEventDate: Option<i64>,
    NominalValue1: Option<i64>,
    NominalValue2: Option<i64>,
    NominalRate: Option<i64>,
    NominalAccrued: Option<i64>,
    InterestCalculationBase: Option<i64>,
    NotionalScalingMultiplier: Option<i64>,
    InterestScalingMultiplier: Option<i64>,
    NextPrincipalRedemptionPayment: Option<i64>,
    PayoffAtSettlement: Option<i64>,
    // Variables that are missing from the variables list. Awaiting for the full names and types.
    Tmd: Option<i64>,
    Fac: Option<i64>,
    Npr: Option<i64>,
    Nac1: Option<i64>,
    Nac2: Option<i64>,
}

// Contract Metadata, necessary for operation of the contract.
#[derive(Encode, Decode, Default, Clone, PartialEq)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct MetaData {
    OracleObjectID: Option<i64>,
    GovernanceObjectID: Option<i64>,
    // If necessary we can add more fields.
}

// This struct contains all the information that defines a contract state.
#[derive(Encode, Decode, Default, Clone, PartialEq)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct ContractState {
    MetaData: MetaData,
    Attributes: Attributes,
    Variables: Variables,
}

// This module's storage items.
decl_storage! {
    trait Store for Module<T: Trait> as ACTUS {
        ContractStates: map i64 => ContractState;
    }
}

// The module's dispatchable functions.
decl_module! {
    /// The module declaration.
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        // Initializing events
        // this is needed only if you are using events in your module
        fn deposit_event<T>() = default;

        fn deploy (origin, meta_data: MetaData, attributes: Attributes) -> Result {
            let sender = ensure_signed(origin)?;

            let key = attributes.ContractID.ok_or("ContractID can't be None when deploying a contract")?;

            ensure!(!<ContractStates<T>>::exists(key), "This ContractID already exists");

            let contract_type = attributes.ContractType.ok_or("ContractType can't be None when deploying a contract")?;

            Ok(())
        }

    }
}

impl<T: Trait> Module<T> {
    fn deploy_PAM(
        sender: T::AccountId,
        key: i64,
        meta_data: MetaData,
        attributes: Attributes,
    ) -> Result {
        let now = <timestamp::Module<T>>::get();
        // TODO: Assert correctness of attributes and metadata, for each contract type.

        // TODO: Initialize state variables.
        let tmd = Some(
            attributes
                .MaturityDate
                .ok_or("MaturityDate can't be None when deploying a contract")?,
        );

        let initial_exchange_date: i64 = attributes
            .InitialExchangeDate
            .ok_or("InitialExchangeDate can't be None when deploying a contract")?;

        let contract_role = attributes
            .ContractRole
            .ok_or("ContractRole can't be None when deploying a contract")?;

        let notional_principal = attributes
            .NotionalPrincipal
            .ok_or("NotionalPrincipal can't be None when deploying a contract")?;

        let day_count_convention = attributes
            .DayCountConvention
            .ok_or("DayCountConvention can't be None when deploying a contract")?;

        let nominal_interest_rate = attributes
            .NominalInterestRate
            .ok_or("NominalInterestRate can't be None when deploying a contract")?;

        // TODO: Assert initial_exchange_date to be larger than 0
        // The outstanding nominal value
        let notional_value_1: Option<i64> = if T::Moment::sa(initial_exchange_date as u64) > now {
            Some(0)
        } else {
            Some(Self::utility_function_R(&contract_role) * notional_principal)
        };

        // The applicable nominal rate
        let notional_rate: Option<i64> = if T::Moment::sa(initial_exchange_date as u64) > now {
            Some(0)
        } else {
            Some(nominal_interest_rate)
        };

        // The current value of nominal accrued interest at the Nominal Rate
        let nominal_accrued: Option<i64> = if attributes.NominalInterestRate != None {
            Some(0)
        } else if attributes.AccruedInterest != None {
            attributes.AccruedInterest
        } else {
            // TODO: implment actual function
            Some(
                Self::utility_function_Y(
                    0, // TODO: Substitute with actual time s
                    1, // TODO: Substitute with actual time t
                    &day_count_convention,
                ) * notional_value_1.ok_or("")?
                    * notional_rate.ok_or("")?,
            )
        };

        let variables_initial = Variables {
            Performance: None,
            LastEventDate: None,
            NominalValue1: notional_value_1,
            NominalValue2: None,
            NominalRate: notional_rate,
            NominalAccrued: None,
            InterestCalculationBase: None,
            NotionalScalingMultiplier: None,
            InterestScalingMultiplier: None,
            NextPrincipalRedemptionPayment: None,
            PayoffAtSettlement: None,
            Tmd: tmd,
            Fac: None,
            Npr: None,
            Nac1: None,
            Nac2: None,
        };

        let contract_state = ContractState {
            MetaData: meta_data,
            Attributes: attributes,
            Variables: variables_initial,
        };

        <ContractStates<T>>::insert(key, contract_state);

        Ok(())
    }

    // Contract Role Sign Convention
    fn utility_function_R(contract_role: &ContractRole) -> i64 {
        match contract_role {
            ContractRole::RPA => 1,
            ContractRole::RPL => -1,
            ContractRole::LG => 1,
            ContractRole::ST => -1,
            ContractRole::RFL => 1,
            ContractRole::PFL => -1,
            ContractRole::BUYER => 1,
            ContractRole::SELLER => -1,
            // TODO: Verify that guarantor maps to -1
            ContractRole::GUARANTOR => -1,
            // TODO: Verify that obligee maps to 1
            ContractRole::OBLIGEE => 1,
        }
    }

    // Year Fraction Convention
    // TODO: Implement actual function
    // TODO: Adjust retun to be a float
    fn utility_function_Y(s: i64, t: i64, day_cont_convention: &DayCountConvention) -> i64 {
        match day_cont_convention {
            DayCountConvention::_AAISDA => 1,
            DayCountConvention::_A360 => 1,
            DayCountConvention::_A365 => 1,
            DayCountConvention::_30E360ISDA => 1,
            DayCountConvention::_30E360 => 1,
            DayCountConvention::_30360 => 1,
            DayCountConvention::_BUS252 => 1,
        }
    }
}

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
