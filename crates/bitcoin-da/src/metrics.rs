use metrics::Gauge;
use metrics_derive::Metrics;
use once_cell::sync::Lazy;

/// This defines the struct which encapsulates all metrics used for Bitcoin DA.
///
/// It is unused because we directly use gauge and histogram macros since that is the
/// only way in which we can provide additional labels to the metric.
/// However, deriving `Metrics` here is convenient to provide descriptions for each of
/// the metrics.
#[allow(unused)]
#[derive(Metrics)]
#[metrics(scope = "bitcoin_da")]
pub struct BitcoinDaMetrics {
    /// Bitcoin DA Transaction count in queue
    #[metric(describe = "How many transactions are currently in the Bitcoin DA queue")]
    pub(crate) transaction_queue_size: Gauge,
}

/// Bitcoin DA metrics
pub static BITCOIN_DA_METRICS: Lazy<BitcoinDaMetrics> = Lazy::new(|| {
    BitcoinDaMetrics::describe();
    BitcoinDaMetrics::default()
});
