use std::sync::{Arc, Mutex};

use prometheus_client::metrics::gauge::Gauge;
use prometheus_client::registry::Registry;
use sysinfo::System;
use tokio::time::{self, Duration};

pub struct SystemMetrics {
    cpu_usage: Gauge<i64>,
    memory_usage: Gauge<i64>,
    system: Arc<Mutex<System>>,
}

impl SystemMetrics {
    pub fn new(registry: &mut Registry) -> Self {
        let cpu_usage = Gauge::default();
        let memory_usage = Gauge::default();

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
        let sys = self.system.lock().unwrap();
        self.cpu_usage.set(sys.global_cpu_usage() as i64);
        self.memory_usage.set(sys.used_memory() as i64);
    }
}
