//! Read JSON from disk (Irony `JsonFileReader` / `IFileReader` read path).

use std::path::Path;

use serde::de::DeserializeOwned;

use super::error::IoError;

/// True if `path` ends with `.json` (case-insensitive), matching Irony `JsonFileReader.CanRead`.
pub fn can_read_json_path(path: impl AsRef<Path>) -> bool {
    path.as_ref()
        .extension()
        .and_then(|e| e.to_str())
        .is_some_and(|e| e.eq_ignore_ascii_case(super::constants::JSON_EXTENSION))
}

/// Reads and deserializes a UTF-8 JSON file.
pub fn read_json_file<T: DeserializeOwned>(path: impl AsRef<Path>) -> Result<T, IoError> {
    let path = path.as_ref();
    let bytes = std::fs::read(path).map_err(|source| IoError::ReadFile {
        path: path.to_path_buf(),
        source,
    })?;
    serde_json::from_slice(&bytes).map_err(|source| IoError::JsonDecode {
        path: path.to_path_buf(),
        source,
    })
}

/// Reads a JSON file into a [`serde_json::Value`].
pub fn read_json_value(path: impl AsRef<Path>) -> Result<serde_json::Value, IoError> {
    read_json_file(path)
}

/// Reads raw bytes (for callers that need encoding detection later).
pub fn read_file_bytes(path: impl AsRef<Path>) -> Result<Vec<u8>, IoError> {
    let path = path.as_ref();
    std::fs::read(path).map_err(|source| IoError::ReadFile {
        path: path.to_path_buf(),
        source,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    #[test]
    fn can_read_detects_json_extension() {
        assert!(can_read_json_path("foo.JSON"));
        assert!(can_read_json_path("x/bar.json"));
        assert!(!can_read_json_path("x/bar.txt"));
    }

    #[test]
    fn read_json_roundtrip() {
        let dir = tempfile::tempdir().expect("tempdir");
        let p = dir.path().join("t.json");
        let mut f = std::fs::File::create(&p).expect("create");
        writeln!(f, "{{\"k\": 1}}").expect("write");
        drop(f);
        let v: serde_json::Value = read_json_value(&p).expect("read");
        assert_eq!(v["k"], 1);
    }
}
