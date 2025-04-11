# cloud-hypervisor-client for Rust

Unofficial Rust crate for interacting with
the [cloud-hypervisor REST API](https://github.com/cloud-hypervisor/cloud-hypervisor/blob/main/docs/api.md)

## Overview

The `cloud-hypervisor-client` crate can be used for managing the endpoints provided by a cloud-hypervisor socket in your
Rust project.

The API client code of this crate has been auto-generated from
the [OpenAPI description for the cloud-hypervisor REST API](https://raw.githubusercontent.com/cloud-hypervisor/cloud-hypervisor/master/vmm/src/api/openapi/cloud-hypervisor.yaml)
using [OpenAPI Generator](https://openapi-generator.tech/).

## Example

A very basic example for listing all existing servers:

```rust
use cloud_hypervisor_client::apis::configuration::Configuration;
use cloud_hypervisor_client::apis::default_api::vm_info_get;

#[tokio::main]
async fn main() -> Result<(), String> {
    let configuration = Configuration::new();

    let vm_info = vm_info_get(&configuration)
        .await
        .map_err(|err| format!("API call to vm_info_get failed: {:?}", err))?;

    println!("Received vm info: {vm_info:?}");

    Ok(())
}
```

For more examples check out the [examples](https://github.com/HenningHolmDE/hcloud-rust/tree/master/examples) folder in
the Git repository.

## Selecting a TLS implementation

The underlying TLS implementation for `reqwest` can be selected
using [Cargo features](https://doc.rust-lang.org/stable/cargo/reference/manifest.html#the-features-section):

- **default-tls** *(enabled by default)*: Provides TLS support to connect over HTTPS.
- **native-tls**: Enables TLS functionality provided by `native-tls`.
- **native-tls-vendored**: Enables the `vendored` feature of `native-tls`.
- **rustls-tls**: Enables TLS functionality provided by `rustls`.

(Refer to [Optional Features](https://docs.rs/reqwest/latest/reqwest/#optional-features) in the `reqwest`
documentation.)

Example for using the TLS functionality provided by `rustls`:

```toml
[dependencies]
cloud_hypervisor_client = { version = "*", default-features = false, features = ["rustls-tls"] }
```

## Attributions

This crate was based on the great work done in: https://github.com/HenningHolmDE/hcloud-rust.
