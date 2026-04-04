//! Composition of [`crate::io`], [`crate::storage`], and [`super::resolver`] for service callers.
//!
//! Higher-level orchestration (mods, definitions) will also use [`crate::parser`] at call sites;
//! keep this module free of parser imports to avoid pulling search/index logic into path-only tests.

use std::path::{Path, PathBuf};

use crate::services::resolver::PathResolver;

/// [`PathResolver::parse`] then [`crate::io::path_ops::resolve_relative_path`] against `app_base`.
///
/// Use this for Irony `PathResolver`-style configured paths (e.g. `%USER_DOCUMENTS%`). For JSON store
/// roots with per-segment env expansion, use [`crate::storage::resolve_storage_path`].
pub fn resolve_parsed_path_against_base(path: &str, app_base: impl AsRef<Path>) -> PathBuf {
    let parsed = PathResolver.parse(path);
    crate::io::path_ops::resolve_relative_path(app_base, &parsed.to_string_lossy())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn resolve_parsed_relative_under_base() {
        let base = Path::new(env!("CARGO_MANIFEST_DIR"));
        let out = resolve_parsed_path_against_base("nested/tail", base);
        assert!(out.ends_with("tail"), "got {out:?}");
    }

    #[test]
    fn storage_resolve_still_available_from_services_crate_path() {
        let base = Path::new(env!("CARGO_MANIFEST_DIR"));
        let p = crate::storage::resolve_storage_path("a/b", base);
        assert!(p.ends_with("b"));
    }
}
