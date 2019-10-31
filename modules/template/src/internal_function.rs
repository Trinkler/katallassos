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
// Each internal function should be on its own separate file.

// This line is necessary to import the parent file, and indirectly import
// all files in the module.
use super::*;

// This is an internal function. It looks like a normal Rust function except that it's inside
// an "impl<T: Trait> Module<T>" block. An internal function can access the storage and the
// functions (dispatchable, internal or pure) of this or any other module. However, they
// can't be called by pure functions.
impl<T: Trait> Module<T> {
    pub fn internal_function() -> Result {
        // Call a pure function.
        let number = pure_function();

        // Store value.
        <SomeValue<T>>::put(number);

        // Return Ok.
        Ok(())
    }
}
