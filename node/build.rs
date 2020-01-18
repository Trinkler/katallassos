// Copyright 2020 by Trinkler Software AG (Switzerland).
// This file is part of Katal Chain.
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

use vergen::{generate_cargo_keys, ConstantsFlags};

const ERROR_MSG: &str = "Failed to generate metadata files";

fn main() {
    generate_cargo_keys(ConstantsFlags::SHA_SHORT).expect(ERROR_MSG);

    build_script_utils::rerun_if_git_head_changed();
}
