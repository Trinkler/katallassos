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

use hex_literal::hex;

pub fn get_testnet_bootnodes() -> Vec<String> {
    return vec![
        "/ip4/134.209.111.205/tcp/30333/p2p/QmUn4Mz3vA4DZD6XUG667yRxcUF35pGBLmg4PCNo8tNuKT"
            .to_string(),
        "/ip4/157.245.46.255/tcp/30333/p2p/QmYYfe4n7BKjfbpMPjr8HnzKxEefrDXU8pqpnYEUWUM2FR"
            .to_string(),
    ];
}
