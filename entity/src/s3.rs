//! S3FileStorage definition for handling file storage operations via AWS S3.

use aws_sdk_s3::Client;

/// Struct representing a file storage backend using AWS S3.
#[derive(Debug, Clone)]
pub struct S3FileStorage {
    /// The name of the S3 bucket used for storage.
    pub bucket: String,

    /// The AWS S3 client instance used to interact with the S3 service.
    pub client: Client,
}
