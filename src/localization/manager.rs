//! `LocalizationManager` — merged JSON bundles, cache, and locale fallback (Irony parity).

use std::collections::HashMap;
use std::sync::RwLock;

use serde_json::Value;

use super::current_locale;
use super::json_merge::merge_json;
use super::json_path::select_string_token;
use super::resource_provider::LocalizationResourceProvider;

/// Loads and caches merged JSON localization objects from [`LocalizationResourceProvider`]s.
pub struct LocalizationManager {
    providers: Vec<Box<dyn LocalizationResourceProvider + Send + Sync>>,
    cache: RwLock<HashMap<String, Value>>,
}

impl LocalizationManager {
    pub fn new(providers: Vec<Box<dyn LocalizationResourceProvider + Send + Sync>>) -> Self {
        Self {
            providers,
            cache: RwLock::new(HashMap::new()),
        }
    }

    /// Resolves `key` using [`current_locale::culture_name`] (dot path, e.g. `App.Title`).
    ///
    /// Returns [`None`] when the key is missing or not a non-empty string (Irony returns `null`).
    pub fn get_resource(&self, key: &str) -> Option<String> {
        self.get_resource_locale(&current_locale::culture_name(), key)
    }

    /// Resolves `key` for a specific locale tag (`en`, `de`, `de-DE`, …).
    pub fn get_resource_locale(&self, locale: &str, key: &str) -> Option<String> {
        let mut resource = self.try_get_cached_resource(locale, key);
        if resource.is_none() {
            self.cache_localization(locale);
            resource = self.try_get_cached_resource(locale, key);
        }
        resource
    }

    fn try_get_cached_resource(&self, locale: &str, key: &str) -> Option<String> {
        let mut cached = self.get_cached_resource(locale, key);
        if cached.is_none()
            && let Some(primary) = primary_language_if_regional(locale)
            && primary != locale
        {
            cached = self.get_cached_resource(primary, key);
            if cached.is_none() {
                self.cache_localization(primary);
                cached = self.get_cached_resource(primary, key);
            }
        }
        cached
    }

    fn get_cached_resource(&self, locale: &str, key: &str) -> Option<String> {
        let guard = self.cache.read().ok()?;
        let v = guard.get(locale)?;
        select_string_token(v, key)
    }

    fn cache_localization(&self, locale: &str) {
        let mut merged = Value::Object(serde_json::Map::new());
        for provider in &self.providers {
            let content = provider.read_resource(locale);
            if let Some(text) = trim_non_empty_json(&content)
                && let Ok(v) = serde_json::from_str::<Value>(text)
            {
                merge_json(&mut merged, v);
            }
        }
        if let Ok(mut guard) = self.cache.write() {
            guard.insert(locale.to_owned(), merged);
        }
    }
}

/// When `locale` is a regional tag (`de-DE`), returns the primary language subtag (`de`) if applicable.
fn primary_language_if_regional(locale: &str) -> Option<&str> {
    let (head, tail) = locale.split_once('-')?;
    if head.len() == 2 && !tail.is_empty() {
        Some(head)
    } else {
        None
    }
}

fn trim_non_empty_json(s: &str) -> Option<&str> {
    let t = s.trim();
    if t.is_empty() { None } else { Some(t) }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::path::Path;

    use super::LocalizationManager;
    use crate::localization::resource_provider::LocalizationResourceProvider;

    /// In-memory provider mirroring Irony localization tests (Moq `ReadResource`).
    #[derive(Debug, Default)]
    struct MockProvider {
        resources: HashMap<String, String>,
    }

    impl LocalizationResourceProvider for MockProvider {
        fn root_path(&self) -> &Path {
            Path::new("")
        }

        fn read_resource(&self, locale: &str) -> String {
            self.resources.get(locale).cloned().unwrap_or_default()
        }
    }

    fn test_manager() -> LocalizationManager {
        let mut mock = MockProvider::default();
        mock.resources.insert(
            "en".to_owned(),
            r#"{"App": {"Title": "Irony Mod Manager"}}"#.to_owned(),
        );
        mock.resources.insert(
            "de".to_owned(),
            r#"{"App": {"Title": "Irony Mod Manager DE"}}"#.to_owned(),
        );
        mock.resources.insert("de-DE".to_owned(), String::new());
        LocalizationManager::new(vec![Box::new(mock)])
    }

    #[test]
    fn should_find_translation() {
        let loc = test_manager();
        let result = loc.get_resource_locale("en", "App.Title");
        assert_eq!(result.as_deref(), Some("Irony Mod Manager"));
    }

    #[test]
    fn should_not_find_translation() {
        let loc = test_manager();
        let result = loc.get_resource_locale("en", "App.Title2");
        assert!(result.is_none());
    }

    #[test]
    fn should_find_translation_after_locale_change() {
        let loc = test_manager();
        let result = loc.get_resource_locale("de", "App.Title");
        assert_eq!(result.as_deref(), Some("Irony Mod Manager DE"));
    }

    #[test]
    fn should_not_find_translation_after_locale_change() {
        let loc = test_manager();
        let result = loc.get_resource_locale("de", "App.Title2");
        assert!(result.is_none());
    }

    #[test]
    fn should_fallback_to_two_letter_iso_language() {
        let loc = test_manager();
        let result = loc.get_resource_locale("de-DE", "App.Title");
        assert_eq!(result.as_deref(), Some("Irony Mod Manager DE"));
    }
}
