//! Katal Chain CLI library.

#![warn(missing_docs)]
#![warn(unused_extern_crates)]

mod chain_spec;
mod cli;
mod service;

pub use substrate_cli::{error, IntoExit, VersionInfo};

fn run() -> cli::error::Result<()> {
    let version = VersionInfo {
        name: "Katal Chain",
        commit: env!("VERGEN_SHA_SHORT"),
        version: env!("CARGO_PKG_VERSION"),
        executable_name: "katalchain",
        author: "brunoffranca, sophieraderm, retotrinkler",
        description: "Cryptocurrency Movement, Standardized.",
        support_url: "https://github.com/katalchain/blockchain/issues/new",
    };
    cli::run(::std::env::args(), cli::Exit, version)
}

error_chain::quick_main!(run);
