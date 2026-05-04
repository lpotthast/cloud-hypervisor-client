//!Unofficial Rust crate for accessing the [cloud-hypervisor REST API](https://github.com/cloud-hypervisor/cloud-hypervisor/blob/main/docs/api.md)
//!
//!# Overview
//!
//!The `cloud-hypervisor-client` crate can be used for managing the endpoints provided by the cloud-hypervisor REST API socket in your Rust project.
//!
//!The API client code of this crate has been auto-generated from the [Official OpenAPI Description for the cloud-hypervisor REST API](https://raw.githubusercontent.com/cloud-hypervisor/cloud-hypervisor/master/vmm/src/api/openapi/cloud-hypervisor.yaml) using [OpenAPI Generator](https://openapi-generator.tech/).
//!
//!# Example
//!
//!Get information about a VM:
//!
//!```rust,no_run
//! use cloud_hypervisor_client::apis::DefaultApi;
//! use cloud_hypervisor_client::socket_based_api_client;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), String> {
//!     let client = socket_based_api_client("cloud_hypervisor_vm_socket.sock");
//!
//!     let vm_info = client.vm_info_get()
//!         .await
//!         .map_err(|err| format!("API call to vm_info_get failed: {:?}", err))?;
//!
//!     println!("Received vm info: {vm_info:?}");
//!
//!     Ok(())
//! }
//!```
//!
//! For more examples check out the [examples](https://github.com/lpotthast/cloud-hypervisor-client/tree/main/examples)
//! folder in the Git repository.

#![allow(unused_imports)]
#![allow(clippy::too_many_arguments)]

extern crate futures;
extern crate hyper;
extern crate serde;
extern crate serde_json;
extern crate serde_repr;
extern crate url;

use crate::apis::DefaultApiClient;
use crate::apis::configuration::Configuration;
use std::path::Path;
use std::sync::Arc;

pub mod apis;
pub mod models;

/// A [`DefaultApiClient`] wired up to talk to the cloud-hypervisor VMM over a Unix domain socket.
///
/// Construct one with [`socket_based_api_client`].
pub type SocketBasedApiClient = DefaultApiClient<hyperlocal::UnixConnector>;

/// Builds a client for the cloud-hypervisor REST API exposed at the given Unix domain socket path.
///
/// This is the main entrypoint of the crate. Cloud-hypervisor exposes its API over a local Unix
/// socket (typically configured via the VMM's `--api-socket` flag); pass that socket's path here
/// and the returned [`SocketBasedApiClient`] can drive any endpoint of the [`DefaultApi`] trait.
///
/// The `vmm_socket_path` is not opened here. The connection is established lazily by `hyper` on the
/// first request. A non-existent or unreachable socket therefore surfaces as an error from the
/// first API call rather than from this function.
///
/// [`DefaultApi`]: apis::DefaultApi
///
/// # Example
///
/// ```rust,no_run
/// use cloud_hypervisor_client::apis::DefaultApi;
/// use cloud_hypervisor_client::socket_based_api_client;
///
/// #[tokio::main]
/// async fn main() -> Result<(), String> {
///     let client = socket_based_api_client("cloud_hypervisor_vm_socket.sock");
///
///     let vm_info = client.vm_info_get()
///         .await
///         .map_err(|err| format!("API call to vm_info_get failed: {:?}", err))?;
///
///     println!("Received vm info: {vm_info:?}");
///
///     Ok(())
/// }
/// ```
pub fn socket_based_api_client(vmm_socket_path: impl AsRef<Path>) -> SocketBasedApiClient {
    use hyperlocal::UnixClientExt;

    let uri: hyper::Uri = hyperlocal::Uri::new(vmm_socket_path, "/api/v1").into();
    let client = hyper_util::client::legacy::Client::unix();
    let configuration = Configuration {
        base_path: uri.to_string(),
        user_agent: None,
        client,
    };
    DefaultApiClient::new(Arc::new(configuration))
}
