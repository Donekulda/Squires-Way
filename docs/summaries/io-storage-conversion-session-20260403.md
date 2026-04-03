---
summary_type: conversion_session
date: 2026-04-03
project: SquiresWay
scope: [io, storage]
conversion_targets_ref: docs/conversion-parts/conversion-parts.yaml
related_reference: Examples/IronyModManager
tags: [csharp-rust-conversion, io, storage, utf-8, json, paths]
---

# Summary: IO and Storage Conversion Session (2026-04-03)

This note captures outcomes from an incremental C#→Rust conversion pass focused on the **io** and **storage** parts, aligned with `docs/plans/csharp-rust-conversion-workflow.md` and the authoritative `docs/conversion-parts/conversion-parts.yaml` (dependency graph and `current_targets` take precedence over informal ordering).

## Encoding policy

- **Application text and JSON use UTF-8 universally** (no carry-over of C# UTF-16 `string` semantics).
- **Read**: JSON/text readers accept UTF-8 with an optional UTF-8 BOM; BOM is stripped before decode.
- **Write**: JSON is written as UTF-8 **without** a BOM.
- **OS boundaries**: filesystem paths use `Path` / `OsStr` as appropriate.

This policy is reflected in `AGENTS.md` and in `src/io/json.rs` behavior.

## Implemented Rust surface (high level)

| Area | Location | Notes |
|------|----------|--------|
| JSON I/O | `src/io/json.rs` | `read_json_file` (BOM-tolerant), `write_json_file` (pretty-printed via `serde_json::to_vec_pretty`), parent directory creation on write |
| IO errors | `src/io/error.rs` | Variants for read/write, parent creation, JSON encode/decode |
| Re-exports | `src/io/mod.rs` | Public API includes `write_json_file` |
| Storage paths | `src/storage/paths.rs` | `json_store_root` (mirrors Irony `JsonStore`-style root: config base + optional app title segment); tests for title append behavior |
| Storage re-exports | `src/storage/mod.rs` | `json_store_root` exported |

## Validation performed

After substantive changes, the project was run through `cargo fmt`, `cargo clippy` (with `-D warnings`), tests, and `cargo build` per workspace conventions.

## Design–implementation review (follow-ups)

A structured review pass compared the Rust layer to Irony’s IO/storage behavior and the stated UTF-8 policy. **Suggested follow-ups** (not all implemented in this session):

1. **Environment segment parity**: In `resolve_env_segment` (`src/storage/paths.rs`), `$`-prefixed segments should match C# `TrimStart('$')` behavior (strip **all** leading `$` characters), not only one.
2. **`write_json_file` contract**: Document clearly that output is **pretty-printed**; if parity with Irony’s compact `Formatting.None` JSON is required, expose compact serialization (separate function or flag).
3. **Durability**: For critical persisted JSON, consider **atomic write** (temp file + rename/replace) to mirror Irony `JsonStore.SetData` and reduce partial-write risk on crash.
4. **Tests**: Add focused tests for `resolve_storage_path` covering `$` / `%` environment expansion edge cases.

## Open conversion backlog (from session tracking)

- Confirm `Examples/IronyModManager` presence; rerun manifest/analysis tooling only if the solution layout changes.
- Continue APTV toward parity with Irony **IO / IO.Common** and **Storage / Storage.Common** with tests.
- **Advance `current_targets`** in `conversion-parts.yaml` to downstream parts (e.g. **services**) when io+storage are ready for consumers.

## References

- Plan: `docs/plans/csharp-rust-conversion-workflow.md`
- Targets: `docs/conversion-parts/conversion-parts.yaml`
- Dependencies: `docs/conversion-parts/dependency-graph.md`
