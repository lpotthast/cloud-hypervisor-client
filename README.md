# cloud-hypervisor-client

Unofficial Rust crate for interacting with
the [cloud-hypervisor REST API](https://github.com/cloud-hypervisor/cloud-hypervisor/blob/main/docs/api.md)

## Overview

The `cloud-hypervisor-client` crate can be used for managing the endpoints provided by a cloud-hypervisor socket in your
Rust project.

The API client code of this crate has been auto-generated from
the [OpenAPI description for the cloud-hypervisor REST API](https://raw.githubusercontent.com/cloud-hypervisor/cloud-hypervisor/master/vmm/src/api/openapi/cloud-hypervisor.yaml)
using [OpenAPI Generator](https://openapi-generator.tech/).

## Installation

Add the crate to your project with Cargo:

```sh
cargo add cloud-hypervisor-client
```

Or add it to your `Cargo.toml` manually:

```toml
[dependencies]
cloud-hypervisor-client = "0.4"
```

The crate targets a local cloud-hypervisor VMM over a Unix domain socket via `hyper` and `hyperlocal`, so it
only works on Unix-like systems.

## Example

A very basic example for listing all existing servers:

```rust
use cloud_hypervisor_client::apis::DefaultApi;
use cloud_hypervisor_client::socket_based_api_client;

#[tokio::main]
async fn main() -> Result<(), String> {
    let client = socket_based_api_client("cloud_hypervisor_vm_socket.sock");

    let vm_info = client
        .vm_info_get()
        .await
        .map_err(|err| format!("API call to vm_info_get failed: {:?}", err))?;

    println!("Received vm info: {vm_info:?}");

    Ok(())
}
```

For more examples check out the [examples](https://github.com/HenningHolmDE/hcloud-rust/tree/master/examples) folder in
the Git repository.

## Minimum Supported Rust Version (MSRV)

This crate's MSRV is `1.81.0`.

## Attributions

This crate was based on the great work done in: https://github.com/HenningHolmDE/hcloud-rust.
