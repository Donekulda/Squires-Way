## Learned User Preferences

- When running the C#→Rust conversion workflow, the user may skip Phase 0.1–0.3 (clone, optional document-codebase, regenerate manifest) and start at Phase 1 if Irony source or analysis is already in place.
- The user expects work to follow the phased conversion plan with explicit TODOs when they ask for task breakdowns.

## Learned Workspace Facts

- SquiresWay is a cross-platform Rust Crusader Kings 3 mod loader, ported from Irony Mod Manager (see README for upstream link and stack rationale).
- After substantive Rust changes, validate with `cargo fmt`, `cargo clippy`, `cargo test`, and `cargo build` (crate uses Rust edition 2024).
- Incremental C#→Rust conversion is driven by `docs/conversion-parts/conversion-parts.yaml` (`current_targets`, part order) and `docs/plans/csharp-rust-conversion-workflow.md`; use `.cursor/skills/convert-csharp-rust` and `analyze-csharp-conversion-parts` for conventions.
- Reference C# for parity is expected under `Examples/IronyModManager`; `.cursor/skills/document-codebase/scripts/setup-irony.ps1` can populate it when missing.
- `docs/solutions/` holds dated troubleshooting and documentation-gap notes (often YAML frontmatter), including architecture clarifications for converted areas.
- `src/parser/` mirrors Irony’s Parser part: it combines application control (e.g. search/filter) with game-definition parsing—not “file parsing only”; treat new code accordingly.
- Crate layout includes `src/core/`, `src/parser/` (currently exposes `parser::search`), and the Bevy application entry in `src/main.rs`.
- Paradox Clausewitz plaintext/binary (and CK3-style envelopes) use the `jomini` crate with the `envelope` feature enabled in `Cargo.toml`.
- Application text and JSON on disk use **UTF-8**; C# reference code uses UTF-16 `string`—do not port length/index semantics blindly. Use `Path`/`OsStr` at OS boundaries and convert explicitly when needed.
