/*
 * Cloud Hypervisor API
 *
 * Local HTTP based API for managing and inspecting a cloud-hypervisor virtual machine.
 *
 * The version of the OpenAPI document: 0.3.0
 *
 * Generated by: https://openapi-generator.tech
 */

use std::borrow::Borrow;
#[allow(unused_imports)]
use std::option::Option;
use std::pin::Pin;
use std::sync::Arc;

use futures::Future;
use hyper;
use hyper_util::client::legacy::connect::Connect;

use super::request as __internal_request;
use super::{configuration, Error};
use crate::models;

pub struct DefaultApiClient<C: Connect>
where
    C: Clone + std::marker::Send + Sync + 'static,
{
    configuration: Arc<configuration::Configuration<C>>,
}

impl<C: Connect> DefaultApiClient<C>
where
    C: Clone + std::marker::Send + Sync,
{
    pub fn new(configuration: Arc<configuration::Configuration<C>>) -> DefaultApiClient<C> {
        DefaultApiClient { configuration }
    }
}

pub trait DefaultApi: Send + Sync {
    fn boot_vm(&self) -> Pin<Box<dyn Future<Output = Result<(), Error>> + Send>>;
    fn create_vm(
        &self,
        vm_config: models::VmConfig,
    ) -> Pin<Box<dyn Future<Output = Result<(), Error>> + Send>>;
    fn delete_vm(&self) -> Pin<Box<dyn Future<Output = Result<(), Error>> + Send>>;
    fn pause_vm(&self) -> Pin<Box<dyn Future<Output = Result<(), Error>> + Send>>;
    fn power_button_vm(&self) -> Pin<Box<dyn Future<Output = Result<(), Error>> + Send>>;
    fn reboot_vm(&self) -> Pin<Box<dyn Future<Output = Result<(), Error>> + Send>>;
    fn resume_vm(&self) -> Pin<Box<dyn Future<Output = Result<(), Error>> + Send>>;
    fn shutdown_vm(&self) -> Pin<Box<dyn Future<Output = Result<(), Error>> + Send>>;
    fn shutdown_vmm(&self) -> Pin<Box<dyn Future<Output = Result<(), Error>> + Send>>;
    fn vm_add_device_put(
        &self,
        device_config: models::DeviceConfig,
    ) -> Pin<Box<dyn Future<Output = Result<models::PciDeviceInfo, Error>> + Send>>;
    fn vm_add_disk_put(
        &self,
        disk_config: models::DiskConfig,
    ) -> Pin<Box<dyn Future<Output = Result<models::PciDeviceInfo, Error>> + Send>>;
    fn vm_add_fs_put(
        &self,
        fs_config: models::FsConfig,
    ) -> Pin<Box<dyn Future<Output = Result<models::PciDeviceInfo, Error>> + Send>>;
    fn vm_add_net_put(
        &self,
        net_config: models::NetConfig,
    ) -> Pin<Box<dyn Future<Output = Result<models::PciDeviceInfo, Error>> + Send>>;
    fn vm_add_pmem_put(
        &self,
        pmem_config: models::PmemConfig,
    ) -> Pin<Box<dyn Future<Output = Result<models::PciDeviceInfo, Error>> + Send>>;
    fn vm_add_user_device_put(
        &self,
        vm_add_user_device: models::VmAddUserDevice,
    ) -> Pin<Box<dyn Future<Output = Result<models::PciDeviceInfo, Error>> + Send>>;
    fn vm_add_vdpa_put(
        &self,
        vdpa_config: models::VdpaConfig,
    ) -> Pin<Box<dyn Future<Output = Result<models::PciDeviceInfo, Error>> + Send>>;
    fn vm_add_vsock_put(
        &self,
        vsock_config: models::VsockConfig,
    ) -> Pin<Box<dyn Future<Output = Result<models::PciDeviceInfo, Error>> + Send>>;
    fn vm_coredump_put(
        &self,
        vm_coredump_data: models::VmCoredumpData,
    ) -> Pin<Box<dyn Future<Output = Result<(), Error>> + Send>>;
    fn vm_counters_get(
        &self,
    ) -> Pin<
        Box<
            dyn Future<
                    Output = Result<
                        std::collections::HashMap<String, std::collections::HashMap<String, i64>>,
                        Error,
                    >,
                > + Send,
        >,
    >;
    fn vm_info_get(&self) -> Pin<Box<dyn Future<Output = Result<models::VmInfo, Error>> + Send>>;
    fn vm_receive_migration_put(
        &self,
        receive_migration_data: models::ReceiveMigrationData,
    ) -> Pin<Box<dyn Future<Output = Result<(), Error>> + Send>>;
    fn vm_remove_device_put(
        &self,
        vm_remove_device: models::VmRemoveDevice,
    ) -> Pin<Box<dyn Future<Output = Result<(), Error>> + Send>>;
    fn vm_resize_put(
        &self,
        vm_resize: models::VmResize,
    ) -> Pin<Box<dyn Future<Output = Result<(), Error>> + Send>>;
    fn vm_resize_zone_put(
        &self,
        vm_resize_zone: models::VmResizeZone,
    ) -> Pin<Box<dyn Future<Output = Result<(), Error>> + Send>>;
    fn vm_restore_put(
        &self,
        restore_config: models::RestoreConfig,
    ) -> Pin<Box<dyn Future<Output = Result<(), Error>> + Send>>;
    fn vm_send_migration_put(
        &self,
        send_migration_data: models::SendMigrationData,
    ) -> Pin<Box<dyn Future<Output = Result<(), Error>> + Send>>;
    fn vm_snapshot_put(
        &self,
        vm_snapshot_config: models::VmSnapshotConfig,
    ) -> Pin<Box<dyn Future<Output = Result<(), Error>> + Send>>;
    fn vmm_nmi_put(&self) -> Pin<Box<dyn Future<Output = Result<(), Error>> + Send>>;
    fn vmm_ping_get(
        &self,
    ) -> Pin<Box<dyn Future<Output = Result<models::VmmPingResponse, Error>> + Send>>;
}

