use parity_codec::{Decode, Encode};
use reals::Real;
use rstd::prelude::*;
use support::{decl_event, decl_module, decl_storage, dispatch::Result, ensure, StorageMap};
use system::ensure_signed;
use time::{Time, UncheckedTime};

// Importing the rest of the files in this crate.
mod attributes;
mod utilities;
mod variables;
use self::attributes::*;
use self::utilities::*;
use self::variables::*;

// The module's configuration trait.
pub trait Trait: system::Trait {
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

// This struct contains all the information that defines a contract state.
#[derive(Encode, Decode, Default, Clone, PartialEq)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct ContractState {
    attributes: Attributes,
    variables: Variables,
}

// This module's storage items.
decl_storage! {
    trait Store for Module<T: Trait> as ACTUS {
        ContractStates: map u128 => ContractState;
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

// tests for this module
// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     use primitives::{Blake2Hasher, H256};
//     use runtime_io::with_externalities;
//     use runtime_primitives::{
//         testing::{Digest, DigestItem, Header},
//         traits::{BlakeTwo256, IdentityLookup},
//         BuildStorage,
//     };
//     use support::{assert_ok, impl_outer_origin};
//
//     impl_outer_origin! {
//         pub enum Origin for Test {}
//     }
//
//     // For testing the module, we construct most of a mock runtime. This means
//     // first constructing a configuration type (`Test`) which `impl`s each of the
//     // configuration traits of modules we want to use.
//     #[derive(Clone, Eq, PartialEq)]
//     pub struct Test;
//     impl system::Trait for Test {
//         type Origin = Origin;
//         type Index = u64;
//         type BlockNumber = u64;
//         type Hash = H256;
//         type Hashing = BlakeTwo256;
//         type Digest = Digest;
//         type AccountId = u64;
//         type Lookup = IdentityLookup<Self::AccountId>;
//         type Header = Header;
//         type Event = ();
//         type Log = DigestItem;
//     }
//     impl Trait for Test {
//         type Event = ();
//     }
//     type ACTUS = Module<Test>;
//
//     // This function basically just builds a genesis storage key/value store according to
//     // our desired mockup.
//     fn new_test_ext() -> runtime_io::TestExternalities<Blake2Hasher> {
//         system::GenesisConfig::<Test>::default()
//             .build_storage()
//             .unwrap()
//             .0
//             .into()
//     }
//
//     #[test]
//     fn it_works_for_default_value() {
//         with_externalities(&mut new_test_ext(), || {});
//     }
// }
