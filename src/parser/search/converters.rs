use crate::core::version::parse_descriptor_version;
use crate::parser::search::constants::filter_commands;
use crate::parser::search::enums::SourceType;
use crate::parser::search::fields;
use crate::parser::search::localization_registry::LocalizationRegistry;
use crate::parser::search::results::{
    BoolFilterResult, CanParseResult, SourceTypeResult, VersionTypeResult,
};

pub trait TypeConverter<T> {
    fn can_convert(&self, locale: &str, key: &str) -> CanParseResult;
    fn convert(&self, locale: &str, value: &str) -> T;
}

fn get_translation_value(
    localization_registry: &dyn LocalizationRegistry,
    locale: &str,
    value: &str,
    translation_keys: &[(&str, &str)],
) -> (String, String, String) {
    for (translation_key, mapped_static_field) in translation_keys {
        let field = localization_registry.get_translation(locale, translation_key);
        if !field.is_empty() && value.starts_with(&field) {
            return (field, locale.to_owned(), (*mapped_static_field).to_owned());
        }
    }

    for (translation_key, mapped_static_field) in translation_keys {
        if let Some(fields) = localization_registry.get_translations(translation_key) {
            for (field_locale, field_value) in fields {
                if !field_value.is_empty() && value.starts_with(&field_value) {
                    return (field_value, field_locale, (*mapped_static_field).to_owned());
                }
            }
        }
    }

    (String::new(), String::new(), String::new())
}

pub struct BoolConverter<'a> {
    localization_registry: &'a dyn LocalizationRegistry,
}

impl<'a> BoolConverter<'a> {
    pub fn new(localization_registry: &'a dyn LocalizationRegistry) -> Self {
        Self {
            localization_registry,
        }
    }

    fn translation_field_keys() -> [(&'static str, &'static str); 2] {
        [
            (filter_commands::ACHIEVEMENTS, fields::ACHIEVEMENTS),
            (filter_commands::SELECTED, fields::SELECTED),
        ]
    }

    fn value_keys() -> [(&'static str, &'static str); 4] {
        [
            (filter_commands::YES, ""),
            (filter_commands::FALSE, ""),
            (filter_commands::NO, ""),
            (filter_commands::TRUE, ""),
        ]
    }

    fn get_include(&self, locale: &str) -> [String; 2] {
        [
            self.localization_registry
                .get_translation(locale, filter_commands::YES),
            self.localization_registry
                .get_translation(locale, filter_commands::TRUE),
        ]
    }

    fn get_exclude(&self, locale: &str) -> [String; 2] {
        [
            self.localization_registry
                .get_translation(locale, filter_commands::NO),
            self.localization_registry
                .get_translation(locale, filter_commands::FALSE),
        ]
    }
}

impl TypeConverter<BoolFilterResult> for BoolConverter<'_> {
    fn can_convert(&self, locale: &str, key: &str) -> CanParseResult {
        let (translation, _, mapped_static_field) = get_translation_value(
            self.localization_registry,
            locale,
            key,
            &Self::translation_field_keys(),
        );
        CanParseResult::new(!translation.is_empty(), mapped_static_field)
    }

    fn convert(&self, locale: &str, value: &str) -> BoolFilterResult {
        let (translation, locale_used, _) = get_translation_value(
            self.localization_registry,
            locale,
            value,
            &Self::value_keys(),
        );
        if !translation.is_empty() && !locale_used.is_empty() {
            if self
                .get_include(&locale_used)
                .iter()
                .any(|x| !x.is_empty() && translation.starts_with(x))
            {
                return BoolFilterResult::new(Some(true));
            }
            if self
                .get_exclude(&locale_used)
                .iter()
                .any(|x| !x.is_empty() && translation.starts_with(x))
            {
                return BoolFilterResult::new(Some(false));
            }
        }
        BoolFilterResult::new(None)
    }
}

pub struct SourceTypeConverter<'a> {
    localization_registry: &'a dyn LocalizationRegistry,
}

