#![warn(missing_docs)]
//! scrounch_backend - Main entry point
//!
//! This file contains the entry point for the `scrounch_backend` application.
//! It initializes the web server, binds it to the specified address and port,
//! and launches the application using the Axum framework.
//!
//! The main components handled in this file include:
//! - Setting up the TCP listener for incoming HTTP requests.
//! - Binding the listener to the Axum application defined in the `scrounch_backend::app()` function.
//! - Running the server asynchronously using the Tokio runtime.
//!
//! # Notes
//! - The server listens on `0.0.0.0:3000` by default, which makes it accessible on all network interfaces.
//! - Ensure that the host address is available before launching the server, as any conflicts will result in a panic.

use clap::Parser;

/// The main entry point for the scrounch_backend application.
///
/// This function initializes the application and starts the Axum web server.
#[tokio::main]
async fn main() {
    // Update environnement variable from .env
    dotenvy::dotenv().ok();
    let cli = scrounch_backend::Arguments::parse();

    let app = scrounch_backend::app().await;

    let address: std::net::SocketAddr = cli.address.parse().expect(
        &format!(
            "Sorry but address: {} is not correctly formatted",
            cli.address
        )[..],
    );

    let listener = tokio::net::TcpListener::bind(address)
        .await
        .expect("Host address is not alvaible");

    axum::serve(listener, app)
        .await
        .expect("Axum server couldn't start");
}
