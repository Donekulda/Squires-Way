//! Application storage layout and lifecycle (Irony `IronyModManager.Storage` / `Storage.Common` slice).
//!
//! Low-level reads and format helpers live in [`crate::io`].

mod error;
pub mod paths;

pub use error::StorageError;
pub use paths::{default_data_dir, resolve_storage_path};
