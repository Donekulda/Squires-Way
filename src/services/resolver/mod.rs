//! Path resolution helpers (Irony `IronyModManager.Services.Resolver`).

mod game_root_path;
mod linux_flatpak_resolver;
mod path_resolver;

pub use game_root_path::{GameRootPathResolver, GameRootPaths, GameRootPathsSnapshot};
pub use linux_flatpak_resolver::LinuxFlatPakResolver;
pub use path_resolver::{PathResolver, parse_path};
