//! Errors for low-level I/O helpers (read paths, JSON decode).

use std::path::PathBuf;

use thiserror::Error;

/// Failure while reading or decoding data in the `io` layer.
#[derive(Debug, Error)]
pub enum IoError {
    #[error("failed to read {path}: {source}")]
    ReadFile {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },
    #[error("failed to write {path}: {source}")]
    WriteFile {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },
    #[error("failed to create parent directory for {path}: {source}")]
    CreateParent {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },
    #[error("failed to decode JSON from {path}: {source}")]
    JsonDecode {
        path: PathBuf,
        #[source]
        source: serde_json::Error,
    },
    #[error("failed to encode JSON for {path}: {source}")]
    JsonEncode {
        path: PathBuf,
        #[source]
        source: serde_json::Error,
    },
}
