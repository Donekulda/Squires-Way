//! Read and write JSON on disk (Irony `JsonFileReader` / `JsonStore` persistence).
//!
//! Application JSON is **UTF-8** end-to-end. C# `string` is UTF-16; this crate stores and parses
//! JSON as UTF-8 bytes (optionally with a UTF-8 BOM, stripped on read for interoperability).

use std::path::Path;

use serde::Serialize;
use serde::de::DeserializeOwned;

use super::error::IoError;

/// UTF-8 byte order mark (some editors prepend this to "UTF-8" files).
const UTF8_BOM: &[u8] = &[0xEF, 0xBB, 0xBF];

#[inline]
fn strip_utf8_bom(bytes: &[u8]) -> &[u8] {
    bytes.strip_prefix(UTF8_BOM).unwrap_or(bytes)
}

/// True if `path` ends with `.json` (case-insensitive), matching Irony `JsonFileReader.CanRead`.
pub fn can_read_json_path(path: impl AsRef<Path>) -> bool {
    path.as_ref()
        .extension()
        .and_then(|e| e.to_str())
        .is_some_and(|e| e.eq_ignore_ascii_case(super::constants::JSON_EXTENSION))
}

/// Reads and deserializes a UTF-8 JSON file (leading UTF-8 BOM is accepted).
pub fn read_json_file<T: DeserializeOwned>(path: impl AsRef<Path>) -> Result<T, IoError> {
    let path = path.as_ref();
    let bytes = std::fs::read(path).map_err(|source| IoError::ReadFile {
        path: path.to_path_buf(),
        source,
    })?;
    let slice = strip_utf8_bom(&bytes);
    serde_json::from_slice(slice).map_err(|source| IoError::JsonDecode {
        path: path.to_path_buf(),
        source,
    })
}

/// Reads a JSON file into a [`serde_json::Value`].
pub fn read_json_value(path: impl AsRef<Path>) -> Result<serde_json::Value, IoError> {
    read_json_file(path)
}

/// Writes `value` as pretty-printed UTF-8 JSON (no BOM), creating parent directories as needed.
///
/// Matches typical Newtonsoft.Json indented output used by Irony `JsonStore` persistence.
pub fn write_json_file<T: Serialize + ?Sized>(
    path: impl AsRef<Path>,
    value: &T,
) -> Result<(), IoError> {
    let path = path.as_ref();
    let bytes = serde_json::to_vec_pretty(value).map_err(|source| IoError::JsonEncode {
        path: path.to_path_buf(),
        source,
    })?;
    if let Some(parent) = path.parent()
        && !parent.as_os_str().is_empty()
    {
        std::fs::create_dir_all(parent).map_err(|source| IoError::CreateParent {
            path: parent.to_path_buf(),
            source,
        })?;
    }
    std::fs::write(path, bytes).map_err(|source| IoError::WriteFile {
        path: path.to_path_buf(),
        source,
    })
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

    #[test]
    fn read_json_accepts_utf8_bom() {
        let dir = tempfile::tempdir().expect("tempdir");
        let p = dir.path().join("bom.json");
        let mut content = Vec::from([0xEFu8, 0xBB, 0xBF]);
        content.extend_from_slice(br#"{"x":true}"#);
        std::fs::write(&p, content).expect("write");
        let v: serde_json::Value = read_json_value(&p).expect("read");
        assert_eq!(v["x"], true);
    }

    #[test]
    fn write_json_roundtrip() {
        let dir = tempfile::tempdir().expect("tempdir");
        let p = dir.path().join("nested").join("out.json");
        let obj = serde_json::json!({ "n": 42 });
        write_json_file(&p, &obj).expect("write");
        let v: serde_json::Value = read_json_value(&p).expect("read");
        assert_eq!(v["n"], 42);
        let raw = std::fs::read(&p).expect("read bytes");
        assert!(
            !raw.starts_with(UTF8_BOM),
            "written JSON should not include BOM"
        );
    }
}
