use once_cell::sync::Lazy;
use regex::Regex;
use serde::{Deserialize, Serialize};

static VERSION_EXTRACT: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"(?:\*|\d+)(?:\.\d+|\.\*)+").expect("valid regex"));

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct GameVersion {
    pub major: i32,
    pub minor: i32,
    pub build: i32,
    pub revision: i32,
    pub explicit_parts: usize,
    pub wildcard_minor: bool,
}

impl GameVersion {
    pub fn parse(raw: &str) -> Option<Self> {
        let trimmed = raw.trim();
        if trimmed.is_empty() {
            return None;
        }

        let mut value = trimmed.to_owned();
        if value.chars().any(char::is_alphabetic) {
            let m = VERSION_EXTRACT.find(&value)?;
            value = m.as_str().to_owned();
        }

        let mut parts = Vec::new();
        let mut wildcard_minor = false;
        for (idx, part) in value.split('.').enumerate() {
            let token = if part.contains('*') {
                if idx <= 1 { 0 } else { i32::MAX }
            } else {
                part.parse::<i32>().ok()?
            };
            if idx == 1 && part.contains('*') {
                wildcard_minor = true;
            }
            parts.push(token);
        }

        if parts.is_empty() {
            return None;
        }

        while parts.len() < 4 {
            parts.push(-1);
        }

        Some(Self {
            major: parts[0],
            minor: parts[1],
            build: parts[2],
            revision: parts[3],
            explicit_parts: value.split('.').count(),
            wildcard_minor,
        })
    }
}

pub fn parse_descriptor_version(raw: &str) -> Option<GameVersion> {
    GameVersion::parse(raw)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_plain_version() {
        let v = GameVersion::parse("1.2.3").expect("version");
        assert_eq!(v.major, 1);
        assert_eq!(v.minor, 2);
        assert_eq!(v.build, 3);
    }

    #[test]
    fn extracts_version_from_dirty_text() {
        let v = GameVersion::parse("v1.11.* beta").expect("version");
        assert_eq!(v.major, 1);
        assert_eq!(v.minor, 11);
        assert_eq!(v.build, i32::MAX);
    }
}
