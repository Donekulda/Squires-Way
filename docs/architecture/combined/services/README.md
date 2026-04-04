# services — combined summary

## Overview

`services` is the **orchestration** layer: path resolution for game installs and Flatpak Steam, plus small integration helpers that tie **`io`** path operations to resolved bases. It is the right place for future mod-collection workflows that compose `parser`, `storage`, and I/O.

## Subfolder summaries

### resolver

See [detailed/services/README.md](../detailed/services/README.md#resolver). Contains `PathResolver`, game-root/DLC resolution, and Linux Flatpak heuristics. Uses **`io::path_ops`** for segment parsing and relative joins.

## Cross-cutting concerns

- **Inward deps**: `io` (path_resolver, integration); `storage` referenced from docs/tests for `resolve_storage_path`; **`parser` is not imported** inside `services` modules (call sites may use both).
- **Integration**: `integration.rs` combines `PathResolver` with `io::path_ops::resolve_relative_path`; tests demonstrate `storage::resolve_storage_path` usage from the services layer.
