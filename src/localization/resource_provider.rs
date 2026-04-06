//! JSON file bundles per locale (`{root}/{locale}.json`), matching Irony `BaseLocalizationResourceProvider`.

use std::path::{Path, PathBuf};

/// Reads UTF-8 JSON localization files from a directory (Irony `ILocalizationResourceProvider`).
pub trait LocalizationResourceProvider: Send + Sync {
    /// Root directory containing `{locale}.json` files.
    fn root_path(&self) -> &Path;

    /// Raw JSON text for `locale`, or empty string if the file is missing or unreadable.
    fn read_resource(&self, locale: &str) -> String;
}

/// Default provider: `{root}/{locale}.json` (UTF-8).
#[derive(Debug, Clone)]
pub struct FileLocalizationResourceProvider {
    root: PathBuf,
}

impl FileLocalizationResourceProvider {
    pub fn new(root: impl Into<PathBuf>) -> Self {
        Self { root: root.into() }
    }
}

impl LocalizationResourceProvider for FileLocalizationResourceProvider {
    fn root_path(&self) -> &Path {
        &self.root
    }

    fn read_resource(&self, locale: &str) -> String {
        let path = self.root.join(format!("{locale}.json"));
        if path.is_file() {
            std::fs::read_to_string(&path).unwrap_or_default()
        } else {
            String::new()
        }
    }
}
