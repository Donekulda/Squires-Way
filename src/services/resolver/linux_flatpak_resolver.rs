//! Linux Flatpak + Steam path detection (Irony `LinuxFlatPakResolver`).

use std::path::{Path, PathBuf};
use std::sync::OnceLock;

const FLATPAK_BINARY: &str = "/usr/bin/flatpak";
const STEAM_RELATIVE_UNDER_HOME: &str = ".var/app/com.valvesoftware.Steam/.local/share/Steam";

/// Detects Flatpak Steam install layout under the user home (Irony `LinuxFlatPakResolver`).
#[derive(Debug, Default)]
pub struct LinuxFlatPakResolver {
    flatpak_steam_root: OnceLock<PathBuf>,
}

impl LinuxFlatPakResolver {
    pub fn new() -> Self {
        Self::default()
    }

    /// `~/.var/app/com.valvesoftware.Steam/.local/share/Steam` (cached).
    pub fn get_flatpak_steam_path(&self) -> PathBuf {
        self.flatpak_steam_root
            .get_or_init(|| {
                dirs::home_dir()
                    .unwrap_or_default()
                    .join(STEAM_RELATIVE_UNDER_HOME)
            })
            .clone()
    }

    /// `true` if `/usr/bin/flatpak` exists (Irony `HasFlatpak`).
    pub fn has_flatpak(&self) -> bool {
        Path::new(FLATPAK_BINARY).is_file()
    }

    /// `true` if `path` is under [`Self::get_flatpak_steam_path`] (case-insensitive, Irony `IsFlatpakSteamInstall`).
    pub fn is_flatpak_steam_install(&self, path: &str) -> bool {
        let p = path.trim();
        if p.is_empty() {
            return false;
        }
        let base = self.get_flatpak_steam_path();
        if Path::new(p).starts_with(&base) {
            return true;
        }
        let base_s = base.to_string_lossy();
        if p.len() < base_s.len() {
            return false;
        }
        p[..base_s.len()].eq_ignore_ascii_case(base_s.as_ref())
    }

    /// `true` if the Flatpak Steam directory exists (Irony `IsSteamInstalled`).
    pub fn is_steam_installed(&self) -> bool {
        self.get_flatpak_steam_path().is_dir()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn flatpak_steam_path_under_home() {
        let r = LinuxFlatPakResolver::new();
        let home = dirs::home_dir().expect("home");
        let expected = home.join(STEAM_RELATIVE_UNDER_HOME);
        assert_eq!(r.get_flatpak_steam_path(), expected);
    }

    #[test]
    fn is_flatpak_steam_install_prefix() {
        let r = LinuxFlatPakResolver::new();
        let base = r.get_flatpak_steam_path();
        let s = base.to_string_lossy();
        let child = format!("{}/steamapps", s);
        assert!(r.is_flatpak_steam_install(&child));
        assert!(!r.is_flatpak_steam_install("/tmp/other"));
    }

    #[test]
    fn is_flatpak_steam_install_empty_false() {
        let r = LinuxFlatPakResolver::new();
        assert!(!r.is_flatpak_steam_install(""));
        assert!(!r.is_flatpak_steam_install("   "));
    }
}
