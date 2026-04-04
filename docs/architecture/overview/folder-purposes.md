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

Definition merge logic ported from Irony’s `ParserMerger` / patch merge flows. Pure merge helpers on script element trees plus formatting hooks; full pipeline integration is ongoing.

### (root files)

`merge_top_level` (list-level merge), `parser_merger` (orchestration toward Irony `ParserMerger`), `format_code` (text formatting helpers); `mod` re-exports the public API.

## platform

OS and UI-shell settings ported incrementally from Irony `IronyModManager.Platform`: **configuration DTOs** (PascalCase JSON interop with C#-style keys), **Linux display server** string helpers, **clipboard** trait + single-line policy + errors, and **`host_os()`** for diagnostics. Avalonia/Skia themes and native font caches are deferred; Bevy/UI wiring consumes this layer later.

### (root files)

`configuration` (options + `PlatformConfiguration` trait), `linux_display`, `clipboard`; `mod` documents scope and APTV notes.

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

`lib.rs` wires crate modules (`core`, `io`, `merging`, `parser`, `platform`, `services`, `storage`) for tests and library consumers. `main.rs` is the binary entry (placeholder until the Bevy app is wired).
