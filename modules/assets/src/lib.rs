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

//! # Assets module
//!

#![cfg_attr(not(feature = "std"), no_std)]
// The above line is needed to compile the Wasm binaries.

// Importing crates declared in the cargo.toml file.
use parity_codec::{Decode, Encode};
use primitives::H256;
use structures::Real;
use support::{decl_module, decl_storage, dispatch::Result, StorageMap};

// Importing the rest of the files in this crate.
mod burn;
mod mint;
mod transfer;
use burn::*;
use mint::*;
use transfer::*;

// This module's configuration trait.
pub trait Trait: system::Trait {}

// This module's storage items.
decl_storage! {
    trait Store for Module<T: Trait> as AssetsStorage {
        pub AssetsBalances: map (u32, H256) => Real;
        pub AssetsSupply: map u32 => Real;
    }
}

// This module's dispatchable functions.
decl_module! {
    // The module declaration.
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {

        pub fn dispatch_transfer(origin,from_address: H256, to_address: H256, asset_id: u32, amount: Real) -> Result {
            // Call corresponding internal function.
            Self::transfer(from_address, to_address, asset_id, amount)?;

            // Return Ok if successful.
            Ok(())
        }

        pub fn dispatch_mint(origin, to_address: H256, asset_id: u32, amount: Real) -> Result {
            // Call corresponding internal function.
            Self::mint(to_address, asset_id, amount)?;

            // Return Ok if successful.
            Ok(())
        }

        pub fn dispatch_burn(origin,from_address: H256, asset_id: u32, amount: Real) -> Result {
            // Call corresponding internal function.
            Self::burn(from_address, asset_id, amount)?;

            // Return Ok if successful.
            Ok(())
        }
    }
}
