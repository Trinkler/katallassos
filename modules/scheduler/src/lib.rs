// Copyright 2019 by Trinkler Software AG (Switzerland).
// This file is part of Katal.
//
// Katal is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version <http://www.gnu.org/licenses/>.
//
// Katal is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

#![cfg_attr(not(feature = "std"), no_std)]
// The above line is needed to compile the Wasm binaries.

// Importing crates declared in the cargo.toml file.
use parity_codec::{Decode, Encode};
use primitives::H256;
use runtime_std::prelude::*;
use support::{decl_module, decl_storage, dispatch::Result, StorageValue};
use time::*;

// Importing the rest of the files in this crate.
mod add;
mod remove;
mod scheduled_event;
use add::*;
use remove::*;
use scheduled_event::*;

// This module's configuration trait.
pub trait Trait: system::Trait + actus::Trait {}

// This module's storage items.
decl_storage! {
    trait Store for Module<T: Trait> as SchedulerStorage {
        pub List: Vec<ScheduledEvent> = Vec::new();
        pub Counter: u32 = 0;
    }
}

// This module's dispatchable functions.
decl_module! {
    // The module declaration.
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {}
}
