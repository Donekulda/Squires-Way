use std::collections::{BTreeSet, HashMap};

pub trait LocalizationRegistry {
    fn get_translation(&self, locale: &str, key: &str) -> String;
    fn get_translation_keys(&self) -> Vec<String>;
    fn get_translations(&self, key: &str) -> Option<HashMap<String, String>>;
    fn register_translation(
        &mut self,
        locale: &str,
        translation_key: &str,
        translation_value: &str,
    );
}

#[derive(Debug, Default, Clone)]
pub struct InMemoryLocalizationRegistry {
    // key -> locale -> value
    values: HashMap<String, HashMap<String, String>>,
}

impl LocalizationRegistry for InMemoryLocalizationRegistry {
    fn get_translation(&self, locale: &str, key: &str) -> String {
        self.values
            .get(key)
            .and_then(|m| m.get(locale))
            .cloned()
            .unwrap_or_default()
    }

    fn get_translation_keys(&self) -> Vec<String> {
        self.values.keys().cloned().collect()
    }

    fn get_translations(&self, key: &str) -> Option<HashMap<String, String>> {
        self.values.get(key).cloned()
    }

    fn register_translation(
        &mut self,
        locale: &str,
        translation_key: &str,
        translation_value: &str,
    ) {
        self.values
            .entry(translation_key.to_owned())
            .or_default()
            .insert(locale.to_owned(), translation_value.to_owned());
    }
}

impl InMemoryLocalizationRegistry {
    pub fn locales(&self) -> Vec<String> {
        let mut locales = BTreeSet::new();
        for by_locale in self.values.values() {
            for locale in by_locale.keys() {
                locales.insert(locale.clone());
            }
        }
        locales.into_iter().collect()
    }
}
