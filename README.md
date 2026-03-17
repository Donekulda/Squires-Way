# Squires Way

Crusader Kings 3 Rust mod loader, based on [Irony Mod Manager](https://github.com/bcssov/IronyModManager).  
Cross-platform: **Windows**, **macOS**, **Linux**.

## Stack (multi-OS friendly)

Libraries are chosen for portability and broad adoption across desktop platforms.

| Area | Crate | Why |
|------|--------|-----|
| **UI** | [Bevy](https://bevyengine.org) + [bevy_egui](https://github.com/vladbat00/bevy_egui) | Bevy: single Rust stack, native windowing (winit) on all three OSes. bevy_egui: immediate-mode UI (lists, forms, buttons, text) with clipboard, accessibility (AccessKit), desktop + optional web. |
| **Parsing** | [jomini](https://github.com/rakaly/jomini) | Paradox Clausewitz (CK3, EU4, HOI4, Vic3) plaintext & binary; serde-like API, high throughput. |
| **Serialization** | serde, serde_json | De facto standard; works everywhere. |
| **Paths** | [directories](https://docs.rs/directories) | Config/cache/data dirs per platform (XDG on Linux, Known Folders on Windows, macOS Standard Directories). |
| **Filesystem** | walkdir, tempfile, zip | Cross-platform directory walking, temp files, and zip archives. |
| **Database** | [rusqlite](https://docs.rs/rusqlite) (bundled) | SQLite in-process; `bundled` feature for a single binary with no system SQLite dependency. |
| **Async & logging** | tokio, tracing, tracing-subscriber | Async runtime and structured logging with `RUST_LOG` env filter; no OS-specific logging. |

## UI (Bevy)

- **Bevy** provides the app loop, windowing (winit), input, and rendering on Windows, macOS, and Linux (and optionally web).
- **bevy_egui** plugs egui into Bevy for app-style UI: windows, panels, text input, lists, buttons. Use `EguiPlugin` and draw in a system with `EguiContexts`.

Linux: install XCB dev libraries (e.g. `libxcb-render0-dev`, `libxcb-shape0-dev`, `libxcb-xfixes0-dev`) if not already present.

## Parsing (jomini)

Parsing of Paradox script and save/game files is done with **[jomini](https://github.com/rakaly/jomini)** ([crates.io](https://crates.io/crates/jomini)):

- **Plaintext and binary** Clausewitz format
- **Serde-like** derive API (`JominiDeserialize`) with attributes for aliases, duplicates, `take_last`, etc.
- **High performance** (parsing at over 1 GB/s), zero dependencies by default
- **CK3 save files**: optional `envelope` feature for the structured container (ZIP-based or plaintext) used by CK3, EU5, Vic3, Imperator

The `envelope` feature is used for CK3 (and other modern PDS) save file metadata and gamestate; omit it if you only need generic Clausewitz text/binary parsing.

## Dependencies (Cargo.toml)

```toml
[dependencies]
bevy = "0.18"
bevy_egui = "0.39"
jomini = { version = "0.34", features = ["envelope"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
directories = "6"
walkdir = "2"
tempfile = "3"
zip = "2"
rusqlite = { version = "0.32", features = ["bundled"] }
tokio = { version = "1", features = ["rt-multi-thread", "fs", "io-util"] }
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter"] }
```