impl<'a> SourceTypeConverter<'a> {
    pub fn new(localization_registry: &'a dyn LocalizationRegistry) -> Self {
        Self {
            localization_registry,
        }
    }

    fn translation_field_keys() -> [(&'static str, &'static str); 1] {
        [(filter_commands::SOURCE, fields::SOURCE)]
    }

    fn value_keys() -> [(&'static str, &'static str); 3] {
        [
            (filter_commands::PARADOX, ""),
            (filter_commands::LOCAL, ""),
            (filter_commands::STEAM, ""),
        ]
    }

    fn get_local(&self, locale: &str) -> String {
        self.localization_registry
            .get_translation(locale, filter_commands::LOCAL)
    }

    fn get_paradox(&self, locale: &str) -> String {
        self.localization_registry
            .get_translation(locale, filter_commands::PARADOX)
    }

    fn get_steam(&self, locale: &str) -> String {
        self.localization_registry
            .get_translation(locale, filter_commands::STEAM)
    }
}

impl TypeConverter<SourceTypeResult> for SourceTypeConverter<'_> {
    fn can_convert(&self, locale: &str, key: &str) -> CanParseResult {
        let (translation, _, mapped_static_field) = get_translation_value(
            self.localization_registry,
            locale,
            key,
            &Self::translation_field_keys(),
        );
        CanParseResult::new(!translation.is_empty(), mapped_static_field)
    }

    fn convert(&self, locale: &str, value: &str) -> SourceTypeResult {
        let (translation, locale_used, _) = get_translation_value(
            self.localization_registry,
            locale,
            value,
            &Self::value_keys(),
        );
        if !translation.is_empty() && !locale_used.is_empty() {
            if self.get_steam(&locale_used).starts_with(&translation) {
                return SourceTypeResult::new(SourceType::Steam);
            }
            if self.get_paradox(&locale_used).starts_with(&translation) {
                return SourceTypeResult::new(SourceType::Paradox);
            }
            if self.get_local(&locale_used).starts_with(&translation) {
                return SourceTypeResult::new(SourceType::Local);
            }
        }
        SourceTypeResult::new(SourceType::None)
    }
}

pub struct VersionConverter<'a> {
    localization_registry: &'a dyn LocalizationRegistry,
}

impl<'a> VersionConverter<'a> {
    pub fn new(localization_registry: &'a dyn LocalizationRegistry) -> Self {
        Self {
            localization_registry,
        }
    }

    fn translation_field_keys() -> [(&'static str, &'static str); 1] {
        [(filter_commands::VERSION, fields::VERSION)]
    }
}

