//! Command-line interface (CLI) utilities.
//!
//! This module defines utilities and functions for handling command-line
//! arguments and options for the `scrounch_backend` application. It allows
//! users to run the application with various configurations or commands via
//! the CLI.
//!
//! # Features
//! - Parsing and handling command-line arguments.
//! - Providing options for different modes of operation (e.g., development, production).

use clap::Parser;

/// Command-line arguments for the `scrounch_backend` application.
///
/// This struct defines the command-line arguments that can be provided when
/// running the application. It uses the `clap` crate to parse and validate
/// the input arguments.
#[derive(Parser, Debug, Default, Clone)]
#[command(version, about, long_about = None)]
pub struct Arguments {
    /// The address and port on which the server will listen for
    /// incoming connections. It defaults to `"0.0.0.0:3000"` if not specified
    /// via the command-line or environment variable.
    #[arg(env, default_value = "0.0.0.0:3000")]
    pub address: String,

    /// When set, disables tracing/logging functionality in the application. Defaults to `false` if not provided.
    #[arg(env, long, default_value_t = false)]
    pub disable_tracing: bool,
}
