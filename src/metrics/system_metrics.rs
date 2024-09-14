use std::sync::{atomic::AtomicU64, Arc, Mutex};

use prometheus_client::metrics::gauge::Gauge;
use prometheus_client::registry::Registry;
use sysinfo::System;
use tokio::time::{self, Duration};

pub struct SystemMetrics {
    cpu_usage: Gauge<f64, AtomicU64>,
    memory_usage: Gauge<f64, AtomicU64>,
    system: Arc<Mutex<System>>,
}

impl SystemMetrics {
    pub fn new(registry: &mut Registry) -> Self {
        let cpu_usage = Gauge::<f64, AtomicU64>::default();
        let memory_usage = Gauge::<f64, AtomicU64>::default();

        registry.register("cpu_usage_percent", "CPU usage percent", cpu_usage.clone());
        registry.register("memory_usage", "Memory usage percent", memory_usage.clone());

        let system = Arc::new(Mutex::new(System::new_all()));

        Self { cpu_usage, memory_usage, system }
    }

    pub async fn start_collecting(self: Arc<Self>) {
        let mut interval = time::interval(Duration::from_secs(5));

        loop {
            interval.tick().await;

            {
                let mut sys = self.system.lock().unwrap();
                sys.refresh_all();
            }

            self.collect_metrics();
        }
    }

    fn collect_metrics(&self) {
        let mut sys = self.system.lock().unwrap();
        sys.refresh_cpu_all();
        self.cpu_usage.set(sys.global_cpu_usage() as f64);
        self.memory_usage.set(sys.used_memory() as f64);
    }
}
