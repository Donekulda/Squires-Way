//! Localization: merged JSON resource bundles and current locale (Irony `IronyModManager.Localization` core).
//!
//! UI/DI attribute plumbing (`StaticLocalizationAttribute`, interceptors, etc.) is not ported; see the C#
//! reference under `Examples/IronyModManager/src/IronyModManager.Localization`.
//!
//! [`crate::parser::search::InMemoryLocalizationRegistry`](crate::parser::search::InMemoryLocalizationRegistry)
//! is a separate in-memory map used by parser search filters; it is not a substitute for this module.

mod current_locale;
mod error;
mod json_merge;
mod json_path;
mod manager;
mod resource_provider;

pub use current_locale::{culture_name, set_current, set_current_checked};
pub use error::LocalizationError;
pub use manager::LocalizationManager;
pub use resource_provider::{FileLocalizationResourceProvider, LocalizationResourceProvider};
