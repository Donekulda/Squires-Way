//! Typed errors for the DI / message-bus layer.

use thiserror::Error;

/// Errors from dependency-injection helpers (message bus, wiring).
#[derive(Debug, Error)]
pub enum DiError {
    /// Failed to downcast a message to the expected concrete type.
    #[error("message bus downcast failed")]
    Downcast,
}
