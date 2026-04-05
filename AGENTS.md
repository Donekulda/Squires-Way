## Learned User Preferences

- When running the C#→Rust conversion workflow, the user may skip Phase 0.1–0.3 (clone, optional document-codebase, regenerate manifest) and start at Phase 1 if Irony source or analysis is already in place.
- The user expects work to follow the phased conversion plan with explicit TODOs when they ask for task breakdowns.
- When implementing an attached plan: do not edit the plan file; todos are usually pre-created—mark them in progress and complete them without recreating the list.

## Learned Workspace Facts

- SquiresWay is a cross-platform Rust Crusader Kings 3 mod loader, ported from Irony Mod Manager (see README for upstream link and stack rationale).
- After substantive Rust changes, validate with `cargo fmt`, `cargo clippy`, `cargo test`, and `cargo build` (crate uses Rust edition 2024).
- Incremental C#→Rust conversion is driven by `docs/conversion-parts/conversion-parts.yaml` (`current_targets`, part order) and `docs/plans/csharp-rust-conversion-workflow.md`; use `.cursor/skills/convert-csharp-rust` and `analyze-csharp-conversion-parts` for conventions.
- Reference C# for parity is expected under `Examples/IronyModManager`; `.cursor/skills/document-codebase/scripts/setup-irony.ps1` can populate it when missing.
- `docs/solutions/` holds dated troubleshooting and documentation-gap notes (often YAML frontmatter), including architecture clarifications for converted areas.
- `docs/summaries/` holds narrative or session summaries when the user asks for summarization alongside plan or conversion work.
- `src/parser/` mirrors Irony’s Parser part: it combines application control (e.g. search/filter) with game-definition parsing—not “file parsing only”; treat new code accordingly.
- Crate layout includes `src/core/`, `src/parser/` (currently exposes `parser::search`), and the Bevy application entry in `src/main.rs`.
- Paradox Clausewitz plaintext/binary (and CK3-style envelopes) use the `jomini` crate with the `envelope` feature enabled in `Cargo.toml`.
- Application text and JSON on disk use **UTF-8**; C# `string` is UTF-16—do not port length/index semantics blindly. Prefer matching Windows wide APIs with UTF-16 only at Win32 boundaries while keeping Linux on `Path`/`OsStr` and `#[cfg]` splits (see `.cursor/rules/rust-os-boundaries.mdc`). Convert explicitly at boundaries when needed.


## Memory (Memstate MCP)

### Before each task
- memstate_get(project_id="SquiresWay") — browse existing knowledge
- memstate_search(query="topic", project_id="SquiresWay") — find by meaning

### After each task
- memstate_remember(project_id="SquiresWay", content="## Summary\n- ...", source="agent")

### Tool guide
- memstate_remember — markdown summaries, decisions, task results (preferred)
- memstate_set — single short values only (config flags, status)
- memstate_get — browse/retrieve before tasks
- memstate_search — semantic lookup when keypath unknown
- memstate_history — audit how knowledge evolved
- memstate_delete — remove outdated memories (history preserved)