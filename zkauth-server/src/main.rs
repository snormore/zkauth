// Enforce documentation for all public items in the crate.
#![warn(missing_docs)]

//! Server for the zkauth authentication protocol.
//!
//! The server listens for incoming client requests and processes them using the zkauth protocol.
//! It uses the `tonic` library to provide a gRPC interface for the client to communicate with the
//! server.
//!
//! The server can be configured using command line options, and can also generate a configuration
//! file for use with the server. The configuration file specifies the public parameters for the
//! zkauth protocol, such as the prime number and generator for the discrete logarithm protocol, or
//! the base points for the elliptic curve protocol.
//!
//! # Usage
//!
//! The server can be run with the following command:
//!
//! ```sh
//! zkauth-server
//! ```
//!
//! The server can be configured using the following command line options:
//!
//! ```sh
//! zkauth-server --help
//! ```
//!
//! The server can also generate a configuration file using the following command:
//!
//! ```sh
//! zkauth-server --config-generate --config-path=config.json
//! ```
//!
//! You can specify the configuration flavor using the `--config-flavor` option, and the number of
//! bits for the prime number using the `--config-prime-bits` option, or specify a prime number
//! directly using the `--config-prime` option.
//!
//! ```sh
//! zkauth-server --config-generate --config-path=config.json --config-flavor=elliptic-curve
//! ```
//!
//! ```sh
//! zkauth-server --config-generate --config-path=config.json --config-prime-bits=256
//! ```
//!
//! ```sh
//! zkauth-server --config-generate --config-path=config.json --config-prime=42765216643065397982265462252423826320512529931694366715111734768493812630447
//! ```

use anyhow::Result;
use clap::Parser;
use zkauth_server::cli::{run, Options};

/// Main entry point for the server.
#[tokio::main]
async fn main() -> Result<()> {
    let opts = Options::parse();

    run(opts).await
}
