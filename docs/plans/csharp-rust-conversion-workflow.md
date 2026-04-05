# C# to Rust Conversion Workflow

Multi-phase workflow for incremental IronyModManager C#→Rust conversion into SquiresWay `src/`.

## Overview

- **Source:** IronyModManager (C# Paradox mod manager) — clone to `Examples/IronyModManager`
- **Target:** SquiresWay `src/` (Rust mod loader)
- **Approach:** Incremental, dependency-first, module-by-module

## Phase 0: Preparation

1. Clone IronyModManager:
   ```powershell
   .\.cursor\skills\document-codebase\scripts\setup-irony.ps1
   ```
   Or manually: `git clone https://github.com/bcssov/IronyModManager.git Examples/IronyModManager`

2. (Optional) Run document-codebase for richer context:
   ```
   /document-codebase Examples/IronyModManager/src Examples/IronyModManager/docs
   ```

3. Run analyze-csharp-conversion-parts skill:
   ```
   /analyze-csharp-conversion-parts Examples/IronyModManager/src docs/conversion-parts
   ```
   This produces `docs/conversion-parts/conversion-parts.yaml` and `dependency-graph.md`.

## Phase 1: Configure Targets

1. Edit `docs/conversion-parts/conversion-parts.yaml`
2. Set `current_targets` to the part(s) to convert, e.g.:
   ```yaml
   current_targets: [core]
   ```
   Or for multiple: `current_targets: [core, parser]`

3. Ensure `current_targets` respects dependency order: dependencies must be converted first (or included in same run).

## Phase 2: Convert per Target

For each part in `current_targets`:

1. Load the convert-csharp-rust skill
2. Apply APTV workflow:
   - **Analyze** — Inventory C# features, hot paths, public API
   - **Plan** — Feature mapping, error model, crate substitutions
   - **Transform** — Port by module with tests alongside
   - **Validate** — Parity tests, `cargo fmt`, `cargo clippy`, `cargo test`

3. Output Rust modules under `src/`:
   - `src/core/` for core part
   - `src/parser/` for parser part
   - etc.

4. **OS-interacting code:** Split by `#[cfg(windows)]` / `#[cfg(unix)]` (and `target_os` when Linux vs macOS differ). Use `OsString`/`OsStr` and `Path` at OS boundaries; keep UTF-8 internally per `AGENTS.md`. See `.cursor/rules/rust-os-boundaries.mdc` and `src/platform/wide.rs`.

## Phase 3: Validate

```bash
cargo fmt --check
cargo clippy -- -D warnings
cargo test
```

Run parity checks if available (e.g. compare output with C# version on same inputs).

## Phase 4: Iterate

1. Update `current_targets` in `conversion-parts.yaml` for next phase
2. Repeat Phase 2–3

## Suggested Conversion Order (Dependency-First)

| Phase | Part | Projects |
|-------|------|----------|
| 1 | core | Common, Models.Common, Models, Shared |
| 2 | game | GameHandler |
| 3 | parser | Parser.Common, Parser |
| 4 | io | IO.Common, IO |
| 5 | storage | Storage.Common, Storage |
| 6 | app | IronyModManager |
| 7 | services | Services.Common, Services |
| 8 | merging | (subset of Services/Parser) |
| 9 | platform | Platform |
| 10 | localization | Localization |
| 11 | di | DI |
| 12 | peripheral | Updater, AppCastGenerator |

## Integration

- **convert-csharp-rust** — Used in Phase 2 for actual conversion
- **document-codebase** — Optional Phase 0
- **analyze-csharp-conversion-parts** — Produces manifest and dependency graph
- **setup-irony.ps1** — Clone IronyModManager
- **deepen-plan** — Can deepen this workflow doc with research
