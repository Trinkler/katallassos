use parity_codec::{Decode, Encode};
use reals::Real;
use rstd::prelude::*;
use support::{decl_event, decl_module, decl_storage, dispatch::Result, ensure, StorageMap};
use system::ensure_signed;
use time::{Time, UncheckedTime};

// Importing the rest of the files in this crate.
mod attributes;
mod contract_events;
mod contract_state;
mod contracts;
mod utilities;
mod variables;
use self::attributes::*;
use self::contract_events::*;
use self::contract_state::*;
use self::contracts::*;
use self::utilities::*;
use self::variables::*;

// Defines an alias for the Result type. It has the name MyResult because Substrate already uses
// the name Result for their own type Result<(), &'static str>.
type MyResult<T> = rstd::result::Result<T, &'static str>;

// This module's configuration trait.
pub trait Trait: system::Trait {
    type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
}

// This module's events.
decl_event!(
    pub enum Event<T>
    where
        AccountId = <T as system::Trait>::AccountId,
    {
        Nothing(AccountId),
    }
);

// This module's storage items.
decl_storage! {
    trait Store for Module<T: Trait> as ACTUS {
        ContractStates: map u128 => ContractState;
    }
}

// This module's dispatchable functions.
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
