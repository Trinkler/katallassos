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

use crate::testnet_fixtures::*;
use hex_literal::hex;
use katalchain_runtime::{
    AccountId, BalancesConfig, ConsensusConfig, GenesisConfig, IndicesConfig, SudoConfig,
    TimestampConfig,
};
use primitives::{crypto::UncheckedInto, ed25519, sr25519, Pair};
use substrate_service;
use substrate_telemetry::TelemetryEndpoints;

const STAGING_TELEMETRY_URL: &str = "wss://telemetry.polkadot.io/submit/";

use ed25519::Public as AuthorityId;

// Note this is the URL for the telemetry server
//const STAGING_TELEMETRY_URL: &str = "wss://telemetry.polkadot.io/submit/";

/// Specialized `ChainSpec`. This is a specialization of the general Substrate ChainSpec type.
pub type ChainSpec = substrate_service::ChainSpec<GenesisConfig>;

/// The chain specification option. This is expected to come in from the CLI and
/// is little more than one of a number of alternatives which can easily be converted
/// from a string (`--chain=...`) into a `ChainSpec`.
#[derive(Clone, Debug)]
pub enum Alternative {
    /// Whatever the current runtime is, with just Alice as an auth.
    Development,
    /// Whatever the current runtime is, with simple Alice/Bob auths.
    LocalTestnet,
    /// Hosted testnet with auto-generated genesis block
    StagingTestnet,
    /// Hosted testnet with unified genesis block and non-standard Validators.
    Testnet,
}

fn authority_key(s: &str) -> AuthorityId {
    ed25519::Pair::from_string(&format!("//{}", s), None)
        .expect("static values are valid; qed")
        .public()
}

fn account_key(s: &str) -> AccountId {
    sr25519::Pair::from_string(&format!("//{}", s), None)
        .expect("static values are valid; qed")
        .public()
}

impl Alternative {
    /// Get an actual chain config from one of the alternatives.
    pub(crate) fn load(self) -> Result<ChainSpec, String> {
        Ok(match self {
            Alternative::Development => ChainSpec::from_genesis(
                "Development",
                "dev",
                || {
                    testnet_genesis(
                        vec![authority_key("Alice")],
                        vec![account_key("Alice")],
                        account_key("Alice"),
                    )
                },
                vec![],
                None,
                None,
                None,
                None,
            ),
            Alternative::LocalTestnet => ChainSpec::from_genesis(
                "Local Testnet",
                "local_testnet",
                || {
                    testnet_genesis(
                        vec![authority_key("Alice"), authority_key("Bob")],
                        vec![
                            account_key("Alice"),
                            account_key("Bob"),
                            account_key("Charlie"),
                            account_key("Dave"),
                            account_key("Eve"),
                            account_key("Ferdie"),
                        ],
                        account_key("Alice"),
                    )
                },
                vec![],
                None,
                None,
                None,
                None,
            ),
            Alternative::StagingTestnet => ChainSpec::from_genesis(
                "Katal Chain Staging", // Name
                "staging",             // Id
                || {
                    testnet_genesis(
                        get_testnet_initial_authorities(), // Initial Authorities
                        get_testnet_endowed_accounts(),    // Endowed Accounts
                        get_testnet_root_key(),
                    )
                }, // Constructor
                get_testnet_bootnodes(), // Boot Nodes
                Some(TelemetryEndpoints::new(vec![(
                    STAGING_TELEMETRY_URL.to_string(),
                    0,
                )])), // Telemetry Endpoints
                None,                  // Protocol Id
                None,                  // Consensus Engine
                get_chain_properties(),
            ),
            // TODO import from file when https://github.com/katalchain/blockchain/issues/97 resolved
            Alternative::Testnet => ChainSpec::from_genesis(
                "Katal Chain", // Name
                "testnet",     // Id
                || {
                    testnet_genesis(
                        get_testnet_initial_authorities(), // Initial Authorities
                        get_testnet_endowed_accounts(),    // Endowed Accounts
                        get_testnet_root_key(),
                    )
                }, // Constructor
                get_testnet_bootnodes(), // Boot Nodes
                Some(TelemetryEndpoints::new(vec![(
                    STAGING_TELEMETRY_URL.to_string(),
                    0,
                )])), // Telemetry Endpoints
                None,          // Protocol Id
                None,          // Consensus Engine
                get_chain_properties(),
            ),
        })
    }

    pub(crate) fn from(s: &str) -> Option<Self> {
        match s {
            "dev" => Some(Alternative::Development),
            "local" => Some(Alternative::LocalTestnet),
            "staging" => Some(Alternative::StagingTestnet),
            "" => Some(Alternative::Testnet),
            _ => None,
        }
    }
}

fn testnet_genesis(
    initial_authorities: Vec<AuthorityId>,
    endowed_accounts: Vec<AccountId>,
    root_key: AccountId,
) -> GenesisConfig {
    GenesisConfig {
		consensus: Some(ConsensusConfig {
			code: include_bytes!("../runtime/wasm/target/wasm32-unknown-unknown/release/katalchain_runtime_wasm.compact.wasm").to_vec(),
			authorities: initial_authorities.clone(),
		}),
		system: None,
		timestamp: Some(TimestampConfig {
			minimum_period: 2, // 4 second block time.
		}),
		indices: Some(IndicesConfig {
			ids: endowed_accounts.clone(),
		}),
		balances: Some(BalancesConfig {
			transaction_base_fee: 1,
			transaction_byte_fee: 0,
			existential_deposit: 500,
			transfer_fee: 0,
			creation_fee: 0,
			balances: endowed_accounts.iter().cloned().map(|k|(k, 1 << 60)).collect(),
			vesting: vec![],
		}),
		sudo: Some(SudoConfig {
			key: root_key,
		}),
	}
}
