# Folder purposes

High-level map of `src/` (SquiresWay Rust crate). Conversion work is tracked against IronyModManager (C#) per `docs/conversion-parts/`.

## core

Shared domain types: enums, mod/collection models, and version parsing. No I/O; safe for any layer to depend on.

### (root files)

Flat modules: `enums`, `models`, `version` — core data shapes used across parser, storage, and services.

## io

Filesystem and format helpers: JSON read/write, path normalization, env-style path segments. Corresponds to the Irony IO slice; orchestration belongs in `storage` / upper layers.

### (root files)

`constants`, `path_ops`, `json`, and `error` — low-level operations only.

## merging

Definition merge logic ported from Irony’s `ParserMerger` / patch merge flows. Currently exposes pure merge helpers on script element trees; full parse/format integration is future work.

### (root files)

`merge_top_level` implements list-level merge behavior; `mod` re-exports public API.

## parser

Application-facing parser surface. Today exposes **search** (filter text, converters, localization registry) — aligned with Irony’s Parser part as “control + definitions,” not raw file parsing only.

### search

Filter parsing, field metadata, converters, localization registry, and result types for mod-definition search UX.

## services

Cross-cutting orchestration: path resolution (game root, Flatpak Steam, tokenized paths), and integration helpers that combine `io` / `storage` with resolved paths.

### resolver

`PathResolver`, `GameRootPathResolver`, `LinuxFlatPakResolver` — platform and layout-aware path resolution.

## storage

Application data directories and resolved storage paths (e.g. JSON store roots). Uses `directories` / env expansion patterns; delegates raw reads to `io`.

### (root files)

`paths` for layout; `error` for storage-specific failures.

## Crate roots (`lib.rs`, `main.rs`)

`lib.rs` wires crate modules for tests and library consumers. `main.rs` is the binary entry (placeholder until the Bevy app is wired).
