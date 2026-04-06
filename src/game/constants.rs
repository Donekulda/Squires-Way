//! Timing and string constants from Irony `IronyModManager.IO.Platforms.SteamHandler`.

/// Milliseconds between polls when waiting for Steam (C# `Delay` = 250).
pub const STEAM_POLL_DELAY_MS: u64 = 250;

/// Max iterations for the direct Steam init loop (C# `MaxAttempts` = 60 * 4).
pub const STEAM_DIRECT_MAX_ATTEMPTS: u32 = 60 * 4;

/// Milliseconds between polls in alternate mode (C# `AlternateDelay` = 3000).
pub const STEAM_ALTERNATE_POLL_DELAY_MS: u64 = 3000;

/// Max iterations waiting for Steam process in alternate mode (C# `AlternateMaxAttempts` = 20).
pub const STEAM_ALTERNATE_MAX_ATTEMPTS: u32 = 20;

/// File written next to the executable on macOS with the Steam app id (C# `steam_appid.txt`).
pub const STEAM_APP_ID_FILE: &str = "steam_appid.txt";

/// Process name used to detect a running Steam client (C# `SteamProcess` = "steam").
pub const STEAM_PROCESS: &str = "steam";

/// `steam://open/main` — used to nudge Steam to start (C# `SteamLaunch`).
pub const STEAM_LAUNCH_MAIN_URI: &str = "steam://open/main";
