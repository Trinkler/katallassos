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

// Each struct (and its child structs and enums) should be on its own separate file.

// This line is necessary to import the parent file, and indirectly import
// all files in the module.
use super::*;

// Structs work just like in Rust. The only difference is that the two next lines always
// need to prefix the definition of a struct. Also, don't use Substrate types.
#[derive(Clone, Copy, Decode, Encode, Default, PartialEq)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct TemplateState {
    pub field1: i16,
    pub field2: u128,
    pub field3: Option<SomeEnum>,
}

// Enums are just like the structs, but not that in the first line different traits
// are derived. That's because enums can't derive the trait Default.
#[derive(Clone, Copy, Decode, Encode, PartialEq)]
#[cfg_attr(feature = "std", derive(Debug))]
pub enum SomeEnum {
    A,
    B,
    C,
}
