/// Defines the Result and Error types
use anyhow;

/// Result type for the KvStore
pub type Result<T> = anyhow::Result<T>;
