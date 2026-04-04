# core

## Overview

Domain types shared across the crate: game/mod enums, mod and collection models, and version string handling. This layer should stay free of filesystem and async concerns so tests and upper layers can depend on it cheaply.

## Root modules

- **Purpose**: expose `enums`, `models`, `version` as submodules.
- **Key files**: `mod.rs`, `enums.rs`, `models.rs`, `version.rs`
- **Key types**: types in `enums` and `models` (e.g. mod descriptors, queryable models); version helpers in `version`.
- **Responsibilities**: stable data shapes for UI, search, and future game integration.
