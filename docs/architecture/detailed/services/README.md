# services

## Overview

Orchestration layer (Irony `Services` slice): combines `core`, `io`, `parser`, and `storage` without those layers depending back on `services`. Exposes path resolution and small integration helpers (e.g. resolving a parsed path against a base directory).

## resolver

- **Purpose**: resolve game install paths, DLC paths, Flatpak Steam layouts, and tokenized path strings.
- **Key files**: `mod.rs`, `path_resolver.rs`, `game_root_path.rs`, `linux_flatpak_resolver.rs`
- **Key types**: `PathResolver`, `parse_path`, `GameRootPathResolver`, `GameRootPaths`, `LinuxFlatPakResolver`
- **Responsibilities**: OS-specific discovery (e.g. Linux Flatpak) and deterministic path joining for game roots.

## Root files

- **integration.rs**: glue between storage resolution and parsed path segments (re-exported from `mod` where appropriate).
