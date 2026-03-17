use std::collections::HashMap;
use std::path::Path;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::core::enums::{
    AchievementStatus, GameAdvancedFeatures, ModDescriptorType, ModSource, SupportedMergeTypes,
};
use crate::core::version::{GameVersion, parse_descriptor_version};

pub trait QueryableModel {
    fn is_match(&self, term: &str) -> bool;
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct ModCollectionSourceInfo {
    pub paradox_id: Option<i64>,
    pub steam_id: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct Mod {
    pub achievement_status: AchievementStatus,
    pub additional_data: HashMap<String, Value>,
    pub dependencies: Vec<String>,
    pub descriptor_file: Option<String>,
    pub file_name: Option<String>,
    pub files: Vec<String>,
    pub full_path: Option<String>,
    pub game: Option<String>,
    pub is_locked: bool,
    pub is_selected: bool,
    pub is_valid: bool,
    pub json_id: Option<String>,
    pub name: Option<String>,
    pub order: i32,
    pub picture: Option<String>,
    pub relationship_data: Vec<HashMap<String, Value>>,
    pub remote_id: Option<i64>,
    pub replace_path: Vec<String>,
    pub source: ModSource,
    pub tags: Vec<String>,
    pub user_dir: Vec<String>,
    pub version: Option<String>,
}

impl Mod {
    pub fn parent_directory(&self) -> Option<String> {
        let full_path = self.full_path.as_ref()?;
        if full_path.trim().is_empty() {
            return self.full_path.clone();
        }

        let path = Path::new(full_path);
        if path.extension().is_some() {
            return path.parent().map(|p| p.to_string_lossy().to_string());
        }

        self.full_path.clone()
    }

    pub fn version_data(&self) -> GameVersion {
        self.version
            .as_deref()
            .and_then(parse_descriptor_version)
            .unwrap_or_default()
    }
}

impl QueryableModel for Mod {
    fn is_match(&self, term: &str) -> bool {
        let Some(name) = self.name.as_deref() else {
            return false;
        };
        if name.trim().is_empty() {
            return false;
        }

        name.to_lowercase().starts_with(&term.to_lowercase())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct ModCollection {
    pub game: Option<String>,
    pub is_selected: bool,
    pub merged_folder_name: Option<String>,
    pub mod_ids: Vec<ModCollectionSourceInfo>,
    pub mod_names: Vec<String>,
    pub mod_paths: Vec<String>,
    pub mods: Vec<String>,
    pub name: Option<String>,
    pub patch_mod_enabled: bool,
}

impl QueryableModel for ModCollection {
    fn is_match(&self, term: &str) -> bool {
        let Some(name) = self.name.as_deref() else {
            return false;
        };
        if name.trim().is_empty() {
            return false;
        }

        name.to_lowercase().starts_with(&term.to_lowercase())
    }
}

impl ModCollection {
    pub fn new() -> Self {
        Self {
            patch_mod_enabled: true,
            ..Self::default()
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct Game {
    pub abrv: Option<String>,
    pub advanced_features: GameAdvancedFeatures,
    pub base_steam_game_directory: Option<String>,
    pub checksum_folders: Vec<String>,
    pub close_app_after_game_launch: bool,
    pub custom_mod_directory: Option<String>,
    pub dlc_container: Option<String>,
    pub executable_location: Option<String>,
    pub game_folders: Vec<String>,
    pub game_index_cache_version: i32,
    pub gog_app_id: Option<i32>,
    pub is_selected: bool,
    pub launch_arguments: Option<String>,
    pub launcher_settings_file_name: Option<String>,
    pub launcher_settings_prefix: Option<String>,
    pub linux_proton_version: Option<String>,
    pub log_location: Option<String>,
    pub mod_descriptor_type: ModDescriptorType,
    pub name: Option<String>,
    pub paradox_game_id: Option<String>,
    pub refresh_descriptors: bool,
    pub remote_steam_user_directory: Vec<String>,
    pub steam_app_id: i32,
    pub steam_root: Option<String>,
    pub supported_merge_types: SupportedMergeTypes,
    pub kind: Option<String>,
    pub user_directory: Option<String>,
    pub workshop_directory: Vec<String>,
}

impl QueryableModel for Game {
    fn is_match(&self, term: &str) -> bool {
        let Some(name) = self.name.as_deref() else {
            return false;
        };
        if name.trim().is_empty() {
            return false;
        }

        name.to_lowercase().starts_with(&term.to_lowercase())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mod_parent_directory_returns_folder_for_file_path() {
        let model = Mod {
            full_path: Some("mods/abc/test.mod".to_owned()),
            ..Mod::default()
        };
        assert_eq!(model.parent_directory().as_deref(), Some("mods/abc"));
    }

    #[test]
    fn mod_parent_directory_returns_original_for_folder_path() {
        let model = Mod {
            full_path: Some("mods/abc".to_owned()),
            ..Mod::default()
        };
        assert_eq!(model.parent_directory().as_deref(), Some("mods/abc"));
    }

    #[test]
    fn queryable_models_match_case_insensitively() {
        let model = ModCollection {
            name: Some("Base Collection".to_owned()),
            ..ModCollection::new()
        };
        assert!(model.is_match("base"));
        assert!(!model.is_match("other"));
    }

    #[test]
    fn mod_defaults_version_data_when_invalid() {
        let model = Mod {
            version: Some("invalid".to_owned()),
            ..Mod::default()
        };
        let version = model.version_data();
        assert_eq!(version.major, 0);
        assert_eq!(version.minor, 0);
    }
}
