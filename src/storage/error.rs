//! Errors for persistence and layout in the `storage` layer.

use std::path::PathBuf;

use thiserror::Error;

/// Failure resolving or accessing application storage locations.
#[derive(Debug, Error)]
pub enum StorageError {
    #[error("no default data directory (invalid application name for XDG / known folders)")]
    NoDefaultDataDir,
    #[error("failed to resolve path {path}: {source}")]
    Io {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },
}
