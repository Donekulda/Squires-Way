//! Low-level filesystem and format helpers (Irony `IronyModManager.IO` / `IO.Common` slice).
//!
//! Keep orchestration and persistence policy in [`crate::storage`].

pub mod constants;
mod error;
pub mod json;
pub mod path_ops;

pub use error::IoError;
pub use json::{
    can_read_json_path, read_file_bytes, read_json_file, read_json_value, write_json_file,
};
