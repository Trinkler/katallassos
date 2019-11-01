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

// Each pure function should be on its own separate file.

// This line is necessary to import the parent file, and indirectly import
// all files in the module.
use super::*;

// This is a pure function. It is a normal Rust function. They can't access the storage, the
// dispatchable functions or the internal functions of this or any other module. They can
// only call other pure function in this or other modules. However, they can be called by
// any other function.
pub fn pure_function() -> u32 {
    23
}
