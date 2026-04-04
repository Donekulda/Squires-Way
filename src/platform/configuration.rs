//! Cross-platform UI shell options aligned with Irony `PlatformConfigurationOptions` (non-renderer-specific).
//!
//! JSON uses PascalCase field names for interoperability with existing C#-serialized settings where applicable.

use serde::{Deserialize, Serialize};

/// Top-level platform options container (Irony `PlatformConfigurationOptions`).
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(default, rename_all = "PascalCase")]
pub struct PlatformConfigurationOptions {
    pub app: AppOptions,
    pub conflict_solver: ConflictSolverOptions,
    pub fonts: FontOptions,
    pub linux_options: LinuxOptions,
    pub logging: LoggingOptions,
    pub title_bar: TitleBarOptions,
    pub tooltips: TooltipOptions,
    pub updates: UpdateOptions,
}

/// Irony `IPlatformConfiguration` — types that expose platform options snapshot.
pub trait PlatformConfiguration {
    fn options(&self) -> &PlatformConfigurationOptions;
}

impl PlatformConfiguration for PlatformConfigurationOptions {
    fn options(&self) -> &PlatformConfigurationOptions {
        self
    }
}

/// Application-level flags (Irony `App`).
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(default, rename_all = "PascalCase")]
pub struct AppOptions {
    pub single_instance: bool,
}

/// Conflict solver UI preferences (Irony `ConflictSolver`).
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(default, rename_all = "PascalCase")]
pub struct ConflictSolverOptions {
    pub use_sub_menus: bool,
}

/// Font loading policy (Irony `Fonts`). Renderer-specific loading stays in the UI stack; this only stores the flag.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(default, rename_all = "PascalCase")]
pub struct FontOptions {
    pub use_inbuilt_fonts_only: bool,
}

/// Linux-specific windowing / compositor hints (Irony `LinuxOptions`).
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(default, rename_all = "PascalCase")]
pub struct LinuxOptions {
    pub display_server: Option<String>,
    #[serde(rename = "UseDBusMenu")]
    pub use_dbus_menu: Option<bool>,
    pub use_deferred_rendering: Option<bool>,
    #[serde(rename = "UseEGL")]
    pub use_egl: Option<bool>,
    #[serde(rename = "UseGPU")]
    pub use_gpu: Option<bool>,
    pub wayland_app_id: Option<String>,
}

/// Logging toggles carried from Irony for parity; Bevy may ignore `enable_avalonia_logger`.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(default, rename_all = "PascalCase")]
pub struct LoggingOptions {
    pub enable_avalonia_logger: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(default, rename_all = "PascalCase")]
pub struct TitleBarOptions {
    pub native: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(default, rename_all = "PascalCase")]
pub struct TooltipOptions {
    pub disable: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(default, rename_all = "PascalCase")]
pub struct UpdateOptions {
    pub disable: bool,
    pub disable_install_only: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_options_roundtrip_json_pascal_case() {
        let opts = PlatformConfigurationOptions::default();
        let json = serde_json::to_string_pretty(&opts).expect("serialize");
        let back: PlatformConfigurationOptions = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(opts, back);
    }

    #[test]
    fn linux_acronym_fields_roundtrip() {
        let mut opts = PlatformConfigurationOptions::default();
        opts.linux_options.use_egl = Some(true);
        opts.linux_options.use_gpu = Some(false);
        opts.linux_options.use_dbus_menu = Some(true);
        let json = serde_json::to_string(&opts).expect("serialize");
        assert!(json.contains("\"UseEGL\":true"));
        assert!(json.contains("\"UseGPU\":false"));
        assert!(json.contains("\"UseDBusMenu\":true"));
    }
}
