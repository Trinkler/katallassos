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

use aura_primitives::sr25519::AuthorityId as AuraId;
use grandpa_primitives::AuthorityId as GrandpaId;
use hex_literal::hex;
use primitives::{crypto::UncheckedInto, ed25519};
use runtime::AccountId;
use serde_json as json;

/// Arbitrary properties defined in chain spec as a JSON object
pub type Properties = json::map::Map<String, json::Value>;

pub fn get_staging_bootnodes() -> Vec<String> {
    return vec![
        "/dns4/bootnode-01.katalchain.com/tcp/30333/p2p/QmaY72HvoT8QfZ1xxSESPyCqPj6Umzx5537QCCQfmcrw2u"
            .to_string(),
        "/dns4/bootnode-02.katalchain.com/tcp/30333/p2p/QmYYfe4n7BKjfbpMPjr8HnzKxEefrDXU8pqpnYEUWUM2FR"
            .to_string(),
    ];
}

// AuraId is sr25519
// subkey -s inspect "$secret"
// GrandpaId is ed25519
// subkey -e inspect "$secret"
pub fn get_staging_initial_authorities() -> Vec<(AuraId, GrandpaId)> {
    return vec![
        (
            // 5GNa5NWbUnhHqDRcsvKRehfb1cxdskaECcmBjxniEgu5mqu5
            hex!["be9128704d6642083e4f9f5fc55e5216dc7b22cba74578c2a553b32391297530"]
                .unchecked_into(),
            // 5FnqauongW5TPgo8KKxmn75b7rr8NSWy9SARu54vkxag7Ncc
            hex!["a4d705ef67f4a1bc2e59ac97823e3793aaa559110f7d3a3e0f3594f6aebcb387"]
                .unchecked_into(),
        ),
        (
            // 5C7pGfLVJicEQjmhcR2Xovi8EoZoeBs4DS3nSs47QLdUaBHB
            hex!["025f53997ccc61bf0bfb51874d5c1837db3ed3d6e267693c0858e359679c3751"]
                .unchecked_into(),
            // 5DxUnqLBsAbxjK77ZZUxC8xYy8BwGDZcwpYyjYrf9ErDzNsD
            hex!["53b902907ed868912f67ea5c410f82da40591b5d83bdaed8ba46ca03dd7c4be3"]
                .unchecked_into(),
        ),
    ];
}

pub fn get_staging_endowed_accounts() -> Vec<AccountId> {
    return vec![
        hex!["be9128704d6642083e4f9f5fc55e5216dc7b22cba74578c2a553b32391297530"] // 5GNa5NWbUnhHqDRcsvKRehfb1cxdskaECcmBjxniEgu5mqu5
            .into(),
        hex!["025f53997ccc61bf0bfb51874d5c1837db3ed3d6e267693c0858e359679c3751"] // 5C7pGfLVJicEQjmhcR2Xovi8EoZoeBs4DS3nSs47QLdUaBHB
            .into(),
    ];
}

pub fn get_staging_root_key() -> AccountId {
    return hex!["be9128704d6642083e4f9f5fc55e5216dc7b22cba74578c2a553b32391297530"] // 5GNa5NWbUnhHqDRcsvKRehfb1cxdskaECcmBjxniEgu5mqu5
        .into();
}

pub fn get_chain_properties() -> Option<Properties> {
    let data = r#"
    {
        "ss58Format": 7,
        "tokenDecimals": 9,
        "tokenSymbol": "XTL"
    }"#;
    return serde_json::from_str(data).unwrap();
}
