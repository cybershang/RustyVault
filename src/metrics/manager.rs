use crate::metrics::system_metrics::SystemMetrics;
use prometheus_client::registry::Registry;
use std::sync::Arc;

pub struct MetricsManager {
    pub prometheus_registry: Arc<Registry>,
    pub system_metrics: Arc<SystemMetrics>,
}

impl MetricsManager {
    pub fn new() -> Self {
        let mut prometheus_registry = Registry::default();
        let system_metrics = Arc::new(SystemMetrics::new(&mut prometheus_registry));
        let prometheus_registry = Arc::new(prometheus_registry);
        MetricsManager { prometheus_registry, system_metrics }
    }
}
