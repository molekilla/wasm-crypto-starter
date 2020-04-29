# Cryptoxide

![Continuous integration](https://github.com/typed-io/cryptoxide/workflows/Continuous%20integration/badge.svg?branch=master)
![MIT or APACHE-2 licensed](https://img.shields.io/badge/licensed-MIT%20or%20APACHE--2-blue.svg)

A pure Rust implementation of various modern cryptographic algorithms, which has no dependencies
and no foreign code (specially C or assembly code). This is compatible with WASM and embedded devices.

This crates package aims to support as many architectures as possible with as
little dependencies as possible.

Disclaimer: There are no warranties in use as everything is cryptographically-related

## Fork information

This is a fork of [Rust-Crypto by DaGenix](https://github.com/DaGenix/rust-crypto), to
which we owe a debt of gratitude for starting some good quality pure Rust implementations
of various cryptographic algorithms.

Notable differences with the original sources:

* Maintained.
* Extended ED25519 support for extended secret key (64 bytes) support.
* Proper implementation of ChaChaPoly1305 (according to spec).
* Many cryptographic algorithms removed: AES, Blowfish, Fortuna, RC4, RIPEMD160, Whirlpool, MD5, SHA1.

## Running benches

normally:

    cargo +nightly bench --features with-bench

or with all the cpu capability enabled:

    RUSTFLAGS="-C target_cpu=native" cargo +nightly bench --features with-bench

## supported compiler versions

| Rust    | `test` |
| ------- | :----: |
| stable  |   ✓    |
| beta    |   ✓    |
| nightly |   ✓    |

We will always aim to support the current stable version. However, it is
likely that an older version of the Rust compiler is also supported.

# License

This project is licensed under either of the following licenses:

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
   http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   http://opensource.org/licenses/MIT)

Please choose the licence you want to use.
