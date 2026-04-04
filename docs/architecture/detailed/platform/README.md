# platform

## Overview

Thin slice of Irony `IronyModManager.Platform`: **data-only** and **trait** surfaces that do not depend on Avalonia/Skia. Intended for Bevy/UI layers and future **`game`** integration (`docs/conversion-parts/conversion-parts.yaml`). Renderer-specific code (themes, Skia caches, Windows volume listeners) stays out of this module until explicitly in scope.

## Root modules

- **Purpose**: expose configuration, Linux display helpers, clipboard abstraction, and `host_os()`.
- **Key files**: `mod.rs` (scope + APTV summary), `configuration.rs`, `linux_display.rs`, `clipboard.rs`
- **Key types**: `PlatformConfigurationOptions`, nested option structs, `PlatformConfiguration`, `Clipboard` / `ClipboardPolicy` / `ClipboardError`, `NoopClipboard`, display string constants and predicates.
- **Responsibilities**: persistable platform settings (serde JSON with PascalCase keys and explicit renames for `UseEGL` / `UseGPU` / `UseDBusMenu`); ASCII case-insensitive display server labels; clipboard contract for UI implementations.

## Integration notes

- **Clipboard policy:** Irony applies `PreventMultiLineText` inside the clipboard implementation. Here, `ClipboardPolicy::normalize_text` is separate from `Clipboard`; UI code should normalize before `set_text` / after `get_text` when parity with Irony is required.
- **Settings files:** Full JSON shape may differ from a flat `PlatformConfigurationOptions` tree depending on how the app loads config; field names on the wire match C# property names where serde attributes apply.
