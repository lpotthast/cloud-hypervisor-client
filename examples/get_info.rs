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
