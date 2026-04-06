//! Typed errors for localization helpers.

use thiserror::Error;

/// Invalid locale or localization configuration.
#[derive(Debug, Error)]
pub enum LocalizationError {
    /// Empty or whitespace-only locale tag.
    #[error("locale name must not be empty")]
    EmptyLocale,
}
