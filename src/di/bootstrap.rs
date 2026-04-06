//! Explicit startup wiring — Rust alternative to C# `IronyModManager.DI.Bootstrap` + `DIContainer`.
//!
//! The C# stack uses SimpleInjector, static `DIContainer` / `DIResolver`, and assembly scanning.
//! Here we expose a small, testable entry type and document Bevy integration.
//!
//! ## Bevy
//!
//! Prefer inserting shared state as a resource, for example:
//!
//! ```ignore
//! use bevy::prelude::*;
//! use std::sync::Arc;
//! use SquiresWay::di::{DiBootstrap, MessageBusResource};
//!
//! App::new()
//!     .insert_resource(MessageBusResource(Arc::new(DiBootstrap::new().into_message_bus())));
//! ```

use bevy::prelude::Resource;

use super::message_bus::InMemoryMessageBus;

/// One place to construct DI-owned handles for app startup (C# `Bootstrap.Setup` subset).
pub struct DiBootstrap {
    message_bus: InMemoryMessageBus,
}

impl DiBootstrap {
    #[must_use]
    pub fn new() -> Self {
        Self {
            message_bus: InMemoryMessageBus::new(),
        }
    }

    #[must_use]
    pub fn message_bus(&self) -> &InMemoryMessageBus {
        &self.message_bus
    }

    #[must_use]
    pub fn into_message_bus(self) -> InMemoryMessageBus {
        self.message_bus
    }
}

impl Default for DiBootstrap {
    fn default() -> Self {
        Self::new()
    }
}

/// Bevy resource wrapping the in-memory message bus (`Arc` is optional; [`InMemoryMessageBus`] is
/// already cheaply cloneable via internal `Arc`).
#[derive(Resource, Clone)]
pub struct MessageBusResource(pub InMemoryMessageBus);
