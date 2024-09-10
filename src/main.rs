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

/// The main entry point for the scrounch_backend application.
///
/// This function initializes the application and starts the Axum web server.
/// It sets up the TCP listener on the specified host and port (`0.0.0.0:3000`),
/// binds the Axum application created by `scrounch_backend::app()`, and runs
/// the server asynchronously using Tokio.
#[tokio::main]
async fn main() {
    let app = scrounch_backend::app().await;

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("Host address is not alvaible");

    axum::serve(listener, app)
        .await
        .expect("Axum server couldn't start");
}
