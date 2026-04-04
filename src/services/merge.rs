//! Contract mirror of Irony [`IModMergeService`](https://github.com/bcssov/IronyModManager)
//! (`IronyModManager.Services.Common/IModMergeService.cs`).
//!
//! Full collection zip/export orchestration depends on `io`, `storage`, and game/mod models. This
//! module defines the surface and a no-op stub until those layers are wired.

/// Error type for future async merge/export operations (`thiserror` for library boundaries).
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum ModMergeServiceError {
    #[error("mod merge service is not implemented yet")]
    NotImplemented,
}

/// Irony `IModMergeService` — collection merge permissions, templates, and export entry points.
/// Methods are synchronous stubs matching the C# names where practical; async variants can be added
/// when `tokio` call sites exist.
pub trait ModMergeService {
    fn allow_mod_merge(&self, collection_name: &str) -> bool;

    fn merge_collection_mod_name_template(&self) -> String;

    fn merge_collection_name_template(&self) -> String;

    fn has_enough_free_space(&self, collection_name: &str) -> bool;

    /// Placeholder for `MergeCollectionByFilesAsync` — returns `NotImplemented` until `IMod` exists in Rust.
    fn merge_collection_by_files(&self, collection_name: &str) -> Result<(), ModMergeServiceError>;

    fn merge_compress_collection(
        &self,
        collection_name: &str,
        copied_name_prefix: &str,
    ) -> Result<(), ModMergeServiceError>;

    fn save_merge_collection_mod_name_template(&self, template: &str) -> bool;

    fn save_merged_collection_name_template(&self, template: &str) -> bool;
}

/// Stub implementation: safe defaults (`false` / empty) until game and collection services exist.
#[derive(Debug, Default, Clone, Copy)]
pub struct ModMergeServiceStub;

impl ModMergeService for ModMergeServiceStub {
    fn allow_mod_merge(&self, _collection_name: &str) -> bool {
        false
    }

    fn merge_collection_mod_name_template(&self) -> String {
        String::new()
    }

    fn merge_collection_name_template(&self) -> String {
        String::new()
    }

    fn has_enough_free_space(&self, _collection_name: &str) -> bool {
        false
    }

    fn merge_collection_by_files(
        &self,
        _collection_name: &str,
    ) -> Result<(), ModMergeServiceError> {
        Err(ModMergeServiceError::NotImplemented)
    }

    fn merge_compress_collection(
        &self,
        _collection_name: &str,
        _copied_name_prefix: &str,
    ) -> Result<(), ModMergeServiceError> {
        Err(ModMergeServiceError::NotImplemented)
    }

    fn save_merge_collection_mod_name_template(&self, _template: &str) -> bool {
        false
    }

    fn save_merged_collection_name_template(&self, _template: &str) -> bool {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stub_returns_false_and_errors() {
        let s = ModMergeServiceStub;
        assert!(!s.allow_mod_merge("c"));
        assert!(s.merge_collection_mod_name_template().is_empty());
        assert!(matches!(
            s.merge_collection_by_files("x"),
            Err(ModMergeServiceError::NotImplemented)
        ));
    }
}
