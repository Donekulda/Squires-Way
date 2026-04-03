//! Cross-cutting orchestration (Irony `IronyModManager.Services` slice): path resolution,
//! mod/game workflows, and future wiring. Dependencies point at [`crate::core`], [`crate::io`],
//! [`crate::parser`], and [`crate::storage`] — not the reverse.

pub mod resolver;
