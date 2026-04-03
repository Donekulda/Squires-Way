use crate::core::version::GameVersion;
use crate::parser::search::enums::SourceType;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct BaseFilterResult {
    pub negate: bool,
}

/// Name filter segment (see Irony `NameFilterResult`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NameFilterResult {
    pub negate: bool,
    pub text: String,
}

impl NameFilterResult {
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            negate: false,
            text: text.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CanParseResult {
    pub negate: bool,
    pub mapped_static_field: String,
    pub result: bool,
}

impl CanParseResult {
    pub fn new(result: bool, mapped_static_field: impl Into<String>) -> Self {
        Self {
            negate: false,
            mapped_static_field: mapped_static_field.into(),
            result,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BoolFilterResult {
    pub negate: bool,
    pub result: Option<bool>,
}

impl BoolFilterResult {
    pub fn new(result: Option<bool>) -> Self {
        Self {
            negate: false,
            result,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SourceTypeResult {
    pub negate: bool,
    pub result: SourceType,
}

impl SourceTypeResult {
    pub fn new(result: SourceType) -> Self {
        Self {
            negate: false,
            result,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VersionTypeResult {
    pub negate: bool,
    pub version: Option<GameVersion>,
}

impl VersionTypeResult {
    pub fn new(version: Option<GameVersion>) -> Self {
        Self {
            negate: false,
            version,
        }
    }
}

/// Parsed mod search/filter query (see Irony `ISearchParserResult` / `SearchParserResult`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SearchParserResult {
    pub achievement_compatible: BoolFilterResult,
    pub is_selected: BoolFilterResult,
    pub name: Vec<NameFilterResult>,
    pub source: Vec<SourceTypeResult>,
    pub version: Vec<VersionTypeResult>,
}

impl Default for SearchParserResult {
    fn default() -> Self {
        Self {
            achievement_compatible: BoolFilterResult::new(None),
            is_selected: BoolFilterResult::new(None),
            name: Vec::new(),
            source: Vec::new(),
            version: Vec::new(),
        }
    }
}
