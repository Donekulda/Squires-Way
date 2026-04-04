//! Cross-cutting orchestration (Irony `IronyModManager.Services` slice): path resolution,
//! mod/game workflows, and future wiring. Dependencies point at [`crate::core`], [`crate::io`],
//! [`crate::parser`], and [`crate::storage`] — not the reverse.

mod integration;
mod merge;

pub mod resolver;

pub use integration::resolve_parsed_path_against_base;
pub use merge::{ModMergeService, ModMergeServiceError, ModMergeServiceStub};
