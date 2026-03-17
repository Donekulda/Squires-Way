use bitflags::bitflags;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum AchievementStatus {
    #[default]
    NotEvaluated,
    Compatible,
    NotCompatible,
    AttemptedEvaluation,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum DefinitionPriorityType {
    #[default]
    None,
    ModOrder,
    Fios,
    Lios,
    ModOverride,
    NoProvider,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum GameAdvancedFeatures {
    #[default]
    None,
    ReadOnly,
    Full,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum HashReportType {
    Collection,
    Game,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum ModDescriptorType {
    #[default]
    DescriptorMod,
    JsonMetadata,
    JsonMetadataV2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum ModSource {
    #[default]
    Local,
    Steam,
    Paradox,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NotificationPosition {
    BottomRight,
    BottomLeft,
    TopRight,
    TopLeft,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum PatchStateMode {
    #[default]
    None,
    Default,
    Advanced,
    ReadOnly,
    DefaultWithoutLocalization,
    AdvancedWithoutLocalization,
    ReadOnlyWithoutLocalization,
}

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
    pub struct SupportedMergeTypes: u8 {
        const BASIC = 0b0000_0001;
        const ZIP = 0b0000_0010;
    }
}

impl Default for SupportedMergeTypes {
    fn default() -> Self {
        Self::empty()
    }
}

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
    pub struct SupportedOperatingSystems: u8 {
        const WINDOWS = 0b0000_0001;
        const OSX = 0b0000_0010;
        const LINUX = 0b0000_0100;
    }
}

impl Default for SupportedOperatingSystems {
    fn default() -> Self {
        Self::empty()
    }
}
