//! Steam launch coordination (Irony `SteamHandler` + `Program.MainAsync` flow).

use std::path::Path;
use std::time::Duration;

use sysinfo::{ProcessRefreshKind, ProcessesToUpdate, RefreshKind, System};

use crate::game::constants::{
    STEAM_ALTERNATE_MAX_ATTEMPTS, STEAM_ALTERNATE_POLL_DELAY_MS, STEAM_DIRECT_MAX_ATTEMPTS,
    STEAM_LAUNCH_MAIN_URI, STEAM_POLL_DELAY_MS,
};
use crate::game::launch;
use crate::game::steam_env::{
    cleanup_steam_app_id_file, set_steam_app_id_env_vars, write_steam_app_id_file_macos,
};

/// Outcome of [`SteamCoordinator::run`] (Irony `Program` exit semantics).
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum GameHandlerOutcome {
    /// No `-i` / `--id` — Irony prints a message and exits without forcing failure.
    MissingSteamAppId,
    Success,
    Failure,
}

/// Pluggable Steam client behavior: process detection + Steamworks API (Irony `SteamHandler` + Steamworks).
///
/// Full Steamworks (`SteamAPI.Init`, Packsize, DllCheck) is **not** linked in this crate yet; the default
/// [`SysSteamRuntime`] returns `false` from [`SteamRuntime::steam_api_init`] until a native integration exists.
pub trait SteamRuntime {
    /// Whether a process named like `steam` / `Steam.exe` is running (Irony `SteamAPI.IsSteamRunning` / process scan).
    fn is_steam_running(&mut self) -> bool;
    /// `SteamAPI.Init()` — must return `true` when the API is ready (stub returns `false` until Steamworks is wired).
    fn steam_api_init(&mut self) -> bool;
    fn steam_api_shutdown(&mut self);
    /// Irony `Packsize.Test()` && `DllCheck.Test()`.
    fn steam_redistributable_checks_ok(&self) -> bool {
        true
    }
}

/// Default runtime: refreshes process list via `sysinfo`; Steam API calls are stubbed.
pub struct SysSteamRuntime {
    system: System,
}

impl SysSteamRuntime {
    #[must_use]
    pub fn new() -> Self {
        let system = System::new_with_specifics(
            RefreshKind::nothing().with_processes(ProcessRefreshKind::everything()),
        );
        Self { system }
    }
}

impl Default for SysSteamRuntime {
    fn default() -> Self {
        Self::new()
    }
}

impl SteamRuntime for SysSteamRuntime {
    fn is_steam_running(&mut self) -> bool {
        self.system.refresh_processes(ProcessesToUpdate::All, true);
        self.system
            .processes()
            .values()
            .any(|p| steam_process_name_matches(&p.name().to_string_lossy()))
    }

    fn steam_api_init(&mut self) -> bool {
        let _ = self;
        false
    }

    fn steam_api_shutdown(&mut self) {
        let _ = self;
    }
}

fn steam_process_name_matches(name: &str) -> bool {
    let name = name.trim();
    let stem = Path::new(name)
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or(name);
    stem.eq_ignore_ascii_case(super::constants::STEAM_PROCESS)
}

/// Orchestrates direct vs alternate Steam launch (Irony `SteamHandler`).
pub struct SteamCoordinator<R: SteamRuntime> {
    runtime: R,
}

impl<R: SteamRuntime> SteamCoordinator<R> {
    #[must_use]
    pub fn new(runtime: R) -> Self {
        Self { runtime }
    }

