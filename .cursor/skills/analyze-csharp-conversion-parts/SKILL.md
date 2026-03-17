---
name: analyze-csharp-conversion-parts
description: Analyze a C# codebase and divide it into conversion parts for incremental Rust migration. Produces a configurable parts manifest (conversion-parts.yaml) and dependency graph. Use when analyzing C# for conversion, dividing codebase for Rust migration, or setting conversion targets. Triggers on "analyze C# for conversion", "divide codebase for Rust migration", "conversion parts", "set conversion targets".
argument-hint: [source-path] [output-path]
---

# Analyze C# Conversion Parts

Analyze a C# codebase and produce a configurable parts manifest so you can set current conversion targets for incremental C#→Rust migration.

## Quick Start

```
/analyze-csharp-conversion-parts Examples/IronyModManager/src docs/conversion-parts
```

Or with default output (`docs/conversion-parts`):

```
/analyze-csharp-conversion-parts Examples/IronyModManager/src
```

## Path Resolution

1. **Source**: `$ARGUMENTS[0]` or `$0` (required) — path to C# src folder containing `.csproj` files
2. **Output**: `$ARGUMENTS[1]` or `$1`; if omitted, use `docs/conversion-parts`

## Workflow

1. List source projects (`.csproj` files under source path)
2. Parse project references from `.csproj` (or use [irony-parts-template.yaml](references/irony-parts-template.yaml) if IronyModManager)
3. Build dependency graph (Mermaid)
4. Map projects to logical parts (core, parser, io, merging, etc.)
5. Write `conversion-parts.yaml` with `current_targets: []` (user fills)
6. Write `dependency-graph.md` with Mermaid diagram

## Output Files

| File | Purpose |
|------|---------|
| `conversion-parts.yaml` | Configurable manifest; user sets `current_targets` |
| `dependency-graph.md` | Mermaid diagram of part dependencies |

## Schema

Read [references/conversion-parts-schema.md](references/conversion-parts-schema.md) for full schema.

Key fields:
- `parts`: list of `{ id, name, projects[], dependencies[], phase, enabled }`
- `current_targets`: list of part IDs to convert in current run
- `phases`: ordered list for dependency-first conversion

## IronyModManager Template

For IronyModManager, use [references/irony-parts-template.yaml](references/irony-parts-template.yaml) as the base. The template maps 27 projects to logical parts: core, parser, io, storage, services, merging, platform, game, localization, di, app, peripheral.

## Optional: Parse Dependencies

If NDepend is not available, run [scripts/analyze-deps.ps1](scripts/analyze-deps.ps1) to parse `.csproj` references and build a dependency graph. Pass source path as argument.

## Integration

- **convert-csharp-rust** — Use `current_targets` from manifest to drive conversion
- **document-codebase** — Optional: run before analysis for richer context
