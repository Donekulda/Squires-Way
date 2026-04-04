# platform (combined)

## Summary

The **`platform`** area is a **standalone** layer: it does not import other SquiresWay modules (`core`, `io`, etc.). It exists so UI and **`game`** (launcher / CK3 integration) can share:

- **Settings DTOs** — mirrors Irony `PlatformConfigurationOptions` for JSON interop.
- **Linux display labels** — `auto` / `wayland` / `x11` predicates for options UI and validation.
- **Clipboard** — trait + policy for single-line normalization; concrete system clipboard wiring belongs in the Bevy/UI crate.

## Relationship to other folders

| Folder | Relationship |
|--------|----------------|
| **services** | May read `host_os()` or platform options when resolving paths; no hard dependency required today. |
| **game** (future) | Expected consumer of Linux options and shell behavior; depends on **`platform`** per `conversion-parts.yaml`. |
| **parser / merging / storage** | No direct dependency on **platform**. |

See [detailed/platform/README.md](../detailed/platform/README.md) for file-level detail.
