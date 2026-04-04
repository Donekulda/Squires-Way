# io

## Overview

Low-level I/O aligned with the Irony `IronyModManager.IO` / `IO.Common` slice: JSON helpers, path segment resolution (`$VAR`, relative segments), and shared I/O errors. Policy (where files live) belongs in `storage`, not here.

## Root modules

- **Purpose**: JSON and path primitives for the rest of the crate.
- **Key files**: `mod.rs`, `constants.rs`, `error.rs`, `json.rs`, `path_ops.rs`
- **Key types**: `IoError`; JSON helpers (`read_json_file`, `write_json_file`, etc.); path ops (`standardize_separator`, `resolve_env_segment`, etc.).
- **Responsibilities**: UTF-8 text and JSON on disk; OS path boundaries via `Path`/`OsStr` patterns as implemented in `path_ops`.
