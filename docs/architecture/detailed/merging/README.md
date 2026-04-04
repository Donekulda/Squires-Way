# merging

## Overview

Ports merge-related behavior from Irony’s parser/services: combining definition trees when mods patch the same files. Landed slices include **pure** list merge logic compatible with `ParserMerger.MergeTopLevel`, parser merger orchestration, and formatting helpers aligned with Irony `FormatCode` / merge pipeline work.

## Root modules

- **Purpose**: expose merge helpers and merger orchestration for script element trees.
- **Key files**: `mod.rs`, `merge_top_level.rs`, `parser_merger.rs`, `format_code.rs`
- **Key types**: `ScriptElement`, `merge_top_level_children`, merger entry points and format helpers as exposed in `mod.rs`.
- **Responsibilities**: deterministic merge of top-level children by target key / append mode; callers supply parse trees and serialize via `format_code` where applicable.
