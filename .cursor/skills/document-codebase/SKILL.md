---
name: document-codebase
description: Generate multi-stage documentation for a codebase folder. Use when documenting codebases, analyzing project structure, or creating architecture docs. Triggers on "document this folder", "analyze codebase", "generate docs for", "document IronyModManager".
argument-hint: [source-folder] [output-folder]
context: fork
agent: Explore
---

# Document Codebase

Multi-stage, multi-agent workflow to document a source folder. Output always goes to a `docs` folder.

## Quick Start

```
/document-codebase Examples/IronyModManager/src Examples/IronyModManager/docs
```

Or with default output (parent of source + `/docs`):
```
/document-codebase Examples/IronyModManager/src
```

## Path Resolution

1. **Source**: `$ARGUMENTS[0]` or `$0` (required)
2. **Output**: `$ARGUMENTS[1]` or `$1`; if omitted, use `{parent of source}/docs`

Example: `Examples/IronyModManager/src` → output `Examples/IronyModManager/docs`

## TodoWrite Setup

At workflow start, create a TodoWrite with all stages:

```
- document-codebase-skeleton (Stage 1)
- document-codebase-overview (Stage 2)
- document-codebase-detailed (Stage 3)
- document-codebase-combined (Stage 4)
- document-codebase-interactions (Stage 5)
```

Mark each complete when done. Run stages sequentially; do not skip ahead.

## Workflow Steps

| Stage | Name | Output |
|-------|------|--------|
| 1 | Skeleton | docs structure, folder-tree.md, toc.yml, index.md |
| 2 | Overview | overview/folder-purposes.md |
| 3 | Detailed | detailed/[FolderName]/README.md per folder |
| 4 | Combined | combined/[ComplexFolder]/README.md |
| 5 | Interactions | architecture/interactions.md, dependencies.md |

### Stage 1: Skeleton
Read [stages/stage1-skeleton.md](stages/stage1-skeleton.md) for checklist. Actions:
- List directory tree of source (exclude bin, obj, .git, *.dll)
- Create docs/ with: skeleton/, overview/, detailed/, combined/, architecture/
- Write skeleton/folder-tree.md mirroring source structure
- Create root toc.yml and index.md placeholder

### Stage 2: Overview
0Read [stages/stage2-overview.md](stages/stage2-overview.md). Actions:
- For each top-level folder: identify purpose (1–2 sentences)
- For each subfolder: identify purpose (1 sentence)
- Output to overview/folder-purposes.md

### Stage 3: Detailed
Read [stages/stage3-detailed.md](stages/stage3-detailed.md). Dispatch one subagent per top-level folder (parallel where possible). Each subagent:
- For each subfolder: list key files, classes, responsibilities
- Write detailed/[FolderName]/README.md
- Add toc.yml for navigation

### Stage 4: Combined
Read [stages/stage4-combined.md](stages/stage4-combined.md). Actions:
- Aggregate subfolder docs into parent-level summaries
- Write combined/[ComplexFolder]/README.md for folders with multiple subfolders
- Update toc.yml

### Stage 5: Interaction Mapping
Read [stages/stage5-interactions.md](stages/stage5-interactions.md). Actions:
- Identify cross-folder dependencies (imports, references)
- Generate Mermaid diagrams for call flow and data flow
- Write architecture/interactions.md and architecture/dependencies.md

## Incremental Runs

To start from a specific stage (e.g., after Stage 2 completed):
- User says "start from stage 3" or passes `--stage 3`
- Skip stages 1–2; read their output from docs folder; proceed from Stage 3

## Config

If `config.yaml` exists in output dir, read `source_dir` and `output_dir`. Use config-template.yaml as reference. Include/exclude patterns control which files are analyzed.

## Additional Resources

- Stage details and output schemas: [reference.md](reference.md)
- Per-stage prompts: [stages/](stages/)
- IronyModManager setup: [setup-irony-mod-manager.md](setup-irony-mod-manager.md)
