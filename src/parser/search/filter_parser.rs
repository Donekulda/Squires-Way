//! Mod search string parser (Irony `IronyModManager.Parser.Mod.Search.Parser`).

use std::collections::{HashMap, HashSet};

use crate::parser::search::constants::filter_operators;
use crate::parser::search::converters::{
    BoolConverter, SourceTypeConverter, TypeConverter, VersionConverter,
};
use crate::parser::search::fields;
use crate::parser::search::localization_registry::LocalizationRegistry;
use crate::parser::search::results::{NameFilterResult, SearchParserResult};

const CACHE_PREFIX: &str = "SearchParserEntry-";
const CACHE_REGION: &str = "SearchParserRegion";
const MAX_RECORDS_TO_CACHE: usize = 1000;
const NAME_PROPERTY: &str = "name";

/// Optional result cache (Irony `ICache`); first-hit wins, bounded like the C# implementation.
pub trait SearchParseCache {
    fn get(&self, region: &str, key: &str) -> Option<SearchParserResult>;
    fn set(&mut self, region: &str, key: &str, value: SearchParserResult);
}

/// In-memory cache for tests and simple hosts.
#[derive(Debug, Default, Clone)]
pub struct InMemorySearchParseCache {
    entries: HashMap<String, SearchParserResult>,
}

impl InMemorySearchParseCache {
    fn composite_key(region: &str, key: &str) -> String {
        format!("{region}:{key}")
    }
}

impl SearchParseCache for InMemorySearchParseCache {
    fn get(&self, region: &str, key: &str) -> Option<SearchParserResult> {
        self.entries.get(&Self::composite_key(region, key)).cloned()
    }

    fn set(&mut self, region: &str, key: &str, value: SearchParserResult) {
        if self.entries.len() >= MAX_RECORDS_TO_CACHE {
            self.entries.clear();
        }
        self.entries.insert(Self::composite_key(region, key), value);
    }
}

/// Parses localized mod search lines into structured filter results.
pub struct SearchParser<'a> {
    localization_registry: &'a dyn LocalizationRegistry,
    bool_converter: BoolConverter<'a>,
    source_converter: SourceTypeConverter<'a>,
    version_converter: VersionConverter<'a>,
}

impl<'a> SearchParser<'a> {
    pub fn new(localization_registry: &'a dyn LocalizationRegistry) -> Self {
        Self {
            localization_registry,
            bool_converter: BoolConverter::new(localization_registry),
            source_converter: SourceTypeConverter::new(localization_registry),
            version_converter: VersionConverter::new(localization_registry),
        }
    }

    fn cache_key(text: &str) -> String {
        format!("{CACHE_PREFIX}{}", text)
    }

    /// Parse `text` for `locale`. When `cache` is `Some`, results match Irony's memoization.
    pub fn parse(
        &self,
        locale: &str,
        text: &str,
        mut cache: Option<&mut dyn SearchParseCache>,
    ) -> SearchParserResult {
        let cache_key = Self::cache_key(text);
        if let Some(ref mut c) = cache
            && let Some(hit) = c.get(CACHE_REGION, &cache_key)
        {
            return hit;
        }

        let or_sep = self
            .localization_registry
            .get_translation(locale, filter_operators::OR_STATEMENT_SEPARATOR);
        let value_sep = self
            .localization_registry
            .get_translation(locale, filter_operators::VALUE_SEPARATOR);
        let statement_sep = self
            .localization_registry
            .get_translation(locale, filter_operators::STATEMENT_SEPARATOR);
        let negate_op = self
            .localization_registry
            .get_translation(locale, filter_operators::NEGATE);

        if text.trim().is_empty() {
            let result = SearchParserResult {
                name: parse_or_statements(&text.to_lowercase(), &or_sep, &negate_op),
                ..Default::default()
            };
            if let Some(c) = cache.as_mut() {
                c.set(CACHE_REGION, &cache_key, result.clone());
            }
            return result;
        }

        match self.parse_inner(
            locale,
            text,
            &or_sep,
            &value_sep,
            &statement_sep,
            &negate_op,
        ) {
            Some(result) => {
                if let Some(c) = cache.as_mut() {
                    c.set(CACHE_REGION, &cache_key, result.clone());
                }
                result
            }
            None => {
                let result = SearchParserResult {
                    name: parse_or_statements(&text.to_lowercase(), &or_sep, &negate_op),
                    ..Default::default()
                };
                if let Some(c) = cache.as_mut() {
                    c.set(CACHE_REGION, &cache_key, result.clone());
                }
                result
            }
        }
    }

