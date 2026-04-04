# parser

## Overview

Public parser module for the app. It currently centers on **search**: parsing filter text, converting values with localization, and producing structured filter results — mirroring Irony’s Parser scope (search + definitions), not a standalone “file parser only” crate.

## search

- **Purpose**: mod-definition search/filter pipeline (Irony-inspired).
- **Key files**: `mod.rs`, `filter_parser.rs`, `converters.rs`, `fields.rs`, `enums.rs`, `constants.rs`, `localization_registry.rs`, `results.rs`
- **Key types**: filter AST/results, converters (bool, version, source, …), localization registry, field metadata.
- **Responsibilities**: parse user filter strings, apply negation/OR, map localized labels, and produce results for UI or agents.
