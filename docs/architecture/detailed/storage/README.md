# storage

## Overview

Application storage layout (Irony `Storage` slice): default data directories, JSON store roots, and resolving storage paths with environment expansion. Uses `io` for actual reads/writes.

## Root modules

- **Purpose**: path resolution for app data and errors for storage operations.
- **Key files**: `mod.rs`, `paths.rs`, `error.rs`
- **Key types**: `StorageError`; `default_data_dir`, `json_store_root`, `resolve_storage_path`
- **Responsibilities**: cross-platform base dirs (via `directories` / conventions) and consistent path joining for persisted state.
