use metrics::Gauge;
use metrics_derive::Metrics;
use once_cell::sync::Lazy;

/// This defines the struct which encapsulates all metrics used for Evm.
///
/// It is unused because we directly use gauge and histogram macros since that is the
/// only way in which we can provide additional labels to the metric.
/// However, deriving `Metrics` here is convenient to provide descriptions for each of
/// the metrics.
#[allow(unused)]
#[derive(Metrics)]
#[metrics(scope = "evm")]
pub struct EvmMetrics {
    #[metric(describe = "Current Block Gas Usage")]
    pub(crate) gas_usage: Gauge,
}

/// EVM metrics
pub static EVM_METRICS: Lazy<EvmMetrics> = Lazy::new(|| {
    EvmMetrics::describe();
    EvmMetrics::default()
});
