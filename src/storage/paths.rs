//! Root paths and config-driven resolution (Irony `DiskOperations.ResolveStoragePath`, `IStorageProvider.GetRootStoragePath`).

use std::path::{Path, PathBuf};

use crate::io::path_ops::{
    resolve_path_segment_environment_variable, resolve_relative_path,
    standardize_directory_separator,
};

/// Joins configured path segments with env expansion, then resolves against `app_base` (Irony `DiskOperations.ResolveStoragePath`).
///
/// `config_storage_path` is typically a relative or absolute path string from app settings; `app_base` is the
/// process base directory (Irony uses `AppDomain.CurrentDomain.BaseDirectory`).
pub fn resolve_storage_path(config_storage_path: &str, app_base: impl AsRef<Path>) -> PathBuf {
    let sep = std::path::MAIN_SEPARATOR;
    let s = standardize_directory_separator(config_storage_path);
    let expanded: String = s
        .split(sep)
        .filter(|x| !x.is_empty())
        .map(resolve_path_segment_environment_variable)
        .collect::<Vec<_>>()
        .join(&sep.to_string());
    resolve_relative_path(app_base, &expanded)
}

/// Root directory for versioned JSON persistence (Irony `JsonStore.InitPath`: resolved storage + optional assembly title subfolder).
///
/// When `app_title_segment` is `Some`, it appends one path component under [`resolve_storage_path`], matching Irony `Path.Combine(storagePath, titleSegment)`.
pub fn json_store_root(
    config_storage_path: &str,
    app_base: impl AsRef<Path>,
    app_title_segment: Option<&str>,
) -> PathBuf {
    let base = resolve_storage_path(config_storage_path, app_base);
    match app_title_segment {
        Some(t) if !t.trim().is_empty() => base.join(t.trim()),
        _ => base,
    }
}

/// Per-user data directory for SquiresWay using [ProjectDirs](directories::ProjectDirs).
pub fn default_data_dir() -> Option<PathBuf> {
    directories::ProjectDirs::from("com", "SquiresWay", "SquiresWay")
        .map(|p| p.data_dir().to_path_buf())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resolve_storage_joins_under_base() {
        let base = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let p = resolve_storage_path("nested/sub", &base);
        assert!(
            p.ends_with("sub"),
            "expected .../nested/sub ending in sub, got {p:?}"
        );
    }

    #[test]
    fn json_store_root_appends_title() {
        let base = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let p = json_store_root("cfg_root", &base, Some("AppTitle"));
        assert!(p.ends_with("AppTitle"), "expected title segment, got {p:?}");
    }

    #[test]
    fn default_data_dir_some() {
        assert!(default_data_dir().is_some());
    }

    #[test]
    fn resolve_storage_expands_dollar_env() {
        let base = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let key = "SQUIRESWAY_TEST_STORAGE_SEG";
        // SAFETY: test-only unique key; removed after assertion.
        unsafe {
            std::env::set_var(key, "from_env");
        }
        let p = resolve_storage_path("$SQUIRESWAY_TEST_STORAGE_SEG", &base);
        unsafe {
            std::env::remove_var(key);
        }
        assert!(
            p.ends_with("from_env"),
            "expected env-expanded segment, got {p:?}"
        );
    }

    #[test]
    fn resolve_storage_expands_percent_env() {
        let base = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let key = "SQUIRESWAY_TEST_STORAGE_PCT";
        unsafe {
            std::env::set_var(key, "pct_val");
        }
        let p = resolve_storage_path("%SQUIRESWAY_TEST_STORAGE_PCT%", &base);
        unsafe {
            std::env::remove_var(key);
        }
        assert!(
            p.ends_with("pct_val"),
            "expected %VAR% expansion, got {p:?}"
        );
    }
}
