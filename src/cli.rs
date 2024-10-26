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

    ///  The base URL of the frontend application, typically used for CORS and redirection purposes.
    /// Example: https://app.example.com
    #[arg(env, long)]
    pub frontend_url: String,

    ///  The base URL of the backend application, used for constructing API endpoints.
    /// Example: https://app.example.com/api
    #[arg(env, long)]
    pub backend_url: String,

    ///  The URL of the OpenID provider (issuer) for authentication purposes.
    /// Example: https://auth.example.com/realms/master
    #[arg(env, long)]
    pub openid_issuer: String,

    ///  The client ID registered with the OpenID provider, used for authentication.
    #[arg(env, long)]
    pub openid_client_id: String,

    ///  The client secret registered with the OpenID provider, if applicable.
    #[arg(env, long)]
    pub openid_client_secret: Option<String>,

    /// The URL of the database the application connects to, typically in the format of a connection string
    /// Example:
    /// - postgresql://${POSTGRES_USER}:${POSTGRES_PASSWORD}@${POSTGRES_HOST}:${POSTGRES_PORT}/${POSTGRES_DB}
    #[arg(env, long)]
    pub database_url: String,

    #[cfg(feature = "cache")]
    /// The URL of the caching server (redis/valkey)
    /// Can be 0, 1 or more
    /// Example
    /// redis://${HOST}:${PORT}/
    /// redis://${USERNAME}:${PASSWORD}@${HOST}:${PORT}/
    /// Please take a look at: [from_url](fred::types::RedisConfig#method.from_url) for more information
    #[arg(env, long)]
    pub cache_url: Option<String>,

    /// The AWS / Minio access key ID used to authenticate API requests to AWS services.
    #[arg(env, long)]
    pub aws_access_key_id: String,

    /// The AWS / Minio secret access key used to authenticate API requests to AWS services.
    #[arg(env, long)]
    pub aws_secret_access_key: String,

    /// The AWS region in which your resources (such as S3 buckets) are hosted.
    /// Example: us-east-1
    #[arg(env, long, default_value = "eu-central-1")]
    pub aws_region: String,

    /// The custom AWS endpoint URL, typically used for local testing with services
    /// like LocalStack.
    /// Example: http://localhost:4566
    #[arg(env, long)]
    pub aws_endpoint_url: String,

    /// The name of the AWS S3 bucket where files will be stored.
    /// This bucket must already exist.
    #[arg(env, long)]
    pub aws_s3_bucket: String,

    /// The base URL of the Sma beverage website, used for importing existing products.
    /// Example: https://api.sma.example.com
    #[arg(env, long)]
    pub sma_url: Option<String>,

    /// The API key required for authenticating requests to the Sma API.
    #[arg(env, long)]
    pub sma_api_key: Option<String>,

    /// A list of categories id to filter products when importing from the Sma API.
    /// If not provided, all product categories will be imported.
    #[arg(env, long, value_delimiter = ' ', num_args = 1..)]
    pub sma_categories: Option<Vec<String>>,
}
