use support::{decl_module, decl_storage, decl_event, StorageMap, dispatch::Result};
use system::ensure_root;
use parity_codec::{Encode, Decode};

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
	trait Store for Module<T: Trait> as OracleModule {
		DataFeeds: map u64 => Data;
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

			Ok(())
		}
	}
}

// Can't get events to work if AccountId is removed. I kept it as is so that the module compiles.
decl_event!(
	pub enum Event<T> where AccountId = <T as system::Trait>::AccountId {
		Updated(AccountId, u64),
	}
);

/// tests for this module
#[cfg(test)]
mod tests {
	use super::*;

	use runtime_io::with_externalities;
	use primitives::{H256, Blake2Hasher};
	use support::{impl_outer_origin, assert_ok};
	use runtime_primitives::{
		BuildStorage,
		traits::{BlakeTwo256, IdentityLookup},
		testing::{Digest, DigestItem, Header}
	};

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
	type OracleModule = Module<Test>;

	// This function basically just builds a genesis storage key/value store according to
	// our desired mockup.
	fn new_test_ext() -> runtime_io::TestExternalities<Blake2Hasher> {
		system::GenesisConfig::<Test>::default().build_storage().unwrap().0.into()
	}

	#[test]
	fn it_works_for_default_value() {
		with_externalities(&mut new_test_ext(), || {
			// TODO: Add revelant tests.
		});
	}
}
