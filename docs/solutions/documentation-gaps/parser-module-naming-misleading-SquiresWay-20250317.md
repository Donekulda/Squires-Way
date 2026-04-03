---
module: Parser
date: 2025-03-17
problem_type: documentation_gap
component: documentation
symptoms:
  - "Parser module name suggests game file parsing only"
  - "Module actually contains app control (search/filter) and game file parsing (definitions)"
root_cause: inadequate_documentation
resolution_type: documentation_update
severity: low
tags: [parser, naming, csharp-rust-conversion, irony-mod-manager, architecture]
---

# Troubleshooting: Parser Module Naming Is Misleading

## Problem

The IronyModManager C# "Parser" project (and its Rust port under `src/parser/`) is named "parser," which suggests it only parses game files. In reality, it mixes two distinct concerns: **application control** (search/filter UI) and **game file parsing** (Clausewitz/PDX definitions). This causes confusion when deciding where to add code or how to structure the Rust conversion.

## Environment

- Module: Parser
- Project: SquiresWay (C#→Rust conversion of IronyModManager)
- Affected Component: `src/parser/`, `IronyModManager.Parser`, `IronyModManager.Parser.Common`
- Date: 2025-03-17

## Symptoms

- "Why is it parser, when its used more for the Application control then anything else, but also for the game files"
- Unclear whether new filter/search logic belongs in `parser` or elsewhere
- Module name does not reflect that it handles both app control and game file parsing

## What Didn't Work

**Direct solution:** The naming comes from the original Irony project structure. No prior fix was attempted; the issue was clarified through analysis.

## Solution

**Document the dual role of the Parser module:**

1. **Application control / search** (what we ported first):
   - `Mod.Search.Parser` — parses user filter/search text (e.g. "version 1.0", "steam", "yes") into structured results
   - `BoolConverter`, `SourceTypeConverter`, `VersionConverter` — convert localized filter commands into typed values
   - `LocalizationRegistry` — locale-aware translations for filter commands
   - **Used for:** Filter UI, mod search, filters by version/source/achievements/selected

2. **Game file parsing** (not yet ported):
   - `IParserManager` — parses game/mod files → `IEnumerable<IDefinition>`
   - `ICodeParser` — parses Clausewitz script text
   - `ModParser` — parses `.mod` descriptor files
   - `DLCParser` — parses DLC info
   - Game-specific parsers (HOI4, Stellaris, etc.)
   - **Used for:** Actual Paradox/Clausewitz game file parsing

**Rust structure options for future clarity:**

- Keep `parser` as umbrella, use submodules: `parser::search` (app control) vs `parser::game` or `parser::definitions` (game files)
- Or split: `search` / `filter` for app control, `game_parser` for game file parsing

## Why This Works

The original C# project structure lumps both concerns under one "Parser" project because both involve "parsing text → structured output." The name is misleading because:
- The **search parser** part is app control logic (filter UI)
- The **game parsers** part is actual game file parsing

Documenting this clarifies where new code belongs and helps future conversion work avoid confusion.

## Prevention

- When adding new parser-related code, ask: "Is this for app control (filter/search) or game file parsing?"
- Consider adding a brief README or module-level doc in `src/parser/mod.rs` explaining the dual role
- Reference this doc when onboarding or when the naming is questioned

## Related Issues

No related issues documented yet.
