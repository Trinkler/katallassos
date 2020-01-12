[<img src = "https://raw.githubusercontent.com/katalchain/brand/master/assets/banner/banner.png" width = "100%">](https://katalchain.com)

# blockchain

> [Katal Chain](https://katalchain.com) is a domain-specific blockchain creating a deterministic financial paradigm.
>
> Katal Chain implements a [classification](https://www.actusfrf.org/taxonomy) of _contract types_ which are mutual agreements between counterparties to exchange cash flows. Nearly every [financial instrument](https://en.wikipedia.org/wiki/Financial_instrument) can be broken down into contract types.
>
> Katal Chain is based on [Substrate](https://github.com/katalchain/substrate) and aims to connect to the multichain framework [Polkadot](https://polkadot.network) to interact with assets
> of [other connected blockchains](https://forum.web3.foundation/t/teams-building-on-polkadot/67) as well as to allow other connected blockchains to have access to contract types built using Katal Chain. View on [Telemetry](https://telemetry.polkadot.io/#list/Katal%20Chain).

### Setup

```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh # Install Rustup
git clone git@github.com:katalchain/blockchain.git
cd blockchain/
./scripts/init.sh # Initialize WASM build environment
```

### Develop

```sh
cargo build # Build native and wasm binaries in debug mode
./target/debug/katalchain purge-chain -y --dev # Remove the whole chain data
./target/debug/katalchain --dev # Run chain in dev mode
```

### Install

```sh
cargo install --locked --path . --force # Build and install native binaries
katalchain # Synchronize testnet chain data
```

### Usage

To access the [Katal Chain Testnet](https://telemetry.polkadot.io/#list/Katal%20Chain) using the great [Polkadot JS Apps Interface](https://polkadot.js.org/apps/#/explorer) do the following:

1.  In [Settings](https://polkadot.js.org/apps/#/settings) tab under the `General` section select `wss://endpoint.katalchain.com` as remote endpoint.
2.  In [Settings](https://polkadot.js.org/apps/#/settings) tab under the `Developer` section copy paste the [custom types definitions](./interface/types.json) into the interface and click the "Save" button.

All done you are now able to for example deploy a contract under the [Extrinsics](https://polkadot.js.org/apps/#/extrinsics) tab using the `contracts` module. If you don't feel like going through all the attributes yourself then you might also want to have a look at the Javascript [example](./interface/example.js).

### Custom Modules

-   [Contracts](https://github.com/katalchain/blockchain/tree/master/modules/contracts)
-   [Assets](https://github.com/katalchain/blockchain/tree/master/modules/assets)
-   [Structures](https://github.com/katalchain/blockchain/tree/master/modules/structures)
    -   [Safe fixed-point arithmetic](https://github.com/katalchain/blockchain/tree/master/modules/structures/src/reals.rs)
    -   [Time in ISO8601 format](https://github.com/katalchain/blockchain/tree/master/modules/structures/src/time.rs)
    -   [Priority queue using a binary heap](https://github.com/katalchain/blockchain/tree/master/modules/structures/src/min_heap.rs)
-   [Oracle](https://github.com/katalchain/blockchain/tree/master/modules/oracle)

### Additional Resources

-   [Subscribe to Newsletter](https://software.us19.list-manage.com/subscribe?u=48964a7b4b5e9480604838bf2&id=982968577c)
-   [Follow on Twitter](https://twitter.com/katalchain)
-   [Follow on Medium](https://medium.com/@katalchain)
-   [Join on Telegram](https://t.me/katalchain)
