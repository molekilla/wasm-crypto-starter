//! Signatory ECDSA and Ed25519 provider for *ring*

#![no_std]
#![forbid(unsafe_code)]
#![warn(missing_docs, rust_2018_idioms, unused_qualifications)]
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/iqlusioninc/signatory/develop/img/signatory-rustacean.png",
    html_root_url = "https://docs.rs/signatory-ring/0.19.0"
)]

#[cfg(test)]
#[macro_use]
extern crate std;

#[cfg(all(test, feature = "ed25519"))]
#[macro_use]
extern crate signatory;

/// ECDSA signing and verification support
#[cfg(feature = "ecdsa")]
pub mod ecdsa;

/// Ed25519 signing and verification support
#[cfg(feature = "ed25519")]
pub mod ed25519;
