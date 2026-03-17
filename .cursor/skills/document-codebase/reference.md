# Document Codebase - Reference

## Output Schemas

### Stage 1: Skeleton
- `folder-tree.md`: Markdown tree mirror of source structure
- `toc.yml`: Root table of contents
- `index.md`: Placeholder with project name and link to skeleton

### Stage 2: Overview
- `folder-purposes.md`: Hierarchical list of folder purposes
  - Format: `## FolderName\nPurpose (1-2 sentences)\n### SubfolderName\nPurpose (1 sentence)`

### Stage 3: Detailed
- `detailed/[FolderName]/README.md`: Per-folder documentation
  - Sections: Overview, Key Files, Key Classes/Types, Responsibilities

### Stage 4: Combined
- `combined/[ComplexFolder]/README.md`: Aggregated summary of subfolders
  - Only for folders with 2+ subfolders

### Stage 5: Architecture
- `interactions.md`: How components interact (narrative + links)
- `dependencies.md`: Mermaid diagrams for call flow and data flow

## Path Resolution

- **Source**: From `$ARGUMENTS[0]` or `$0`
- **Output**: From `$ARGUMENTS[1]` or `$1`; if omitted, use `path.dirname(source) + '/docs'`
- **Config override**: If `config.yaml` exists in output dir, read `source_dir` and `output_dir` from it

## config.yaml Schema

Location: `{output_dir}/config.yaml` or alongside the skill as `config-template.yaml`.

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `source_dir` | string | Yes | Path to source folder to document |
| `output_dir` | string | Yes | Path to docs output folder |
| `include_patterns` | string[] | No | Glob patterns for files to include (default: `*.cs`, `*.csproj`, `*.sln`) |
| `exclude_patterns` | string[] | No | Glob patterns to exclude (default: `bin/**`, `obj/**`, `.git/**`, `*.dll`) |

### Override Priority

1. **Arguments**: `$0` and `$1` always override config when provided
2. **Config file**: If no `$1`, and `config.yaml` exists in output dir, use `source_dir` and `output_dir` from config
3. **Default**: `output = path.dirname(source) + '/docs'`

### Reusability

Copy `config-template.yaml` to `docs/config.yaml` and edit `source_dir`/`output_dir` to run the workflow on different projects without changing skill arguments.

## Stage Invocation

To run from a specific stage (e.g., after Stage 2 completed):
- Pass `--stage 3` in arguments or mention "start from stage 3"
- Agent skips stages 1–2 and reads their output from docs folder