    fn parse_inner(
        &self,
        locale: &str,
        text: &str,
        or_sep: &str,
        value_sep: &str,
        statement_sep: &str,
        negate_op: &str,
    ) -> Option<SearchParserResult> {
        let lower = text.to_lowercase();
        let segments: Vec<&str> = if statement_sep.is_empty() {
            vec![lower.as_str()]
        } else {
            lower.split(statement_sep).collect()
        };

        let mut token_pairs: Vec<(String, String)> = Vec::new();
        for raw in segments {
            let trimmed = raw.trim();
            if trimmed.is_empty() {
                continue;
            }
            let parts: Vec<&str> = trimmed.split(value_sep).collect();
            let (key, value) = if parts.len() == 2 {
                (parts[0].trim().to_string(), parts[1].trim().to_string())
            } else {
                (NAME_PROPERTY.to_string(), trimmed.to_string())
            };
            token_pairs.push((key, value));
        }

        let mut seen = HashSet::new();
        let mut tokens: Vec<(String, String)> = Vec::new();
        for (k, v) in token_pairs {
            if seen.insert(k.clone()) {
                tokens.push((k, v));
            }
        }

        if tokens.is_empty() {
            return None;
        }

        let mut nothing_set = true;
        let mut result = SearchParserResult::default();

        for (token_key, item_value) in &tokens {
            let mapped = self.resolve_converter_field(locale, token_key);
            if let Some(mapped_field) = mapped {
                if self.apply_mapped_field(
                    locale,
                    &mapped_field,
                    item_value,
                    or_sep,
                    negate_op,
                    &mut result,
                ) {
                    nothing_set = false;
                }
            } else {
                result.name = parse_or_statements(item_value, or_sep, negate_op);
            }
        }

        if nothing_set {
            return None;
        }

        Some(result)
    }

    fn resolve_converter_field(&self, locale: &str, token_key: &str) -> Option<String> {
        let order = [
            self.version_converter.can_convert(locale, token_key),
            self.bool_converter.can_convert(locale, token_key),
            self.source_converter.can_convert(locale, token_key),
        ];
        for c in order {
            if c.result && !c.mapped_static_field.is_empty() {
                return Some(c.mapped_static_field);
            }
        }
        None
    }

    fn apply_mapped_field(
        &self,
        locale: &str,
        mapped_field: &str,
        item_value: &str,
        or_sep: &str,
        negate_op: &str,
        result: &mut SearchParserResult,
    ) -> bool {
        match mapped_field {
            f if f == fields::ACHIEVEMENTS => {
                let negate = trim_negate_prefix(item_value, negate_op);
                let parsed_val = strip_leading_negate(item_value, negate_op);
                let mut v = self.bool_converter.convert(locale, parsed_val);
                v.negate = negate;
                result.achievement_compatible = v;
                true
            }
            f if f == fields::SELECTED => {
                let negate = trim_negate_prefix(item_value, negate_op);
                let parsed_val = strip_leading_negate(item_value, negate_op);
                let mut v = self.bool_converter.convert(locale, parsed_val);
                v.negate = negate;
                result.is_selected = v;
                true
            }
            f if f == fields::SOURCE => {
                let values = parse_or_statements(item_value, or_sep, negate_op);
                result.source.clear();
                for n in values {
                    let mut v = self.source_converter.convert(locale, &n.text);
                    v.negate = n.negate;
                    result.source.push(v);
                }
                true
            }
            f if f == fields::VERSION => {
                let values = parse_or_statements(item_value, or_sep, negate_op);
                result.version.clear();
                for n in values {
                    let mut v = self.version_converter.convert(locale, &n.text);
                    v.negate = n.negate;
                    result.version.push(v);
                }
                true
            }
            _ => false,
        }
    }
}

