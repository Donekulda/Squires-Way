//! Path helpers aligned with Irony `PathOperations` / `Extensions.StandardizeDirectorySeparator`.

use std::path::{Component, Path, PathBuf};

/// Resolves a single path segment’s environment or placeholder (Irony `PathResolver.ResolveEnvironmentVariable`).
///
/// `$VAR` strips **all** leading `$` characters (C# `TrimStart('$')`), then reads `VAR`. `%VAR%` reads `VAR`.
pub fn resolve_path_segment_environment_variable(variable: &str) -> String {
    if variable.is_empty() {
        return String::new();
    }
    if variable.starts_with('$') {
        let name = variable.trim_start_matches('$');
        if name.is_empty() {
            return variable.to_string();
        }
        return std::env::var(name).unwrap_or_else(|_| variable.to_string());
    }
    if variable.len() > 2 && variable.starts_with('%') && variable.ends_with('%') {
        let name = &variable[1..variable.len() - 1];
        return std::env::var(name).unwrap_or_else(|_| variable.to_string());
    }
    variable.to_string()
}

/// Replaces `/` and `\` with the platform directory separator (Irony `StandardizeDirectorySeparator`).
pub fn standardize_directory_separator(s: &str) -> String {
    if s.is_empty() {
        return String::new();
    }
    let sep = std::path::MAIN_SEPARATOR;
    s.chars()
        .map(|c| if c == '/' || c == '\\' { sep } else { c })
        .collect()
}

/// Resolves `path` against `base` and normalizes `.` / `..` lexically (Irony `PathOperations.ResolveRelativePath`).
///
/// Does not require the path to exist on disk. If `path` is absolute, returns it normalized.
pub fn resolve_relative_path(base: impl AsRef<Path>, path: &str) -> PathBuf {
    let base = base.as_ref();
    let path = standardize_directory_separator(path);
    let path = path.trim();
    if path.is_empty() {
        return normalize_path_components(base.to_path_buf());
    }
    let p = Path::new(&path);
    let combined = if p.is_absolute() {
        p.to_path_buf()
    } else {
        base.join(p)
    };
    normalize_path_components(combined)
}

/// Best-effort filesystem casing for an existing path (Irony `GetActualPathCasing`).
///
/// On most platforms this uses [`std::fs::canonicalize`]. If the path does not exist, returns the input unchanged.
pub fn actual_path_casing(path: impl AsRef<Path>) -> std::io::Result<PathBuf> {
    let path = path.as_ref();
    if path.as_os_str().is_empty() {
        return Ok(PathBuf::new());
    }
    if path.exists() {
        std::fs::canonicalize(path)
    } else {
        Ok(path.to_path_buf())
    }
}

fn normalize_path_components(path: PathBuf) -> PathBuf {
    let mut out = PathBuf::new();
    for c in path.components() {
        match c {
            Component::Prefix(_) | Component::RootDir => {
                out = PathBuf::new();
                out.push(c);
            }
            Component::CurDir => {}
            Component::ParentDir => {
                let _ = out.pop();
            }
            Component::Normal(x) => out.push(x),
        }
    }
    if out.as_os_str().is_empty() {
        PathBuf::from(".")
    } else {
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn standardize_separator() {
        let s = standardize_directory_separator(r"a/b\c");
        assert!(
            !s.contains('/'),
            "forward slashes should become platform separator"
        );
        assert!(
            s.contains(std::path::MAIN_SEPARATOR),
            "expected platform separator in {s:?}"
        );
    }

    #[test]
    fn resolve_relative_strips_dotdot() {
        let base = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let r = resolve_relative_path(&base, "foo/../bar");
        assert!(r.ends_with("bar"), "expected .../bar, got {r:?}");
        assert!(!r.ends_with("foo"));
    }

    #[test]
    fn resolve_env_segment_strips_all_leading_dollar() {
        let key = "SQUIRESWAY_TRIM_DOLLAR";
        unsafe {
            std::env::set_var(key, "v");
        }
        let s = resolve_path_segment_environment_variable(&format!("$${}", key));
        unsafe {
            std::env::remove_var(key);
        }
        assert_eq!(s, "v");
    }
}
