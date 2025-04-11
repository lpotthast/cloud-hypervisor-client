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
//! For more examples check out the [examples](https://github.com/lpotthast/cloud-hypervisor-client) folder in the Git repository.
//!
//!# Selecting TLS implementation
//!
//!The underlying TLS implementation for `reqwest` can be selected using [Cargo features](https://doc.rust-lang.org/stable/cargo/reference/manifest.html#the-features-section):
//!- **default-tls** *(enabled by default)*: Provides TLS support to connect over HTTPS.
//!- **native-tls**: Enables TLS functionality provided by `native-tls`.
//!- **native-tls-vendored**: Enables the `vendored` feature of `native-tls`.
//!- **rustls-tls**: Enables TLS functionality provided by `rustls`.
//!
//!(Refer to [Optional Features](https://docs.rs/reqwest/latest/reqwest/#optional-features) in the `reqwest` documentation.)
//!
//!Example for using the TLS functionality provided by `rustls`:
//!```toml
//![dependencies]
//!cloud_hypervisor_client = { version = "*", default-features = false, features = ["rustls-tls"] }
//!```

#![allow(unused_imports)]
#![allow(clippy::too_many_arguments)]

extern crate futures;
extern crate hyper;
extern crate serde;
extern crate serde_json;
extern crate serde_repr;
extern crate url;

use crate::apis::configuration::Configuration;
use crate::apis::DefaultApiClient;
use std::path::Path;
use std::sync::Arc;

pub mod apis;
pub mod models;

pub type SocketBasedApiClient = DefaultApiClient<hyperlocal::UnixConnector>;

pub fn socket_based_api_client(vmm_socket_path: impl AsRef<Path>) -> SocketBasedApiClient {
    use hyperlocal::UnixClientExt;

    let uri: hyper::Uri = hyperlocal::Uri::new(vmm_socket_path, "/api/v1").into();
    let client = hyper_util::client::legacy::Client::unix();
    let configuration = Configuration {
        base_path: uri.to_string(),
        user_agent: None,
        client,
        basic_auth: None,
        oauth_access_token: None,
        api_key: None,
    };
    DefaultApiClient::new(Arc::new(configuration))
}
