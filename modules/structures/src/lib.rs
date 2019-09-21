//
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
//

//! # Structures module
//!
//! This module implements several composite data types, or _structures_, necessary for Katal's
//! runtime. The data types are:
//! 1. **Reals**: does safe fixed-point arithmetic.
//! 2. **Time**: represents time in the ISO8601 format.
//! 3. **MinHeap**: implements a priority queue using a binary heap.

#![cfg_attr(not(feature = "std"), no_std)]
// The above line is needed to compile the Wasm binaries.

// These are necessary to work with Substrate.
use parity_codec::{Decode, Encode};
// These are necessary to do operator overloading.
use core::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

mod min_heap;
mod reals;
mod time;

pub use min_heap::*;
pub use reals::*;
pub use time::*;
