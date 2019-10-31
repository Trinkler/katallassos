// Copyright 2019 by Trinkler Software AG (Switzerland).
// This file is part of the Katal Chain.
//
// Katal Chain is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version <http://www.gnu.org/licenses/>.
//
// Katal Chain is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

//! # Template module
//!
//! ## Overview
//! This module exemplifies and explains the structure used for the rest of the Katal modules.
//! Except for the pure Rust modules (like the Reals and Time).

#![cfg_attr(not(feature = "std"), no_std)]
// The above line is needed to compile the Wasm binaries.

// Importing crates declared in the cargo.toml file.
use parity_codec::{Decode, Encode};
use support::{decl_module, decl_storage, dispatch::Result, StorageMap, StorageValue};

// Importing the rest of the files in this crate.
mod internal_function;
mod pure_function;
mod somesubmodule;
mod template_struct;
use internal_function::*;
use pure_function::*;
use somesubmodule::*;
use template_struct::*;

// This module's configuration trait. If you need to access the storage or the functions of
// another module you need to add their trait here.
pub trait Trait: system::Trait {}

// This module's storage items. You can use values or maps. Don't use Substrate types
// (ex: T::Hash), use the Rust primitives types (ex: u64) or the Substrate primitive types
// (ex: H256) or structs composed from those types.
decl_storage! {
    trait Store for Module<T: Trait> as TemplateStorage {
        pub SomeValue: u32;
        pub SomeMap: map u32 => TemplateState;
    }
}

// This module's dispatchable functions. These functions should be as short as possible.
// Ideally, they should only call other internal function to handle the logic and then return
// Ok if the call is successful.
decl_module! {
    // The module declaration.
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {

        // Dispatchable function can call both internal functions and pure functions.
        // But they can only be called by internal functions. However, you should avoid
        // having these functions called internally, they are for external use.
        pub fn dispatch_function(origin) -> Result {
            // Call corresponding internal function.
            Self::internal_function()?;

            // Return Ok if successful.
            Ok(())
        }
    }
}

// The unit tests for this module. The next lines until the next comment are
// all boilerplate. Just copy and paste.
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
        // If Events are ever added to this module, then the next line
        // needs to be commented out.
        // type Event = ();
    }
    // This next line should have the name of the module, in this
    // case it is Template.
    type Template = Module<Test>;

    // You can leave the next block unchanged.
    fn new_test_ext() -> runtime_io::TestExternalities<Blake2Hasher> {
        system::GenesisConfig::<Test>::default()
            .build_storage()
            .unwrap()
            .0
            .into()
    }

    // This is an example of a test function.
    #[test]
    fn set_works() {
        // The next line/block is obligatory.
        with_externalities(&mut new_test_ext(), || {
            // The body of the function begins here.
            let value = 101;
            // This is how you call a function. Note the use of 'Template'
            // instead of 'Self'.
            Template::internal_function();
            // This is how you access the module storage. Note the use of 'Test' instead
            // of 'T'. Of course, you can use other methods (put, mutate, etc).
            let value = <SomeValue<Test>>::get();
            // Then, of course, you should use some asserts in your test!
            assert_eq!(value, 23);
        });
    }
}