impl TypeConverter<VersionTypeResult> for VersionConverter<'_> {
    fn can_convert(&self, locale: &str, key: &str) -> CanParseResult {
        let (translation, _, mapped_static_field) = get_translation_value(
            self.localization_registry,
            locale,
            key,
            &Self::translation_field_keys(),
        );
        CanParseResult::new(!translation.is_empty(), mapped_static_field)
    }

    fn convert(&self, _locale: &str, value: &str) -> VersionTypeResult {
        VersionTypeResult::new(parse_descriptor_version(value))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
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
        registry.register_translation("en", filter_commands::SOURCE, "en-source");
        registry.register_translation("fr", filter_commands::SOURCE, "fr-source");
        registry.register_translation("en", filter_commands::PARADOX, "pdx");
        registry.register_translation("fr", filter_commands::PARADOX, "fr-pdx");
        registry.register_translation("en", filter_commands::STEAM, "steam");
        registry.register_translation("fr", filter_commands::STEAM, "fr-steam");
        registry.register_translation("en", filter_commands::LOCAL, "local");
        registry.register_translation("fr", filter_commands::LOCAL, "fr-local");
        registry.register_translation("en", filter_commands::VERSION, "en-ver");
        registry.register_translation("fr", filter_commands::VERSION, "fr-ver");
        registry
    }

    #[test]
    fn bool_converter_can_convert_across_locales() {
        let registry = setup_registry();
        let converter = BoolConverter::new(&registry);
        let local = converter.can_convert("en", "en-ach");
        assert!(local.result);
        assert_eq!(local.mapped_static_field, fields::ACHIEVEMENTS);

        let foreign = converter.can_convert("en", "fr-ach");
        assert!(foreign.result);
    }

    #[test]
    fn bool_converter_convert_handles_true_false_none() {
        let registry = setup_registry();
        let converter = BoolConverter::new(&registry);
        assert_eq!(converter.convert("en", "yes").result, Some(true));
        assert_eq!(converter.convert("en", "fr-no").result, Some(false));
        assert_eq!(converter.convert("en", "empty").result, None);
    }

    #[test]
    fn source_type_converter_maps_expected_values() {
        let registry = setup_registry();
        let converter = SourceTypeConverter::new(&registry);
        assert_eq!(converter.convert("en", "steam").result, SourceType::Steam);
        assert_eq!(
            converter.convert("en", "fr-pdx").result,
            SourceType::Paradox
        );
        assert_eq!(converter.convert("en", "local").result, SourceType::Local);
        assert_eq!(converter.convert("en", "dummy").result, SourceType::None);
    }

    #[test]
    fn version_converter_parses_version_strings() {
        let registry = setup_registry();
        let converter = VersionConverter::new(&registry);
        let result = converter.convert("en", "1.0");
        assert_eq!(result.version.expect("version").major, 1);
    }

    #[test]
    fn version_converter_returns_none_for_invalid_values() {
        let registry = setup_registry();
        let converter = VersionConverter::new(&registry);
        assert!(converter.convert("en", "test").version.is_none());
    }

    #[test]
    fn version_converter_can_convert_across_locales() {
        let registry = setup_registry();
        let converter = VersionConverter::new(&registry);
        assert!(converter.can_convert("en", "en-ver").result);
        assert!(converter.can_convert("en", "fr-ver").result);
        assert_eq!(
            converter.can_convert("en", "en-ver").mapped_static_field,
            fields::VERSION
        );
    }

    #[test]
    fn source_converter_can_convert() {
        let registry = setup_registry();
        let converter = SourceTypeConverter::new(&registry);
        assert!(converter.can_convert("en", "en-source").result);
        assert!(converter.can_convert("en", "fr-source").result);
        assert_eq!(
            converter.can_convert("en", "en-source").mapped_static_field,
            fields::SOURCE
        );
    }

    #[test]
    fn source_converter_not_convert_unknown() {
        let registry = setup_registry();
        let converter = SourceTypeConverter::new(&registry);
        assert!(!converter.can_convert("en", "en-fake").result);
    }

    #[test]
    fn bool_converter_not_convert_unknown() {
        let registry = setup_registry();
        let converter = BoolConverter::new(&registry);
        assert!(!converter.can_convert("en", "en-fake").result);
    }

    #[test]
    fn translation_value_prefers_local_match_before_foreign_match() {
        let mut registry = InMemoryLocalizationRegistry::default();
        registry.register_translation("en", filter_commands::SOURCE, "source");
        registry.register_translation("fr", filter_commands::SOURCE, "source-fr");
        let result_local = get_translation_value(
            &registry,
            "en",
            "source",
            &[(filter_commands::SOURCE, fields::SOURCE)],
        );
        assert_eq!(result_local.1, "en");

        let result_foreign = get_translation_value(
            &registry,
            "en",
            "source-fr",
            &[(filter_commands::SOURCE, fields::SOURCE)],
        );
        // Local pass runs first and prefix-matches "source-fr" against "source".
        assert_eq!(result_foreign.1, "en");
    }

    #[test]
    fn get_translation_value_returns_empty_when_missing() {
        let registry = InMemoryLocalizationRegistry::default();
        let result = get_translation_value(
            &registry,
            "en",
            "missing",
            &[(filter_commands::SOURCE, fields::SOURCE)],
        );
        assert!(result.0.is_empty());
        assert!(result.1.is_empty());
        assert!(result.2.is_empty());
    }

    #[test]
    fn registry_locales_collects_all_unique() {
        let mut registry = InMemoryLocalizationRegistry::default();
        registry.register_translation("en", "k1", "v1");
        registry.register_translation("fr", "k1", "v2");
        registry.register_translation("en", "k2", "v3");
        assert_eq!(registry.locales(), vec!["en".to_owned(), "fr".to_owned()]);
    }

    #[test]
    fn registry_get_translation_keys_contains_registered() {
        let mut registry = InMemoryLocalizationRegistry::default();
        registry.register_translation("en", "key-a", "value-a");
        registry.register_translation("en", "key-b", "value-b");
        let mut keys = registry.get_translation_keys();
        keys.sort();
        assert_eq!(keys, vec!["key-a".to_owned(), "key-b".to_owned()]);
    }

    #[test]
    fn translation_lookup_returns_default_for_missing() {
        let registry = InMemoryLocalizationRegistry::default();
        assert_eq!(registry.get_translation("en", "missing"), "");
    }

    #[test]
    fn translations_lookup_returns_none_for_missing_key() {
        let registry = InMemoryLocalizationRegistry::default();
        assert!(registry.get_translations("missing").is_none());
    }

    #[test]
    fn translations_lookup_returns_all_locales_for_key() {
        let mut registry = InMemoryLocalizationRegistry::default();
        registry.register_translation("en", "key", "v-en");
        registry.register_translation("fr", "key", "v-fr");
        let mut values = registry.get_translations("key").expect("values");
        assert_eq!(values.remove("en").as_deref(), Some("v-en"));
        assert_eq!(values.remove("fr").as_deref(), Some("v-fr"));
    }

    #[test]
    fn parse_result_defaults_and_fields_are_constructible() {
        let result = CanParseResult::new(true, "field");
        assert!(result.result);
        assert_eq!(result.mapped_static_field, "field");
    }

    #[test]
    fn base_filter_result_default_negate_is_false() {
        let base = crate::parser::search::results::BaseFilterResult::default();
        assert!(!base.negate);
    }

    #[test]
    fn bool_result_wrapper_stores_value() {
        let value = BoolFilterResult::new(Some(true));
        assert_eq!(value.result, Some(true));
    }

    #[test]
    fn source_result_wrapper_stores_value() {
        let value = SourceTypeResult::new(SourceType::Steam);
        assert_eq!(value.result, SourceType::Steam);
    }

    #[test]
    fn version_result_wrapper_stores_value() {
        let value = VersionTypeResult::new(parse_descriptor_version("1.0"));
        assert_eq!(value.version.expect("version").major, 1);
    }

    #[test]
    fn translation_value_uses_mapped_field() {
        let mut registry = InMemoryLocalizationRegistry::default();
        registry.register_translation("en", filter_commands::VERSION, "version");
        let result = get_translation_value(
            &registry,
            "en",
            "version",
            &[(filter_commands::VERSION, fields::VERSION)],
        );
        assert_eq!(result.2, fields::VERSION);
    }

    #[test]
    fn translation_keys_can_be_mapped_multiple_entries() {
        let mut registry = InMemoryLocalizationRegistry::default();
        registry.register_translation("en", filter_commands::ACHIEVEMENTS, "ach");
        registry.register_translation("en", filter_commands::SELECTED, "sel");
        let result_ach = get_translation_value(
            &registry,
            "en",
            "ach",
            &[
                (filter_commands::ACHIEVEMENTS, fields::ACHIEVEMENTS),
                (filter_commands::SELECTED, fields::SELECTED),
            ],
        );
        let result_sel = get_translation_value(
            &registry,
            "en",
            "sel",
            &[
                (filter_commands::ACHIEVEMENTS, fields::ACHIEVEMENTS),
                (filter_commands::SELECTED, fields::SELECTED),
            ],
        );
        assert_eq!(result_ach.2, fields::ACHIEVEMENTS);
        assert_eq!(result_sel.2, fields::SELECTED);
    }

    #[test]
    fn source_type_default_is_none() {
        assert_eq!(SourceType::default(), SourceType::None);
    }

    #[test]
    fn converter_handles_prefixed_values() {
        let registry = setup_registry();
        let converter = SourceTypeConverter::new(&registry);
        assert_eq!(
            converter.convert("en", "steam workshop").result,
            SourceType::Steam
        );
    }

    #[test]
    fn bool_converter_handles_prefixed_values() {
        let registry = setup_registry();
        let converter = BoolConverter::new(&registry);
        assert_eq!(converter.convert("en", "yes please").result, Some(true));
    }

    #[test]
    fn version_converter_handles_empty_input() {
        let registry = setup_registry();
        let converter = VersionConverter::new(&registry);
        assert!(converter.convert("en", "").version.is_none());
    }

    #[test]
    fn registry_overwrites_existing_translation() {
        let mut registry = InMemoryLocalizationRegistry::default();
        registry.register_translation("en", "key", "a");
        registry.register_translation("en", "key", "b");
        assert_eq!(registry.get_translation("en", "key"), "b");
    }

    #[test]
    fn can_convert_empty_when_no_translation() {
        let registry = InMemoryLocalizationRegistry::default();
        let converter = VersionConverter::new(&registry);
        assert!(!converter.can_convert("en", "version").result);
    }

    #[test]
    fn bool_converter_with_foreign_locale_value() {
        let registry = setup_registry();
        let converter = BoolConverter::new(&registry);
        assert_eq!(converter.convert("en", "fr-true").result, Some(true));
    }

    #[test]
    fn source_converter_with_foreign_locale_value() {
        let registry = setup_registry();
        let converter = SourceTypeConverter::new(&registry);
        assert_eq!(
            converter.convert("en", "fr-local").result,
            SourceType::Local
        );
    }

    #[test]
    fn source_converter_prefers_steam_before_paradox() {
        let mut registry = InMemoryLocalizationRegistry::default();
        registry.register_translation("en", filter_commands::STEAM, "s");
        registry.register_translation("en", filter_commands::PARADOX, "s");
        registry.register_translation("en", filter_commands::LOCAL, "l");
        let converter = SourceTypeConverter::new(&registry);
        assert_eq!(converter.convert("en", "s").result, SourceType::Steam);
    }

    #[test]
    fn bool_converter_prefers_include_before_exclude_on_same_prefix() {
        let mut registry = InMemoryLocalizationRegistry::default();
        registry.register_translation("en", filter_commands::YES, "x");
        registry.register_translation("en", filter_commands::TRUE, "x");
        registry.register_translation("en", filter_commands::NO, "x");
        registry.register_translation("en", filter_commands::FALSE, "x");
        let converter = BoolConverter::new(&registry);
        assert_eq!(converter.convert("en", "x").result, Some(true));
    }

    #[test]
    fn version_converter_accepts_wildcard_patch() {
        let registry = setup_registry();
        let converter = VersionConverter::new(&registry);
        let parsed = converter.convert("en", "1.10.*").version.expect("version");
        assert_eq!(parsed.major, 1);
        assert_eq!(parsed.minor, 10);
    }

    #[test]
    fn localization_keys_can_be_read_even_when_empty() {
        let registry = InMemoryLocalizationRegistry::default();
        assert!(registry.get_translation_keys().is_empty());
    }

    #[test]
    fn source_converter_can_convert_only_source_fields() {
        let registry = setup_registry();
        let converter = SourceTypeConverter::new(&registry);
        assert!(!converter.can_convert("en", "en-ach").result);
        assert!(converter.can_convert("en", "en-source").result);
    }

    #[test]
    fn bool_converter_can_convert_selected_field() {
        let registry = setup_registry();
        let converter = BoolConverter::new(&registry);
        let result = converter.can_convert("en", "en-sel");
        assert!(result.result);
        assert_eq!(result.mapped_static_field, fields::SELECTED);
    }

    #[test]
    fn bool_converter_uses_no_for_false() {
        let registry = setup_registry();
        let converter = BoolConverter::new(&registry);
        assert_eq!(converter.convert("en", "nope").result, Some(false));
    }

    #[test]
    fn source_converter_uses_none_for_unmatched_foreign() {
        let registry = setup_registry();
        let converter = SourceTypeConverter::new(&registry);
        assert_eq!(converter.convert("en", "fr-dummy").result, SourceType::None);
    }

    #[test]
    fn version_converter_can_convert_negative_case() {
        let registry = setup_registry();
        let converter = VersionConverter::new(&registry);
        assert!(!converter.can_convert("en", "en-fake").result);
    }

    #[test]
    fn can_convert_result_negate_default_false() {
        let result = CanParseResult::new(true, "x");
        assert!(!result.negate);
    }

    #[test]
    fn source_result_negate_default_false() {
        let result = SourceTypeResult::new(SourceType::None);
        assert!(!result.negate);
    }

    #[test]
    fn version_result_negate_default_false() {
        let result = VersionTypeResult::new(None);
        assert!(!result.negate);
    }

    #[test]
    fn bool_result_negate_default_false() {
        let result = BoolFilterResult::new(None);
        assert!(!result.negate);
    }

    #[test]
    fn get_translation_value_handles_empty_candidate() {
        let mut registry = InMemoryLocalizationRegistry::default();
        registry.register_translation("en", filter_commands::SOURCE, "");
        let result = get_translation_value(
            &registry,
            "en",
            "any",
            &[(filter_commands::SOURCE, fields::SOURCE)],
        );
        assert!(result.0.is_empty());
    }

    #[test]
    fn get_translation_value_uses_prefix_matching() {
        let mut registry = InMemoryLocalizationRegistry::default();
        registry.register_translation("en", filter_commands::SOURCE, "so");
        let result = get_translation_value(
            &registry,
            "en",
            "source",
            &[(filter_commands::SOURCE, fields::SOURCE)],
        );
        assert_eq!(result.0, "so");
    }

    #[test]
    fn source_converter_localized_precedence() {
        let mut registry = InMemoryLocalizationRegistry::default();
        registry.register_translation("en", filter_commands::STEAM, "steam");
        registry.register_translation("fr", filter_commands::STEAM, "steam-fr");
        registry.register_translation("en", filter_commands::PARADOX, "pdx");
        registry.register_translation("en", filter_commands::LOCAL, "local");
        let converter = SourceTypeConverter::new(&registry);
        assert_eq!(
            converter.convert("en", "steam-fr").result,
            SourceType::Steam
        );
    }

    #[test]
    fn bool_converter_empty_registry_returns_none() {
        let registry = InMemoryLocalizationRegistry::default();
        let converter = BoolConverter::new(&registry);
        assert_eq!(converter.convert("en", "yes").result, None);
    }

    #[test]
    fn source_converter_empty_registry_returns_none_type() {
        let registry = InMemoryLocalizationRegistry::default();
        let converter = SourceTypeConverter::new(&registry);
        assert_eq!(converter.convert("en", "steam").result, SourceType::None);
    }

    #[test]
    fn version_converter_with_whitespace_returns_none() {
        let registry = InMemoryLocalizationRegistry::default();
        let converter = VersionConverter::new(&registry);
        assert!(converter.convert("en", "   ").version.is_none());
    }

    #[test]
    fn mapped_field_empty_when_not_found() {
        let registry = InMemoryLocalizationRegistry::default();
        let converter = BoolConverter::new(&registry);
        let result = converter.can_convert("en", "x");
        assert!(result.mapped_static_field.is_empty());
    }

    #[test]
    fn bool_converter_uses_translated_false_literal() {
        let registry = setup_registry();
        let converter = BoolConverter::new(&registry);
        assert_eq!(converter.convert("en", "fr-false").result, Some(false));
    }

    #[test]
    fn source_converter_matches_prefix_not_exact_only() {
        let registry = setup_registry();
        let converter = SourceTypeConverter::new(&registry);
        assert_eq!(
            converter.convert("en", "steam-123").result,
            SourceType::Steam
        );
    }
}
