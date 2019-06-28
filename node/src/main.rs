//! Substrate Node Template CLI library.

#![warn(missing_docs)]
#![warn(unused_extern_crates)]

mod chain_spec;
mod cli;
mod service;

pub use substrate_cli::{error, IntoExit, VersionInfo};

fn main() {
    let version = VersionInfo {
        name: "Katal Chain",
        commit: env!("VERGEN_SHA_SHORT"),
        version: env!("CARGO_PKG_VERSION"),
        executable_name: "katal",
        author: "brunoffranca, sophieraderm, retotrinkler",
        description: "Standard Framework for Finance.",
        support_url: "https://github.com/Trinkler/katal-chain/issues",
    };

    if let Err(e) = cli::run(::std::env::args(), cli::Exit, version) {
        eprintln!("Error starting the node: {}\n\n{:?}", e, e);
        std::process::exit(1)
    }
}
