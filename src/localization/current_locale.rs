//! Process-wide current UI locale (Irony `CurrentLocale` subset — no `CultureInfo` / thread culture).

use std::sync::RwLock;

use once_cell::sync::Lazy;

use super::error::LocalizationError;

static CURRENT_LOCALE: Lazy<RwLock<String>> = Lazy::new(|| RwLock::new(String::from("en")));

/// Read-only access to the active culture name (e.g. `en`, `de`, `de-DE`).
pub fn culture_name() -> String {
    CURRENT_LOCALE.read().expect("locale lock poisoned").clone()
}

/// Sets the current culture name used by [`crate::localization::LocalizationManager::get_resource`].
pub fn set_current(culture_name: impl Into<String>) {
    let name = culture_name.into();
    let mut g = CURRENT_LOCALE.write().expect("locale lock poisoned");
    *g = name;
}

/// Like [`set_current`], but rejects empty or whitespace-only tags.
pub fn set_current_checked(culture_name: &str) -> Result<(), LocalizationError> {
    if culture_name.trim().is_empty() {
        return Err(LocalizationError::EmptyLocale);
    }
    set_current(culture_name);
    Ok(())
}
