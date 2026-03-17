# Conversion Parts Schema

Schema for `conversion-parts.yaml` — the configurable manifest for incremental C#→Rust conversion.

## Root Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `phases` | `string[]` | Yes | Ordered list of phase IDs (conversion order) |
| `parts` | `Part[]` | Yes | List of conversion parts |
| `current_targets` | `string[]` | Yes | Part IDs to convert in current run (user configures) |

## Part Object

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `id` | `string` | Yes | Unique part identifier (e.g. `core`, `parser`) |
| `name` | `string` | Yes | Human-readable name |
| `projects` | `string[]` | Yes | C# project names (e.g. `IronyModManager.Common`) |
| `dependencies` | `string[]` | No | Part IDs this part depends on (for validation) |
| `phase` | `string` | Yes | Phase this part belongs to |
| `enabled` | `boolean` | No | Whether part is included in conversion (default: true) |

## Validation Rules

1. `current_targets` must only contain valid part `id` values
2. For each part in `current_targets`, all entries in `dependencies` must either be in `current_targets` or already converted (tracked separately)
3. `phases` order defines recommended conversion order (dependency-first)

## Example

```yaml
phases: [core, parser, io, storage, services, merging, platform, game, localization, di, app, peripheral]

parts:
  - id: core
    name: Core Models
    projects: [IronyModManager.Common, IronyModManager.Models.Common, IronyModManager.Models, IronyModManager.Shared]
    dependencies: []
    phase: core
    enabled: true

  - id: parser
    name: Parser
    projects: [IronyModManager.Parser.Common, IronyModManager.Parser]
    dependencies: [core]
    phase: parser
    enabled: true

current_targets: [core]
```
