use prometheus_client::metrics::gauge::Gauge;
use prometheus_client::registry::Registry;
use std::sync::{atomic::AtomicU64, Arc, Mutex};
use sysinfo::{Disks, System};
use tokio::time::{self, Duration};

pub const CPU_USAGE_PERCENT: &str = "cpu_usage_percent";
pub const CPU_USAGE_PERCENT_HELP: &str = "CPU usage percent";
pub const TOTAL_MEMORY: &str = "total_memory";
pub const TOTAL_MEMORY_HELP: &str = "Total memory";
pub const USED_MEMORY: &str = "used_memory";
pub const USED_MEMORY_HELP: &str = "Used memory";
pub const FREE_MEMORY: &str = "free_memory";
pub const FREE_MEMORY_HELP: &str = "Free memory";
pub const TOTAL_DISK_SPACE: &str = "total_disk_space";
pub const TOTAL_DISK_SPACE_HELP: &str = "Total disk space";
pub const TOTAL_DISK_AVAILABLE: &str = "total_disk_available";
pub const TOTAL_DISK_AVAILABLE_HELP: &str = "Total disk available";
// pub const NETWORK_IN: &str = "network_in";
// pub const NETWORK_IN_HELP: &str = "Network in";
// pub const NETWORK_OUT: &str = "network_out";
// pub const NETWORK_OUT_HELP: &str = "Network out";
pub const LOAD_AVERAGE: &str = "load_average";
pub const LOAD_AVERAGE_HELP: &str = "System load average";

pub struct SystemMetrics {
    system: Arc<Mutex<System>>,
    collection_interval: u64,
    cpu_usage: Gauge<f64, AtomicU64>,
    total_memory: Gauge<f64, AtomicU64>,
    used_memory: Gauge<f64, AtomicU64>,
    free_memory: Gauge<f64, AtomicU64>,
    total_disk_available: Gauge<f64, AtomicU64>,
    total_disk_space: Gauge<f64, AtomicU64>,
    // network_in: Gauge<f64, AtomicU64>,
    // network_out: Gauge<f64, AtomicU64>,
    load_avg: Gauge<f64, AtomicU64>,
}

impl SystemMetrics {
    pub fn new(registry: &mut Registry, collection_interval: u64) -> Self {
        let cpu_usage = Gauge::<f64, AtomicU64>::default();

        let total_memory = Gauge::<f64, AtomicU64>::default();
        let used_memory = Gauge::<f64, AtomicU64>::default();
        let free_memory = Gauge::<f64, AtomicU64>::default();

        let total_disk_space = Gauge::<f64, AtomicU64>::default();
        let total_disk_available = Gauge::<f64, AtomicU64>::default();

        // let network_in = Gauge::<f64, AtomicU64>::default();
        // let network_out = Gauge::<f64, AtomicU64>::default();
        let load_avg = Gauge::<f64, AtomicU64>::default();

        registry.register(CPU_USAGE_PERCENT, CPU_USAGE_PERCENT_HELP, cpu_usage.clone());

        registry.register(TOTAL_MEMORY, TOTAL_MEMORY_HELP, total_memory.clone());
        registry.register(USED_MEMORY, USED_MEMORY_HELP, used_memory.clone());
        registry.register(FREE_MEMORY, FREE_MEMORY_HELP, free_memory.clone());

        registry.register(TOTAL_DISK_SPACE, TOTAL_DISK_SPACE_HELP, total_disk_space.clone());
        registry.register(TOTAL_DISK_AVAILABLE, TOTAL_DISK_AVAILABLE_HELP, total_disk_available.clone());

        // registry.register(NETWORK_IN, NETWORK_IN_HELP, network_in.clone());
        // registry.register(NETWORK_OUT, NETWORK_OUT_HELP, network_out.clone());

        registry.register(LOAD_AVERAGE, LOAD_AVERAGE_HELP, load_avg.clone());

        let system = Arc::new(Mutex::new(System::new_all()));

        Self {
            system,
            collection_interval,
            cpu_usage,
            total_memory,
            used_memory,
            free_memory,
            total_disk_available,
            total_disk_space,
            // network_in,
            // network_out,
            load_avg,
        }
    }

    pub async fn start_collecting(self: Arc<Self>) {
        let mut interval = time::interval(Duration::from_secs(self.collection_interval));

        loop {
            interval.tick().await;
            self.collect_metrics();
        }
    }

    fn collect_metrics(&self) {
        let mut sys = self.system.lock().unwrap();
        sys.refresh_all();

        self.cpu_usage.set(sys.global_cpu_usage() as f64);

        self.total_memory.set(sys.total_memory() as f64);
        self.used_memory.set(sys.used_memory() as f64);
        self.free_memory.set(sys.free_memory() as f64);

        let mut total_available_space = 0;
        let mut total_disk_space = 0;

        for disk in Disks::new_with_refreshed_list().list() {
            total_available_space += disk.available_space();
            total_disk_space += disk.total_space();
        }
        self.total_disk_available.set(total_available_space as f64);
        self.total_disk_space.set(total_disk_space as f64);

        // let mut total_network_in = 0;
        // let mut total_network_out = 0;

        // TODO: network data stays at zero all the time
        // for (_, n) in Networks::new_with_refreshed_list().list() {
        //     total_network_in += n.received();
        //     total_network_out += n.transmitted();
        // }

        // self.network_in.set(total_network_in as f64);
        // self.network_out.set(total_network_out as f64);

        self.load_avg.set(System::load_average().one as f64);
    }
}
