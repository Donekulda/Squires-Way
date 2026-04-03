//! Game install / DLC root discovery (Irony `GameRootPathResolver`).

use std::path::{Path, PathBuf};

/// Minimal fields needed for [`GameRootPathResolver::get_path`] — parity with
/// `IGame.ExecutableLocation` and `IGame.DLCContainer` (`IronyModManager.Models.Common`).
pub trait GameRootPaths {
    fn executable_location(&self) -> &str;
    fn dlc_container(&self) -> &str;
}

/// Plain data for tests and callers that do not yet carry a full [`crate::core`] game model.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GameRootPathsSnapshot {
    pub executable_location: String,
    pub dlc_container: String,
}

impl GameRootPaths for GameRootPathsSnapshot {
    fn executable_location(&self) -> &str {
        &self.executable_location
    }

    fn dlc_container(&self) -> &str {
        &self.dlc_container
    }
}

/// Discover a directory ancestor of the game executable that contains the DLC folder layout
/// used by Irony (`IronyModManager.Services.Resolver.GameRootPathResolver`).
pub struct GameRootPathResolver;

impl GameRootPathResolver {
    pub const DLC_FOLDER: &'static str = "dlc";

    /// Irony `GameRootPathResolver.ResolveDLCDirectory`: if `base_path` is blank, returns `dlc`
    /// as a relative segment; otherwise `base_path` + `dlc`.
    pub fn resolve_dlc_directory(base_path: &str, dlc: &str) -> PathBuf {
        if base_path.trim().is_empty() {
            PathBuf::from(dlc)
        } else {
            Path::new(base_path.trim()).join(dlc)
        }
    }

    /// Walks from the executable's directory upward until `ResolveDLCDirectory(DLCContainer, DLC_FOLDER)` exists.
    ///
    /// Returns [`None`] if the executable path is empty, has no parent, or no matching DLC tree is found.
    pub fn get_path<G: GameRootPaths>(game: &G) -> Option<PathBuf> {
        let exe = game.executable_location().trim();
        if exe.is_empty() {
            return None;
        }
        let mut path = Path::new(exe).parent()?.to_path_buf();
        loop {
            let directory = Self::join_dlc_for_game(game, &path);
            if directory.exists() {
                return Some(path);
            }
            path = path.parent()?.to_path_buf();
        }
    }

    fn join_dlc_for_game<G: GameRootPaths>(game: &G, base: &Path) -> PathBuf {
        let rel = Self::resolve_dlc_directory(game.dlc_container().trim(), Self::DLC_FOLDER);
        base.join(rel)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn resolve_dlc_empty_base_returns_dlc_only() {
        let p = GameRootPathResolver::resolve_dlc_directory("", "dlc");
        assert_eq!(p, PathBuf::from("dlc"));
    }

    #[test]
    fn resolve_dlc_combines_base_and_subdir() {
        let p = GameRootPathResolver::resolve_dlc_directory("foo", "dlc");
        assert_eq!(p, Path::new("foo").join("dlc"));
    }

    #[test]
    fn get_path_finds_dlc_one_level_up() {
        let root = tempfile::tempdir().expect("tempdir");
        let bin = root.path().join("bin");
        fs::create_dir_all(&bin).expect("mkdir bin");
        let dlc = root.path().join("dlc");
        fs::create_dir_all(&dlc).expect("mkdir dlc");

        let exe = bin.join("game.exe");
        let game = GameRootPathsSnapshot {
            executable_location: exe.to_string_lossy().into_owned(),
            dlc_container: String::new(),
        };

        let found = GameRootPathResolver::get_path(&game).expect("expected root");
        assert_eq!(found, root.path());
    }

    #[test]
    fn get_path_finds_nested_dlc_container() {
        let root = tempfile::tempdir().expect("tempdir");
        let bin = root.path().join("bin");
        fs::create_dir_all(&bin).expect("mkdir bin");
        let nested = root.path().join("crusader kings iii").join("dlc");
        fs::create_dir_all(&nested).expect("mkdir nested dlc");

        let exe = bin.join("ck3.exe");
        let game = GameRootPathsSnapshot {
            executable_location: exe.to_string_lossy().into_owned(),
            dlc_container: "crusader kings iii".to_string(),
        };

        let found = GameRootPathResolver::get_path(&game).expect("expected root");
        assert_eq!(found, root.path());
    }

    #[test]
    fn get_path_none_when_no_dlc_tree() {
        let root = tempfile::tempdir().expect("tempdir");
        let bin = root.path().join("bin");
        fs::create_dir_all(&bin).expect("mkdir bin");
        let exe = bin.join("solo.exe");
        let game = GameRootPathsSnapshot {
            executable_location: exe.to_string_lossy().into_owned(),
            dlc_container: String::new(),
        };
        assert!(GameRootPathResolver::get_path(&game).is_none());
    }
}
