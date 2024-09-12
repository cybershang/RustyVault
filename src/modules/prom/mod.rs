//! The Prometheus metrics module. The user can use this module to store arbitary data
//! into RustyVault. The data stored in RustyVault is encrypted.

use std::{ops::Deref, sync::{Arc, RwLock}};

use crate::{
    core::Core,
    errors::RvError,
    logical::{
        secret::Secret, Backend, Field, FieldType, LogicalBackend, Operation, Path, PathOperation, Request, Response,
    },
    modules::Module,
    new_fields, new_fields_internal, new_logical_backend, new_logical_backend_internal, new_path, new_path_internal,
    new_secret, new_secret_internal,
    storage::StorageEntry,
};

use prometheus_client::{
    encoding::{text::encode::encode, EncodeLabelSet, EncodeLabelValue},
    metrics,
    metrics::{counter::Counter, family::Family, gauge::Gauge},
    registry::Registry,
};

pub struct PromModule {
    pub name: String,
    pub backend: Arc<PromBackend>,
}

pub struct PromBackend {
    pub inner: Arc<PromBackendInner>,
}

pub struct PromBackendInner {
    pub core: Arc<RwLock<Core>>,
}

impl Deref for PromBackend {
    type Target = PromBackendInner;

    fn deref(&self) -> &PromBackendInner {
        &self.inner
    }
}


pub struct Prom {
    pub registry: Registry,
    pub system_metrics: SystemMetrics,
}


// pub struct SystemMetrics {
//     pub cpu_usage: Gauge,
//     pub memory_usage: Gauge,
// }

impl PromBackend {
    pub fn new(core: Arc<RwLock<Core>>) -> Self {
        Self { inner: Arc::new(PromBackendInner { core }) }
    }
    pub fn new_backend(&self) -> LogicalBackend {}
}

// business code
impl PromBackendInner{}

impl PromModule{
    pub fn new(core: Arc<RwLock<Core>>) -> Self {
        Self { name: "prom".to_string(), backend: Arc::new(PromBackend::new(core)) }
    }
}

impl Module for PromModule {
    fn name(&self) -> String {
        return self.name.clone();
    }

    fn setup(&mut self, core: &Core) -> Result<(), RvError> {
        let prom = Arc::clone(&self.backend);
        let prom_backend_new_func = move |_c: Arc<RwLock<Core>>| -> Result<Arc<dyn Backend>, RvError> {
            let mut prom_backend = prom.new_backend();
            prom_backend.init()?;
            Ok(Arc::new(prom_backend))
        };
        core.add_logical_backend("prom", Arc::new(prom_backend_new_func))
    }

    fn cleanup(&mut self, core: &Core) -> Result<(), RvError> {
        core.delete_logical_backend("kv")
    }
}
