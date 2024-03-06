// Enforce documentation for all public items in the crate.
#![warn(missing_docs)]

//! The zkauth client library.
//!
//! This library provides the client for the zkauth service, acting as the prover for the zero-
//! knowledge authentication protocol.

/// The client module.
pub mod client;

/// The command-line interface module.
pub mod cli;

#[doc(inline)]
pub use zkauth_protobuf::v1::auth_client::AuthClient;

#[cfg(test)]
mod test;
