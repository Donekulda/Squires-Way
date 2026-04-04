//! Clipboard abstraction for UI integration (Irony `IIronyClipboard` surface, without Avalonia types).

use std::borrow::Cow;

use thiserror::Error;

/// Failures when reading or writing the system clipboard.
#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum ClipboardError {
    #[error("clipboard is unavailable on this platform or in this build")]
    Unavailable,
    #[error("clipboard text is not valid UTF-8")]
    InvalidUtf8,
}

/// When set, clipboard get/set should normalize to a single line (Irony `PreventMultiLineText`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct ClipboardPolicy {
    pub prevent_multiline: bool,
}

impl ClipboardPolicy {
    /// Collapses newlines for single-line clipboard semantics when `prevent_multiline` is true.
    #[must_use]
    pub fn normalize_text<'a>(&self, text: &'a str) -> Cow<'a, str> {
        if !self.prevent_multiline || !text.as_bytes().iter().any(|&b| b == b'\r' || b == b'\n') {
            return Cow::Borrowed(text);
        }
        let single = text
            .chars()
            .map(|c| if c == '\r' || c == '\n' { ' ' } else { c })
            .collect::<String>();
        Cow::Owned(single)
    }
}

/// Minimal clipboard port surface (no async; UI crate can block briefly or use `spawn_local`).
pub trait Clipboard: Send + Sync {
    fn get_text(&self) -> Result<String, ClipboardError>;
    fn set_text(&self, text: &str) -> Result<(), ClipboardError>;
}

/// Placeholder used until a Bevy / `arboard` (or platform) implementation is wired.
#[derive(Debug, Default, Clone, Copy)]
pub struct NoopClipboard;

impl Clipboard for NoopClipboard {
    fn get_text(&self) -> Result<String, ClipboardError> {
        Err(ClipboardError::Unavailable)
    }

    fn set_text(&self, _text: &str) -> Result<(), ClipboardError> {
        Err(ClipboardError::Unavailable)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn policy_replaces_newlines_when_enabled() {
        let p = ClipboardPolicy {
            prevent_multiline: true,
        };
        assert_eq!(p.normalize_text("a\nb").as_ref(), "a b");
        assert_eq!(p.normalize_text("a\rb\nc").as_ref(), "a b c");
    }

    #[test]
    fn policy_passes_through_when_disabled() {
        let p = ClipboardPolicy::default();
        let s = "line1\nline2";
        match p.normalize_text(s) {
            Cow::Borrowed(b) => assert_eq!(b, s),
            Cow::Owned(_) => panic!("expected borrow"),
        }
    }
}
