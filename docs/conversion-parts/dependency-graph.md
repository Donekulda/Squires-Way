# IronyModManager Conversion Dependency Graph

Dependency order for incremental C#→Rust conversion. Convert parts in phase order; within a phase, parts can be converted in parallel if they share no dependencies.

## Part Dependencies

```mermaid
flowchart TD
    subgraph phase1 [Phase 1: Core]
        core[Core Models]
    end
    subgraph phase2 [Phase 2: Parser]
        parser[Parser]
    end
    subgraph phase3 [Phase 3: IO]
        io[IO]
    end
    subgraph phase4 [Phase 4: Storage]
        storage[Storage]
    end
    subgraph phase5 [Phase 5: Services]
        services[Services]
    end
    subgraph phase6 [Phase 6: Merging]
        merging[Merging]
    end
    subgraph phase7 [Phase 7: Platform]
        platform[Platform]
    end
    subgraph phase8 [Phase 8: Game]
        game[GameHandler]
    end
    subgraph phase9 [Phase 9: Localization]
        localization[Localization]
    end
    subgraph phase10 [Phase 10: DI]
        di[DependencyInjection]
    end
    subgraph phase11 [Phase 11: App]
        app[App]
    end
    subgraph phase12 [Phase 12: Peripheral]
        peripheral[Peripheral]
    end

    core --> parser
    core --> io
    core --> storage
    core --> localization
    core --> di
    core --> peripheral
    parser --> services
    io --> services
    storage --> services
    services --> merging
    core --> platform
    platform --> game
    merging --> app
    game --> app
```

## Conversion Order

1. **core** — Common, Models.Common, Models, Shared (no dependencies)
2. **parser**, **io**, **storage**, **platform**, **localization**, **di**, **peripheral** — depend on core
3. **services** — depends on parser, io, storage
4. **merging** — depends on services
5. **game** — depends on platform
6. **app** — depends on merging, game, localization, di
