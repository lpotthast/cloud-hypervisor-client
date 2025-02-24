use cloud_hypervisor_client::apis::configuration::Configuration;
use cloud_hypervisor_client::apis::default_api::vm_info_get;

#[tokio::main]
async fn main() -> Result<(), String> {
    let configuration = Configuration {
        base_path: "cloud_hypervisor_vm_socket.sock".to_owned(),
        ..Default::default()
    };

    let vm_info = vm_info_get(&configuration)
        .await
        .map_err(|err| format!("API call to vm_info_get failed: {:?}", err))?;

    println!("Received vm info: {vm_info:?}");

    Ok(())
}
