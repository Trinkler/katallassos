use hex_literal::{hex, hex_impl};
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
    /// Hosted testnet with non-standard Validators.
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
            Alternative::Testnet => ChainSpec::from_genesis(
                "Katal",   // Name
                "testnet", // Id
                || {
                    testnet_genesis(
                        vec![hex![
                            "a4d705ef67f4a1bc2e59ac97823e3793aaa559110f7d3a3e0f3594f6aebcb387"
                        ] // 5FnqauongW5TPgo8KKxmn75b7rr8NSWy9SARu54vkxag7Ncc
                        .unchecked_into()], // Initial Authorities
                        vec![hex![
                            "be9128704d6642083e4f9f5fc55e5216dc7b22cba74578c2a553b32391297530"
                        ] // 5FnqauongW5TPgo8KKxmn75b7rr8NSWy9SARu54vkxag7Ncc
                        .unchecked_into()], // Endowed Accounts
                        hex!["be9128704d6642083e4f9f5fc55e5216dc7b22cba74578c2a553b32391297530"] // 5FnqauongW5TPgo8KKxmn75b7rr8NSWy9SARu54vkxag7Ncc
                            .unchecked_into(), // Root Key
                    )
                }, // Constructor
                vec![
					"/ip4/134.209.111.205/tcp/30333/p2p/Qmd2tEYAE9916Ep2ipVu69vReHoVpS29Gk8GaJNAUZsLyz".to_string(),
				], // Boot Nodes
                Some(TelemetryEndpoints::new(vec![(
                    STAGING_TELEMETRY_URL.to_string(),
                    0,
                )])), // Telemetry Endpoints
                None,      // Protocol Id
                None,      // Consensus Engine
                None,      // Properties
            ),
        })
    }

    pub(crate) fn from(s: &str) -> Option<Self> {
        match s {
            "dev" => Some(Alternative::Development),
            "local" => Some(Alternative::LocalTestnet),
            "" | "testnet" => Some(Alternative::Testnet),
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
			minimum_period: 1, // 10 second block time.
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
