//! Environment variables and `steam_appid.txt` handling (Irony `SteamHandler.SetAppId` / `CleanupAppId`).

use std::fs;
use std::path::Path;

use crate::game::constants::STEAM_APP_ID_FILE;

/// Sets `SteamAppId`, `SteamOverlayGameId`, and `SteamGameId` (Irony `SetAppId`).
///
/// Uses [`std::env::set_var`], which maps to `setenv` on Unix and matches Irony's Linux `libc::setenv` path.
pub fn set_steam_app_id_env_vars(app_id: u64) {
    let v = app_id.to_string();
    // SAFETY: `set_var` is `unsafe` in Rust 2024 due to process-wide environment races. Irony sets
    // these during GameHandler startup before other threads read Steam env; keep calls in launcher setup.
    unsafe {
        std::env::set_var("SteamAppId", &v);
        std::env::set_var("SteamOverlayGameId", &v);
        std::env::set_var("SteamGameId", &v);
    }
}

/// Writes `steam_appid.txt` in the current directory (Irony only does this on macOS).
pub fn write_steam_app_id_file_macos(app_id: u64) -> std::io::Result<()> {
    if !cfg!(target_os = "macos") {
        return Ok(());
    }
    fs::write(STEAM_APP_ID_FILE, app_id.to_string())
}

/// Deletes `steam_appid.txt` if present (Irony `CleanupAppId`).
pub fn cleanup_steam_app_id_file() {
    let path = Path::new(STEAM_APP_ID_FILE);
    if path.exists() {
        let _ = fs::remove_file(path);
    }
}
