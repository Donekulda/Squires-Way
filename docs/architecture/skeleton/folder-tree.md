# Source folder structure

SquiresWay library and binary entrypoints (`src/`).

```
src/
├── lib.rs
├── main.rs
├── core/
│   ├── mod.rs
│   ├── enums.rs
│   ├── models.rs
│   └── version.rs
├── io/
│   ├── mod.rs
│   ├── constants.rs
│   ├── error.rs
│   ├── json.rs
│   └── path_ops.rs
├── merging/
│   ├── mod.rs
│   ├── format_code.rs
│   ├── merge_top_level.rs
│   └── parser_merger.rs
├── parser/
│   ├── mod.rs
│   └── search/
│       ├── mod.rs
│       ├── constants.rs
│       ├── converters.rs
│       ├── enums.rs
│       ├── fields.rs
│       ├── filter_parser.rs
│       ├── localization_registry.rs
│       └── results.rs
├── platform/
│   ├── mod.rs
│   ├── clipboard.rs
│   ├── configuration.rs
│   └── linux_display.rs
├── services/
│   ├── mod.rs
│   ├── integration.rs
│   ├── merge.rs
│   └── resolver/
│       ├── mod.rs
│       ├── game_root_path.rs
│       ├── linux_flatpak_resolver.rs
│       └── path_resolver.rs
└── storage/
    ├── mod.rs
    ├── error.rs
    └── paths.rs
```
