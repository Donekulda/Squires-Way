//! OS and UI-shell concerns ported incrementally from Irony `IronyModManager.Platform`.
//!
//! # Scope (this milestone)
//! - **In scope:** configuration DTOs (`PlatformConfigurationOptions`), Linux display server string helpers,
//!   clipboard trait + policy + errors, host OS string for diagnostics.
//! - **Deferred:** Avalonia themes, Skia font caches, Windows volume listeners, and other renderer-specific types.
//!
//! # APTV summary
//! - **Analyze:** C# uses interfaces (`IPlatformConfiguration`, `IIronyClipboard`) and POCOs; heavy UI lives in Avalonia/Skia.
//! - **Plan:** `struct` + `trait` + `Result` + `thiserror`; UTF-8 `String`/`str` only; paths via `std::path` at call sites.
//! - **Transform:** Data structures and pure helpers first; clipboard integration behind `Clipboard` trait.
//! - **Validate:** Unit tests for JSON roundtrip and string predicates; `cargo fmt`, `clippy`, `test`, `build`.

mod clipboard;
mod configuration;
mod linux_display;

pub use clipboard::{Clipboard, ClipboardError, ClipboardPolicy, NoopClipboard};
pub use configuration::{
    AppOptions, ConflictSolverOptions, FontOptions, LinuxOptions, LoggingOptions,
    PlatformConfiguration, PlatformConfigurationOptions, TitleBarOptions, TooltipOptions,
    UpdateOptions,
};
pub use linux_display::{AUTO, WAYLAND, X11, is_auto, is_wayland, is_x11};

/// `std::env::consts::OS` — useful for branching in `game` / resolver code without extra deps.
#[must_use]
pub fn host_os() -> &'static str {
    std::env::consts::OS
}
