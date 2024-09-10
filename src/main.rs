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
use tower_http::trace::TraceLayer;
use tracing_subscriber::{fmt, prelude::*};

/// The main entry point for the scrounch_backend application.
///
/// This function initializes the application and starts the Axum web server.
#[tokio::main]
async fn main() {
    // Update environnement variable from .env
    dotenvy::dotenv().ok();
    let cli = scrounch_backend::Arguments::parse();

    let tracing_is_enable = !cli.disable_tracing;
    if tracing_is_enable {
        let filter = tracing_subscriber::filter::Targets::new()
            .with_target("tower_http::trace::on_response", tracing::Level::TRACE)
            .with_target("tower_http::trace::on_request", tracing::Level::TRACE)
            .with_target("tower_http::trace::make_span", tracing::Level::DEBUG)
            .with_target("scrounch_backend", tracing::Level::INFO)
            .with_default(tracing::Level::INFO);

        let tracing_layer = fmt::layer();

        let env_filter = tracing_subscriber::EnvFilter::builder()
            .with_default_directive(tracing_subscriber::filter::LevelFilter::INFO.into())
            .from_env_lossy();

        tracing_subscriber::registry()
            .with(tracing_layer)
            .with(env_filter)
            .with(filter)
            .init();
    }

    let app = scrounch_backend::app()
        .await
        .layer(TraceLayer::new_for_http());

    let address: std::net::SocketAddr = cli.address.parse().expect(
        &format!(
            "Sorry but address: {} is not correctly formatted",
            cli.address
        )[..],
    );

    let listener = tokio::net::TcpListener::bind(address)
        .await
        .expect("Host address is not alvaible");

    tracing::info!("Server is starting on {address}");
    axum::serve(listener, app)
        .await
        .expect("Axum server couldn't start");
}
