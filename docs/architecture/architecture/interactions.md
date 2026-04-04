# Component interactions

## Overview

SquiresWay is structured in layers: **core** at the bottom (types), **io** and **storage** for paths and files, **parser** for search/filter logic, **merging** for pure merge helpers, and **services** on top for resolution and glue. **`lib.rs`** exposes modules to the binary and tests; **`main.rs`** is the process entry until the Bevy UI is wired.

## Key relationships

- **core** → (nothing in-crate): foundational types for `parser` and others.
- **io** → (nothing in-crate): primitives used by `storage` and `services::resolver`.
- **storage** → **io**: `paths` uses `path_ops` for env expansion and relative resolution.
- **parser** → **core**: search `results` / `converters` use `GameVersion` and related helpers.
- **services::resolver** → **io**: `path_resolver` uses `path_ops` for segment handling.
- **services::integration** → **io**, documents **storage**: combines `PathResolver` with relative path resolution; tests call `storage::resolve_storage_path`.
- **merging** → (nothing in-crate): standalone merge logic for future parser integration.

## Data flow

1. **Config / paths**: User or settings supply path strings → `storage::resolve_storage_path` or `services::resolve_parsed_path_against_base` → concrete `PathBuf` → `io` read/write as needed.
2. **Search**: User filter text → `parser::search::filter_parser` and converters → structured results (with `core` version types where applicable).

## Entry points

- **Library**: `src/lib.rs` — module graph for integration tests and future Bevy app.
- **Binary**: `src/main.rs` — placeholder `main`.
- **Tests**: `#[cfg(test)]` modules colocated with implementation; integration patterns in `services::integration` tests.