    /// Direct integration path: set env + optional macOS `steam_appid.txt`, wait for Steam, init API, then shutdown API.
    ///
    /// Parity: `SteamHandler.InitAsync` + `ShutdownAPIAsync`.
    pub fn run_direct_path(&mut self, app_id: u64) -> bool {
        set_steam_app_id_env_vars(app_id);
        if let Err(e) = write_steam_app_id_file_macos(app_id) {
            tracing::debug!(error = %e, "steam_appid.txt");
        }

        if !self.runtime.is_steam_running() {
            if launch::launch_external_command(STEAM_LAUNCH_MAIN_URI).is_err() {
                return false;
            }
            let mut attempts = 0u32;
            while !self.runtime.is_steam_running() {
                if attempts > STEAM_DIRECT_MAX_ATTEMPTS {
                    return false;
                }
                std::thread::sleep(Duration::from_millis(STEAM_POLL_DELAY_MS));
                attempts += 1;
            }
        }

        let mut ok = self.initialize_and_validate();
        let mut init_attempts = 0u32;
        while !ok {
            if init_attempts > STEAM_DIRECT_MAX_ATTEMPTS {
                break;
            }
            ok = self.initialize_and_validate();
            std::thread::sleep(Duration::from_millis(STEAM_POLL_DELAY_MS));
            init_attempts += 1;
        }

        self.shutdown_api();
        ok
    }

    /// Alternate path: open `steam://open/main` if Steam is not running, poll for process (Irony `InitAlternateAsync`).
    pub fn run_alternate_path(&mut self) -> bool {
        if self.runtime.is_steam_running() {
            return true;
        }
        let opened = launch::launch_external_command(STEAM_LAUNCH_MAIN_URI).is_ok();
        let mut attempts = 0u32;
        while !self.runtime.is_steam_running() {
            if attempts > STEAM_ALTERNATE_MAX_ATTEMPTS {
                break;
            }
            std::thread::sleep(Duration::from_millis(STEAM_ALTERNATE_POLL_DELAY_MS));
            attempts += 1;
        }
        opened
    }

    /// Full handler entry (Irony `MainAsync` minus console prints).
    pub fn run(&mut self, args: &super::args::GameHandlerArgs) -> GameHandlerOutcome {
        let Some(app_id) = args.steam_app_id else {
            return GameHandlerOutcome::MissingSteamAppId;
        };
        let ok = if args.use_alternate_launch {
            self.run_alternate_path()
        } else {
            self.run_direct_path(app_id)
        };
        if ok {
            GameHandlerOutcome::Success
        } else {
            GameHandlerOutcome::Failure
        }
    }

    fn initialize_and_validate(&mut self) -> bool {
        let init_ok = self.runtime.steam_api_init();
        if !self.runtime.steam_redistributable_checks_ok() {
            self.runtime.steam_api_shutdown();
            cleanup_steam_app_id_file();
            return false;
        }
        init_ok
    }

    fn shutdown_api(&mut self) {
        self.runtime.steam_api_shutdown();
        cleanup_steam_app_id_file();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game::args::GameHandlerArgs;

    struct FakeSteam {
        steam_running: bool,
        init_calls: u32,
        init_succeed_on_call: Option<u32>,
    }

    impl SteamRuntime for FakeSteam {
        fn is_steam_running(&mut self) -> bool {
            self.steam_running
        }

        fn steam_api_init(&mut self) -> bool {
            self.init_calls += 1;
            if let Some(n) = self.init_succeed_on_call {
                self.init_calls == n
            } else {
                false
            }
        }

        fn steam_api_shutdown(&mut self) {}
    }

    #[test]
    fn alternate_returns_true_when_steam_already_running() {
        let mut c = SteamCoordinator::new(FakeSteam {
            steam_running: true,
            init_calls: 0,
            init_succeed_on_call: None,
        });
        assert!(c.run_alternate_path());
    }

    #[test]
    fn direct_succeeds_when_steam_running_and_init_returns_true_first_call() {
        let mut c = SteamCoordinator::new(FakeSteam {
            steam_running: true,
            init_calls: 0,
            init_succeed_on_call: Some(1),
        });
        assert!(c.run_direct_path(1));
    }

    #[test]
    fn run_missing_app_id() {
        let mut c = SteamCoordinator::new(FakeSteam {
            steam_running: true,
            init_calls: 0,
            init_succeed_on_call: None,
        });
        assert_eq!(
            c.run(&GameHandlerArgs::default()),
            GameHandlerOutcome::MissingSteamAppId
        );
    }
}
