use parity_codec::{Decode, Encode};
use reals::Real;
use rstd::prelude::*;
use srml_support::{decl_event, decl_module, decl_storage, dispatch::Result, ensure, StorageMap};
use system::{ensure_signed, AccountId};
use timestamp;

mod attributes;
mod variables;

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

// Contract Metadata, necessary for operation of the contract.
#[derive(Encode, Decode, Default, Clone, PartialEq)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct MetaData {
    OracleObjectID: Option<AccountId>,
    GovernanceObjectID: Option<AccountId>,
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
