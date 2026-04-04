# Dependency diagrams

Module-level `crate::` dependencies in `src/` (approximate layering).

## Module dependencies

```mermaid
flowchart TB
    subgraph leaf [No in-crate deps]
        merging[merging]
    end
    subgraph foundation [Foundation]
        core[core]
        io[io]
    end
    subgraph mid [Mid layer]
        storage[storage]
        parser[parser]
    end
    subgraph top [Orchestration]
        services[services]
    end
    core --> parser
    io --> storage
    io --> services
    storage --> services
```

Notes:

- **parser** imports **core** only (among non-parser modules).
- **storage** imports **io**.
- **services** imports **io**; **storage** appears in integration tests and docs, not as a required `use` in every resolver file.
- **merging** is currently isolated until wired to a parser pipeline.

## Call flow (path resolution)

```mermaid
sequenceDiagram
    participant Caller
    participant Services as services::integration
    participant PR as PathResolver
    participant IO as io::path_ops
    Caller->>PR: parse path tokens
    PR->>IO: segment helpers
    Services->>IO: resolve_relative_path
    Services-->>Caller: PathBuf
```

## Call flow (filter search)

```mermaid
sequenceDiagram
    participant UI as Future_UI_or_agent
    participant FP as parser::search::filter_parser
    participant Conv as converters
    participant Core as core::version
    UI->>FP: filter string
    FP->>Conv: typed conversion
    Conv->>Core: version parse
    Conv-->>FP: values
    FP-->>UI: SearchParserResult
```