impl<C: Connect> DefaultApi for DefaultApiClient<C>
where
    C: Clone + std::marker::Send + Sync,
{
    #[allow(unused_mut)]
    fn boot_vm(&self) -> Pin<Box<dyn Future<Output = Result<(), Error>> + Send>> {
        let mut req = __internal_request::Request::new(hyper::Method::PUT, "/vm.boot".to_string());
        req = req.returns_nothing();

        req.execute(self.configuration.borrow())
    }

    #[allow(unused_mut)]
    fn create_vm(
        &self,
        vm_config: models::VmConfig,
    ) -> Pin<Box<dyn Future<Output = Result<(), Error>> + Send>> {
        let mut req =
            __internal_request::Request::new(hyper::Method::PUT, "/vm.create".to_string());
        req = req.with_body_param(vm_config);
        req = req.returns_nothing();

        req.execute(self.configuration.borrow())
    }

    #[allow(unused_mut)]
    fn delete_vm(&self) -> Pin<Box<dyn Future<Output = Result<(), Error>> + Send>> {
        let mut req =
            __internal_request::Request::new(hyper::Method::PUT, "/vm.delete".to_string());
        req = req.returns_nothing();

        req.execute(self.configuration.borrow())
    }

    #[allow(unused_mut)]
    fn pause_vm(&self) -> Pin<Box<dyn Future<Output = Result<(), Error>> + Send>> {
        let mut req = __internal_request::Request::new(hyper::Method::PUT, "/vm.pause".to_string());
        req = req.returns_nothing();

        req.execute(self.configuration.borrow())
    }

    #[allow(unused_mut)]
    fn power_button_vm(&self) -> Pin<Box<dyn Future<Output = Result<(), Error>> + Send>> {
        let mut req =
            __internal_request::Request::new(hyper::Method::PUT, "/vm.power-button".to_string());
        req = req.returns_nothing();

        req.execute(self.configuration.borrow())
    }

    #[allow(unused_mut)]
    fn reboot_vm(&self) -> Pin<Box<dyn Future<Output = Result<(), Error>> + Send>> {
        let mut req =
            __internal_request::Request::new(hyper::Method::PUT, "/vm.reboot".to_string());
        req = req.returns_nothing();

        req.execute(self.configuration.borrow())
    }

    #[allow(unused_mut)]
    fn resume_vm(&self) -> Pin<Box<dyn Future<Output = Result<(), Error>> + Send>> {
        let mut req =
            __internal_request::Request::new(hyper::Method::PUT, "/vm.resume".to_string());
        req = req.returns_nothing();

        req.execute(self.configuration.borrow())
    }

    #[allow(unused_mut)]
    fn shutdown_vm(&self) -> Pin<Box<dyn Future<Output = Result<(), Error>> + Send>> {
        let mut req =
            __internal_request::Request::new(hyper::Method::PUT, "/vm.shutdown".to_string());
        req = req.returns_nothing();

        req.execute(self.configuration.borrow())
    }

    #[allow(unused_mut)]
    fn shutdown_vmm(&self) -> Pin<Box<dyn Future<Output = Result<(), Error>> + Send>> {
        let mut req =
            __internal_request::Request::new(hyper::Method::PUT, "/vmm.shutdown".to_string());
        req = req.returns_nothing();

        req.execute(self.configuration.borrow())
    }

    #[allow(unused_mut)]
    fn vm_add_device_put(
        &self,
        device_config: models::DeviceConfig,
    ) -> Pin<Box<dyn Future<Output = Result<models::PciDeviceInfo, Error>> + Send>> {
        let mut req =
            __internal_request::Request::new(hyper::Method::PUT, "/vm.add-device".to_string());
        req = req.with_body_param(device_config);

        req.execute(self.configuration.borrow())
    }

    #[allow(unused_mut)]
    fn vm_add_disk_put(
        &self,
        disk_config: models::DiskConfig,
    ) -> Pin<Box<dyn Future<Output = Result<models::PciDeviceInfo, Error>> + Send>> {
        let mut req =
            __internal_request::Request::new(hyper::Method::PUT, "/vm.add-disk".to_string());
        req = req.with_body_param(disk_config);

        req.execute(self.configuration.borrow())
    }

    #[allow(unused_mut)]
    fn vm_add_fs_put(
        &self,
        fs_config: models::FsConfig,
    ) -> Pin<Box<dyn Future<Output = Result<models::PciDeviceInfo, Error>> + Send>> {
        let mut req =
            __internal_request::Request::new(hyper::Method::PUT, "/vm.add-fs".to_string());
        req = req.with_body_param(fs_config);

        req.execute(self.configuration.borrow())
    }

    #[allow(unused_mut)]
    fn vm_add_net_put(
        &self,
        net_config: models::NetConfig,
    ) -> Pin<Box<dyn Future<Output = Result<models::PciDeviceInfo, Error>> + Send>> {
        let mut req =
            __internal_request::Request::new(hyper::Method::PUT, "/vm.add-net".to_string());
        req = req.with_body_param(net_config);

        req.execute(self.configuration.borrow())
    }

    #[allow(unused_mut)]
    fn vm_add_pmem_put(
        &self,
        pmem_config: models::PmemConfig,
    ) -> Pin<Box<dyn Future<Output = Result<models::PciDeviceInfo, Error>> + Send>> {
        let mut req =
            __internal_request::Request::new(hyper::Method::PUT, "/vm.add-pmem".to_string());
        req = req.with_body_param(pmem_config);

        req.execute(self.configuration.borrow())
    }

    #[allow(unused_mut)]
    fn vm_add_user_device_put(
        &self,
        vm_add_user_device: models::VmAddUserDevice,
    ) -> Pin<Box<dyn Future<Output = Result<models::PciDeviceInfo, Error>> + Send>> {
        let mut req =
            __internal_request::Request::new(hyper::Method::PUT, "/vm.add-user-device".to_string());
        req = req.with_body_param(vm_add_user_device);

        req.execute(self.configuration.borrow())
    }

    #[allow(unused_mut)]
    fn vm_add_vdpa_put(
        &self,
        vdpa_config: models::VdpaConfig,
    ) -> Pin<Box<dyn Future<Output = Result<models::PciDeviceInfo, Error>> + Send>> {
        let mut req =
            __internal_request::Request::new(hyper::Method::PUT, "/vm.add-vdpa".to_string());
        req = req.with_body_param(vdpa_config);

        req.execute(self.configuration.borrow())
    }

    #[allow(unused_mut)]
    fn vm_add_vsock_put(
        &self,
        vsock_config: models::VsockConfig,
    ) -> Pin<Box<dyn Future<Output = Result<models::PciDeviceInfo, Error>> + Send>> {
        let mut req =
            __internal_request::Request::new(hyper::Method::PUT, "/vm.add-vsock".to_string());
        req = req.with_body_param(vsock_config);

        req.execute(self.configuration.borrow())
    }

    #[allow(unused_mut)]
    fn vm_coredump_put(
        &self,
        vm_coredump_data: models::VmCoredumpData,
    ) -> Pin<Box<dyn Future<Output = Result<(), Error>> + Send>> {
        let mut req =
            __internal_request::Request::new(hyper::Method::PUT, "/vm.coredump".to_string());
        req = req.with_body_param(vm_coredump_data);
        req = req.returns_nothing();

        req.execute(self.configuration.borrow())
    }

    #[allow(unused_mut)]
    fn vm_counters_get(
        &self,
    ) -> Pin<
        Box<
            dyn Future<
                    Output = Result<
                        std::collections::HashMap<String, std::collections::HashMap<String, i64>>,
                        Error,
                    >,
                > + Send,
        >,
    > {
        let mut req =
            __internal_request::Request::new(hyper::Method::GET, "/vm.counters".to_string());

        req.execute(self.configuration.borrow())
    }

    #[allow(unused_mut)]
    fn vm_info_get(&self) -> Pin<Box<dyn Future<Output = Result<models::VmInfo, Error>> + Send>> {
        let mut req = __internal_request::Request::new(hyper::Method::GET, "/vm.info".to_string());

        req.execute(self.configuration.borrow())
    }

    #[allow(unused_mut)]
    fn vm_receive_migration_put(
        &self,
        receive_migration_data: models::ReceiveMigrationData,
    ) -> Pin<Box<dyn Future<Output = Result<(), Error>> + Send>> {
        let mut req = __internal_request::Request::new(
            hyper::Method::PUT,
            "/vm.receive-migration".to_string(),
        );
        req = req.with_body_param(receive_migration_data);
        req = req.returns_nothing();

        req.execute(self.configuration.borrow())
    }

    #[allow(unused_mut)]
    fn vm_remove_device_put(
        &self,
        vm_remove_device: models::VmRemoveDevice,
    ) -> Pin<Box<dyn Future<Output = Result<(), Error>> + Send>> {
        let mut req =
            __internal_request::Request::new(hyper::Method::PUT, "/vm.remove-device".to_string());
        req = req.with_body_param(vm_remove_device);
        req = req.returns_nothing();

        req.execute(self.configuration.borrow())
    }

    #[allow(unused_mut)]
    fn vm_resize_put(
        &self,
        vm_resize: models::VmResize,
    ) -> Pin<Box<dyn Future<Output = Result<(), Error>> + Send>> {
        let mut req =
            __internal_request::Request::new(hyper::Method::PUT, "/vm.resize".to_string());
        req = req.with_body_param(vm_resize);
        req = req.returns_nothing();

        req.execute(self.configuration.borrow())
    }

    #[allow(unused_mut)]
    fn vm_resize_zone_put(
        &self,
        vm_resize_zone: models::VmResizeZone,
    ) -> Pin<Box<dyn Future<Output = Result<(), Error>> + Send>> {
        let mut req =
            __internal_request::Request::new(hyper::Method::PUT, "/vm.resize-zone".to_string());
        req = req.with_body_param(vm_resize_zone);
        req = req.returns_nothing();

        req.execute(self.configuration.borrow())
    }

    #[allow(unused_mut)]
    fn vm_restore_put(
        &self,
        restore_config: models::RestoreConfig,
    ) -> Pin<Box<dyn Future<Output = Result<(), Error>> + Send>> {
        let mut req =
            __internal_request::Request::new(hyper::Method::PUT, "/vm.restore".to_string());
        req = req.with_body_param(restore_config);
        req = req.returns_nothing();

        req.execute(self.configuration.borrow())
    }

    #[allow(unused_mut)]
    fn vm_send_migration_put(
        &self,
        send_migration_data: models::SendMigrationData,
    ) -> Pin<Box<dyn Future<Output = Result<(), Error>> + Send>> {
        let mut req =
            __internal_request::Request::new(hyper::Method::PUT, "/vm.send-migration".to_string());
        req = req.with_body_param(send_migration_data);
        req = req.returns_nothing();

        req.execute(self.configuration.borrow())
    }

    #[allow(unused_mut)]
    fn vm_snapshot_put(
        &self,
        vm_snapshot_config: models::VmSnapshotConfig,
    ) -> Pin<Box<dyn Future<Output = Result<(), Error>> + Send>> {
        let mut req =
            __internal_request::Request::new(hyper::Method::PUT, "/vm.snapshot".to_string());
        req = req.with_body_param(vm_snapshot_config);
        req = req.returns_nothing();

        req.execute(self.configuration.borrow())
    }

    #[allow(unused_mut)]
    fn vmm_nmi_put(&self) -> Pin<Box<dyn Future<Output = Result<(), Error>> + Send>> {
        let mut req = __internal_request::Request::new(hyper::Method::PUT, "/vmm.nmi".to_string());
        req = req.returns_nothing();

        req.execute(self.configuration.borrow())
    }

    #[allow(unused_mut)]
    fn vmm_ping_get(
        &self,
    ) -> Pin<Box<dyn Future<Output = Result<models::VmmPingResponse, Error>> + Send>> {
        let mut req = __internal_request::Request::new(hyper::Method::GET, "/vmm.ping".to_string());

        req.execute(self.configuration.borrow())
    }
}
