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

// For each level of submodule, you should have a file like this. It must be named 'mod.rs'
// and it should only be used to do imports.

// This line imports the parent of this file. Because of the structure that we are using,
// this allows the children of this file to use any function or type defined in the rest
// of the module.
use super::*;

// This imports the children of this files.
mod another_function;
mod another_struct;

// This exports the children to the parent of this file, making them available to the
// rest of the module. Note the use of the command 'pub'.
pub use another_function::*;
pub use another_struct::*;
