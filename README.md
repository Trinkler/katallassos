[<img src = "https://raw.githubusercontent.com/Trinkler/brand/master/assets/katal/banner.png" width = "100%">](https://katal.io)

# katal-chain [![buddy pipeline](https://app.buddy.works/trinkler/katal-chain/pipelines/pipeline/195629/badge.svg?token=3a967ff05891e3690d97195573654d05994285b9798ed78a42d7178be77fa4c3 "buddy pipeline")](https://app.buddy.works/trinkler/katal-chain/pipelines/pipeline/195629)

> [Katal](https://katal.io): Standard Framework for Finance.

### Usage

- [Read Research Papers](https://github.com/Trinkler/katal-research)
- [Read Reference Docs](https://katal.io/docs)
- [Subscribe to Newsletter](https://software.us19.list-manage.com/subscribe?u=48964a7b4b5e9480604838bf2&id=982968577c)
- [Join Telegram](https://t.me/katalchain)
- [Join Reddit](https://www.reddit.com/r/katalchain)

### Initial Setup

```sh
# Install Rustup
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
git clone git@github.com:Trinkler/katal-chain.git
cd katal-chain/
./scripts/init.sh
```

### Building

```sh
./scripts/build.sh # Build wasm binaries
cargo build --release # Build native binaries
```

### Run

```sh
./target/release/katal
```
