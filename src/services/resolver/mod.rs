//! Path resolution helpers (Irony `IronyModManager.Services.Resolver`).

mod game_root_path;

pub use game_root_path::{GameRootPathResolver, GameRootPaths, GameRootPathsSnapshot};
