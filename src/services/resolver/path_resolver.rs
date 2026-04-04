//! Configured path parsing (Irony `PathResolver` — special folders + env segments).

use std::path::PathBuf;

use crate::io::path_ops::{
    resolve_path_segment_environment_variable, standardize_directory_separator,
};

/// Parses path strings with special segments and environment placeholders (Irony `PathResolver.Parse`).
///
/// Recognized segments (case-insensitive via uppercase match, same as Irony):
/// - `%USER_DOCUMENTS%` → user documents directory ([`dirs::document_dir`])
/// - `$LINUX_DATA_HOME` → local application data ([`dirs::data_local_dir`], C# `SpecialFolder.LocalApplicationData`)
///
/// Other segments use [`crate::io::path_ops::resolve_path_segment_environment_variable`].
#[derive(Clone, Copy, Debug, Default)]
pub struct PathResolver;

impl PathResolver {
    pub fn parse(self, path: &str) -> PathBuf {
        if path.trim().is_empty() {
            return PathBuf::from(path);
        }

        let s = standardize_directory_separator(path);
        let sep = std::path::MAIN_SEPARATOR;
        let segments: Vec<&str> = s.split(sep).filter(|seg| !seg.is_empty()).collect();

        let mut out = PathBuf::new();
        for item in segments {
            let upper = item.to_uppercase();
            let resolved = match upper.as_str() {
                "%USER_DOCUMENTS%" => dirs::document_dir()
                    .map(|p| p.to_string_lossy().into_owned())
                    .unwrap_or_default(),
                "$LINUX_DATA_HOME" => dirs::data_local_dir()
                    .map(|p| p.to_string_lossy().into_owned())
                    .unwrap_or_default(),
                _ => resolve_path_segment_environment_variable(item),
            };
            out.push(resolved);
        }
        out
    }
}

/// Standalone parse (parity with `new PathResolver().Parse(path)`).
pub fn parse_path(path: &str) -> PathBuf {
    PathResolver.parse(path)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn parse_empty_returns_empty() {
        let p = PathResolver.parse("");
        assert!(p.as_os_str().is_empty());
    }

    #[test]
    fn parse_plain_joins_segments() {
        let p = PathResolver.parse("foo/bar");
        assert!(p.ends_with(Path::new("bar")));
        assert!(p.to_string_lossy().contains("foo"));
    }

    #[test]
    fn parse_user_documents_segment() {
        let Some(expected) = dirs::document_dir() else {
            return;
        };
        let p = PathResolver.parse("%USER_DOCUMENTS%/mods");
        assert!(
            p.starts_with(&expected),
            "got {p:?} expected prefix {expected:?}"
        );
        assert!(p.ends_with("mods"));
    }

    #[test]
    fn parse_expands_dollar_env() {
        let key = "SQUIRESWAY_PATH_RESOLVER_TEST";
        unsafe {
            std::env::set_var(key, "resolved_seg");
        }
        let p = PathResolver.parse(&format!("${}/tail", key));
        unsafe {
            std::env::remove_var(key);
        }
        assert!(p.ends_with("tail"), "got {p:?}");
        assert!(p.to_string_lossy().contains("resolved_seg"));
    }
}
