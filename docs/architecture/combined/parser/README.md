# parser — combined summary

## Overview

The `parser` crate module is the home for **application parser behavior** ported from Irony: today that means the **search** subsystem (filter strings, converters, localization). Clausewitz file parsing and jomini integration will attach here or in sibling modules as conversion continues.

## Subfolder summaries

### search

See [detailed/parser/README.md](../detailed/parser/README.md#search). Implements filter parsing, typed converters (version, bool, source, …), field registry, and structured results. Depends on **`core`** (e.g. `GameVersion`) for version semantics.

## Cross-cutting concerns

- **Inward deps**: `core` only (via `parser::search` results/converters).
- **Outward**: other crates (`services`, future UI) call into `parser::search` for filtering; `parser` does not import `services` or `storage`.
