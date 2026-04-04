# merging

## Overview

Ports merge-related behavior from Irony’s parser/services: combining definition trees when mods patch the same files. The first landed slice is **pure** list merge logic compatible with `ParserMerger.MergeTopLevel` (child ordering under a root block); parsing and `FormatCode` remain in C# parity work.

## Root modules

- **Purpose**: expose merge helpers for script element trees.
- **Key files**: `mod.rs`, `merge_top_level.rs`
- **Key types**: `ScriptElement`, `merge_top_level_children`
- **Responsibilities**: deterministic merge of top-level children by target key / append mode; callers must supply parse trees and later serialize.
