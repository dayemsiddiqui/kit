//! Workflow configuration

use crate::config::env;

/// Workflow configuration
///
/// # Environment Variables
///
/// - `WORKFLOW_POLL_INTERVAL_MS` - Worker poll interval in milliseconds (default: 1000)
/// - `WORKFLOW_CONCURRENCY` - Number of workflows to process concurrently (default: 4)
/// - `WORKFLOW_LOCK_TIMEOUT_SECS` - Lease duration in seconds (default: 30)
/// - `WORKFLOW_MAX_ATTEMPTS` - Max workflow attempts (default: 3)
/// - `WORKFLOW_RETRY_BACKOFF_SECS` - Linear backoff seconds (default: 5)
#[derive(Debug, Clone)]
pub struct WorkflowConfig {
    /// Worker poll interval in milliseconds
    pub poll_interval_ms: u64,
    /// Max concurrent workflows processed by a worker
    pub concurrency: usize,
    /// Lease duration in seconds
    pub lock_timeout_secs: u64,
    /// Max attempts per workflow
    pub max_attempts: i32,
    /// Linear backoff seconds per attempt
    pub retry_backoff_secs: i64,
}

impl WorkflowConfig {
    /// Build config from environment variables
    pub fn from_env() -> Self {
        Self {
            poll_interval_ms: env("WORKFLOW_POLL_INTERVAL_MS", 1000u64),
            concurrency: env("WORKFLOW_CONCURRENCY", 4usize),
            lock_timeout_secs: env("WORKFLOW_LOCK_TIMEOUT_SECS", 30u64),
            max_attempts: env("WORKFLOW_MAX_ATTEMPTS", 3i32),
            retry_backoff_secs: env("WORKFLOW_RETRY_BACKOFF_SECS", 5i64),
        }
    }
}

impl Default for WorkflowConfig {
    fn default() -> Self {
        Self::from_env()
    }
}
