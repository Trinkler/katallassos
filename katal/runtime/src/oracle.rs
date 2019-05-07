use parity_codec::{Decode, Encode};
#[cfg(feature = "std")]
use rand::random;
use support::{decl_event, decl_module, decl_storage, dispatch::Result, StorageMap};
use system::ensure_root;
/// The module's configuration trait.
pub trait Trait: system::Trait {
    type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
}

// The struct that holds the value of a data feed.
#[derive(Encode, Decode, Default, Clone, PartialEq)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct Data {
    price: i64,
    time: u64,
}

// This module's storage items.
decl_storage! {
    trait Store for Module<T: Trait> as Oracle {
        DataFeeds get(lookup): map u64 => Data;
    }
}

// The module's dispatchable functions.
decl_module! {
    /// The module declaration.
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        // Initializing events
        fn deposit_event<T>() = default;

        // Updating the value of an existing data feed or creating a new one.
        pub fn update(origin, key: u64, price: i64, time: u64) -> Result {
            ensure_root(origin)?;

            let value = Data {
                price: price,
                time: time,
            };

            <DataFeeds<T>>::insert(key, value);

            Self::deposit_event(RawEvent::Updated(key, price, time));

            Ok(())
        }
    }
}

// If you do not use AccountId in a event, it will complain that AccountId trait is not being used. If you also delete the importing of the trait ('where AccountId = <T as system::Trait>::AccountId'), it will also not compile. The only solution then seems to be to have an event that uses AccountId but is simply never used. That is why we have the Nothing event.
decl_event!(
    pub enum Event<T>
    where
        AccountId = <T as system::Trait>::AccountId,
    {
        Nothing(AccountId),
        // Updated event serves both as trigger for front-end apps and as a way of storing the price history of the data feed.
        Updated(u64, i64, u64),
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
    type Oracle = Module<Test>;

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
    fn it_can_update_and_get_random_values() {
        with_externalities(&mut new_test_ext(), || {
            let key: u64 = rand::random::<u64>();
            let price: i64 = rand::random::<i64>();;
            let time: u64 = rand::random::<u64>();;

            assert_ok!(Oracle::update(Origin::ROOT, key, price, time));
            assert_eq!(Oracle::lookup(key).price, price);
            assert_eq!(Oracle::lookup(key).time, time);
        });
    }
}
