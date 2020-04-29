## WASM Crypto Starter for XDV platform apps
### Vue based template
### 0.1.0

### Install

#### Prerequisites

* `rustup`
* `node`
* `wasm-pack`

#### Setup

1. Clone repo
2. `npm i`


### How to install Rust libraries with WASI support

> This is a manual guide until a tool is created

1. Install `cargo install cargo-wasi` and `curl https://wasmtime.dev/install.sh -sSf | bash`
2. `cd wasi_modules`
3. Clone repo
4. Run `cargo wasi build --lib`
5. Run `cargo wasi check --lib`
6. Run `cargo wasi bench`

Library should be ready to use with AssemblyScript


### Included libraries

* [yubihsm.rs](https://github.com/iqlusioninc/yubihsm.rs)
* [libsecp256k1](https://github.com/paritytech/libsecp256k1)
* [signatory](https://github.com/iqlusioninc/signatory)
* [cryptoxide](https://github.com/typed-io/cryptoxide)
* [rust-crypto-wasm](https://github.com/buttercup/rust-crypto-wasm)

### XDV Architecture specification

[Architecture Document](https://swarm-gateways.net/bzz:/d5b9063eea355591cf25f875cf93629b6b6a271020fd5b555ad73dd56346d698/#/)