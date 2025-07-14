//! Metrics collection for the parallel prover service
//!
//! This module defines metrics that track various aspects of parallel prover service,
//! including block processing times and current block numbers.

use metrics::{Gauge, Histogram};
use metrics_derive::Metrics;
use once_cell::sync::Lazy;

/// Collection of metrics for monitoring parallel prover service performance and state
#[derive(Metrics)]
#[metrics(scope = "parallel_prover_service")]
pub struct ParallelProverMetrics {
    /// Number of ongoing proving jobs
    #[metric(describe = "Number of ongoing proving jobs")]
    pub ongoing_proving_jobs: Gauge,
}

/// Parallel prover metrics
pub static PARALLEL_PROVER_METRICS: Lazy<ParallelProverMetrics> = Lazy::new(|| {
    ParallelProverMetrics::describe();
    ParallelProverMetrics::default()
});
