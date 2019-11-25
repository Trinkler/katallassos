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

//! Katal Chain Blockchain CLI library.

#![warn(missing_docs)]
#![warn(unused_extern_crates)]

mod chain_spec;
#[macro_use]
mod service;
mod cli;

pub use substrate_cli::{error, IntoExit, VersionInfo};

fn main() -> Result<(), cli::error::Error> {
    let version = VersionInfo {
        name: "Katal Chain Blockchain",
        commit: env!("VERGEN_SHA_SHORT"),
        version: env!("CARGO_PKG_VERSION"),
        executable_name: "katalchain",
        author: "brunoffranca, sophieraderm, retotrinkler",
        description: "Cryptocurrency Movement, Standardized.",
        support_url: "https://github.com/katalchain/blockchain/issues/new",
    };

    cli::run(std::env::args(), cli::Exit, version)
}
