//! Definition merge helpers (Irony `ParserMerger` / patch collection merge paths).
//!
//! C# reference:
//! - [`ParserMerger.MergeTopLevel`](https://github.com/bcssov/IronyModManager) — `IronyModManager.Parser/ParserMerger.cs`
//! - Orchestration: `ModPatchCollectionService` (merge loops), `ModMergeService` (collection zip/export)
//!
//! ## Feature map (APTV)
//! - C# `MergeTopLevel` → [`merge_top_level`] / [`merge_top_level_str`] (jomini parse + list merge + format).
//! - C# `IParseResponse` / `IScriptElement` → [`ScriptElement`] + jomini `ObjectReader` bridge in `parser_merger`.
//! - Errors: empty `String` on failure (Irony returns `string.Empty`).

mod format_code;
mod merge_top_level;
mod parser_merger;

pub use merge_top_level::{ScriptElement, merge_top_level_children};
pub use parser_merger::{merge_top_level, merge_top_level_str};