fn trim_negate_prefix(value: &str, negate_op: &str) -> bool {
    value.trim().starts_with(negate_op)
}

fn strip_leading_negate<'a>(value: &'a str, negate_op: &str) -> &'a str {
    let mut t = value.trim();
    if !negate_op.is_empty() && t.starts_with(negate_op) {
        t = t[negate_op.len()..].trim();
    }
    t
}

fn parse_or_statements(text: &str, or_sep: &str, negate_op: &str) -> Vec<NameFilterResult> {
    let mut list = Vec::new();
    let t = text.trim();
    if t.is_empty() {
        return list;
    }

    if !or_sep.is_empty() && t.contains(or_sep) {
        for x in t.split(or_sep) {
            let x = x.trim();
            if x.is_empty() {
                continue;
            }
            let negate = trim_negate_prefix(x, negate_op);
            let body = strip_leading_negate(x, negate_op);
            list.push(NameFilterResult {
                negate,
                text: body.to_string(),
            });
        }
    } else {
        let negate = trim_negate_prefix(t, negate_op);
        let body = strip_leading_negate(t, negate_op);
        list.push(NameFilterResult {
            negate,
            text: body.to_string(),
        });
    }

    list
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::version::GameVersion;
    use crate::parser::search::constants::filter_commands;
    use crate::parser::search::enums::SourceType;
    use crate::parser::search::localization_registry::InMemoryLocalizationRegistry;

    fn setup_registry() -> InMemoryLocalizationRegistry {
        let mut registry = InMemoryLocalizationRegistry::default();
        registry.register_translation("en", filter_commands::ACHIEVEMENTS, "en-ach");
        registry.register_translation("fr", filter_commands::ACHIEVEMENTS, "fr-ach");
        registry.register_translation("en", filter_commands::SELECTED, "en-sel");
        registry.register_translation("fr", filter_commands::SELECTED, "fr-sel");
        registry.register_translation("en", filter_commands::YES, "yes");
        registry.register_translation("fr", filter_commands::YES, "fr-yes");
        registry.register_translation("en", filter_commands::TRUE, "true");
        registry.register_translation("fr", filter_commands::TRUE, "fr-true");
        registry.register_translation("en", filter_commands::NO, "no");
        registry.register_translation("fr", filter_commands::NO, "fr-no");
        registry.register_translation("en", filter_commands::FALSE, "false");
        registry.register_translation("fr", filter_commands::FALSE, "fr-false");
        registry.register_translation("en", filter_commands::VERSION, "en-ver");
        registry.register_translation("fr", filter_commands::VERSION, "fr-ver");
        registry.register_translation("en", filter_commands::SOURCE, "en-src");
        registry.register_translation("fr", filter_commands::SOURCE, "fr-src");
        registry.register_translation("en", filter_commands::PARADOX, "pdx");
        registry.register_translation("fr", filter_commands::PARADOX, "fr-pdx");
        registry.register_translation("en", filter_commands::STEAM, "steam");
        registry.register_translation("fr", filter_commands::STEAM, "fr-steam");
        registry.register_translation("en", filter_commands::LOCAL, "local");
        registry.register_translation("fr", filter_commands::LOCAL, "fr-local");
        registry.register_translation("en", filter_operators::OR_STATEMENT_SEPARATOR, "||");
        registry.register_translation("en", filter_operators::STATEMENT_SEPARATOR, "&&");
        registry.register_translation("en", filter_operators::VALUE_SEPARATOR, ":");
        registry.register_translation("en", filter_operators::NEGATE, "--");
        registry
    }

    #[test]
    fn should_be_able_to_convert() {
        let registry = setup_registry();
        let parser = SearchParser::new(&registry);
        let line = "test test &&EN-VER:2.0 &&fr-src:steam &&en-ach:yest &&en-sel:true";
        let result = parser.parse("en", line, None);
        assert_eq!(result.achievement_compatible.result, Some(true));
        assert_eq!(result.is_selected.result, Some(true));
        assert_eq!(result.source.len(), 1);
        assert_eq!(result.source[0].result, SourceType::Steam);
        assert_eq!(result.version.len(), 1);
        assert_eq!(result.version[0].version, GameVersion::parse("2.0"));
        assert_eq!(result.name.len(), 1);
        assert!(!result.name[0].negate);
        assert_eq!(result.name[0].text, "test test");
    }

    #[test]
    fn should_be_able_to_recognize_text_search() {
        let registry = setup_registry();
        let parser = SearchParser::new(&registry);
        let result = parser.parse("en", "test test", None);
        assert!(result.achievement_compatible.result.is_none());
        assert!(result.is_selected.result.is_none());
        assert!(result.source.is_empty());
        assert!(result.version.is_empty());
        assert_eq!(result.name.len(), 1);
        assert!(!result.name[0].negate);
        assert_eq!(result.name[0].text, "test test");
    }

    #[test]
    fn should_trim_search_text() {
        let registry = setup_registry();
        let parser = SearchParser::new(&registry);
        let result = parser.parse("en", "test test ", None);
        assert!(result.achievement_compatible.result.is_none());
        assert_eq!(result.name.len(), 1);
        assert_eq!(result.name[0].text, "test test");
    }

    #[test]
    fn should_filter_out_duplicates() {
        let registry = setup_registry();
        let parser = SearchParser::new(&registry);
        let result = parser.parse("en", "en-ach:yes && en-ach:no", None);
        assert_eq!(result.achievement_compatible.result, Some(true));
    }

    #[test]
    fn should_treat_close_matches_as_search_text() {
        let registry = setup_registry();
        let parser = SearchParser::new(&registry);
        let result = parser.parse("en", "en-ach", None);
        assert!(result.achievement_compatible.result.is_none());
        assert_eq!(result.name.len(), 1);
        assert_eq!(result.name[0].text, "en-ach");
    }

    #[test]
    fn should_treat_close_separator_matches_as_search_text() {
        let registry = setup_registry();
        let parser = SearchParser::new(&registry);
        let result = parser.parse("en", "test:test", None);
        assert_eq!(result.name.len(), 1);
        assert_eq!(result.name[0].text, "test:test");

        let result = parser.parse("en", "test:test:test", None);
        assert_eq!(result.name.len(), 1);
        assert_eq!(result.name[0].text, "test:test:test");
    }

    #[test]
    fn should_be_able_to_parse_or_operator() {
        let registry = setup_registry();
        let parser = SearchParser::new(&registry);
        let line = "test test &&EN-VER:2.0||3.0 &&fr-src:steam &&en-ach:yest &&en-sel:true";
        let result = parser.parse("en", line, None);
        assert_eq!(result.achievement_compatible.result, Some(true));
        assert_eq!(result.is_selected.result, Some(true));
        assert_eq!(result.source.len(), 1);
        assert_eq!(result.version.len(), 2);
        assert_eq!(result.version[0].version, GameVersion::parse("2.0"));
        assert_eq!(result.version[1].version, GameVersion::parse("3.0"));
        assert_eq!(result.name.len(), 1);
        assert_eq!(result.name[0].text, "test test");
    }

    #[test]
    fn should_be_able_to_parse_negate_operator() {
        let registry = setup_registry();
        let parser = SearchParser::new(&registry);
        let line = "--test test &&EN-VER:--2.0||3.0 &&fr-src:steam &&en-ach:yest &&en-sel:true";
        let result = parser.parse("en", line, None);
        assert_eq!(result.achievement_compatible.result, Some(true));
        assert_eq!(result.is_selected.result, Some(true));
        assert_eq!(result.source.len(), 1);
        assert_eq!(result.version.len(), 2);
        assert!(result.version[0].negate);
        assert_eq!(result.version[0].version, GameVersion::parse("2.0"));
        assert_eq!(result.version[1].version, GameVersion::parse("3.0"));
        assert!(!result.version[1].negate);
        assert_eq!(result.name.len(), 1);
        assert!(result.name[0].negate);
        assert_eq!(result.name[0].text, "test test");
    }

    #[test]
    fn cache_returns_same_shape() {
        let registry = setup_registry();
        let parser = SearchParser::new(&registry);
        let mut cache = InMemorySearchParseCache::default();
        let line = "hello";
        let a = parser.parse("en", line, Some(&mut cache));
        let b = parser.parse("en", line, Some(&mut cache));
        assert_eq!(a, b);
    }
}
