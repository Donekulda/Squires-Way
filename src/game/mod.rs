//! Irony **`IronyModManager.GameHandler`** slice: CLI args, Steam timing constants, env + `steam_appid.txt`,
//! and a [`SteamCoordinator`](coordinator::SteamCoordinator) mirroring `SteamHandler` + `Program.MainAsync`.
//!
//! The C# helper also references [`IronyModManager.IO.Platforms.SteamHandler`](https://github.com/bcssov/IronyModManager)
//! (Steamworks). [`SysSteamRuntime`](coordinator::SysSteamRuntime) stubs `steam_api_init` until Steamworks is linked;
//! use [`SteamRuntime`](coordinator::SteamRuntime) with a test double for unit tests.
//!
//! # APTV
//! - **Analyze:** thin `Program` + `SteamHandler` (process checks, env, `steam://` launch, Steamworks init).
//! - **Plan:** pure Rust orchestration + OS launch helpers; trait for Steam API.
//! - **Transform:** this module + [`crate::platform`] `host_os()` at call sites when branching.
//! - **Validate:** unit tests for args + coordinator fakes; `cargo fmt`, `clippy`, `test`.

mod args;
pub mod constants;
mod coordinator;
mod launch;
mod steam_env;

pub use args::{GameHandlerArgs, ParseArgsError, parse_args};
pub use coordinator::{GameHandlerOutcome, SteamCoordinator, SteamRuntime, SysSteamRuntime};

/// Match Irony `Environment.CurrentDirectory = AppDomain.CurrentDomain.BaseDirectory` before reading/writing `steam_appid.txt`.
pub fn set_working_directory_to_executable_dir() -> std::io::Result<()> {
    let exe = std::env::current_exe()?;
    if let Some(dir) = exe.parent() {
        std::env::set_current_dir(dir)?;
    }
    Ok(())
}
