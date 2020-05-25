//! Client configuration.

use crate::tls::TlsOptions;
use http02::Uri;

/// A k8s client configuration.
///
/// This type is designed to hold all possible variants of the configuration.
/// It also abstracts the client from the various ways to obtain the
/// configuration.
///
/// The implementation is fairly limited, and only covers the use cases we
/// support.
#[derive(Debug, Clone)]
pub struct Config {
    /// The base URL to use when constructing HTTP requests to the k8s API
    /// server.
    pub base: Uri,

    /// The bearer token to use at the `Authorization` header.
    pub token: String,

    /// The TLS configuration parameters to use at the HTTP client.
    pub tls_options: TlsOptions,
}

pub mod in_cluster;
