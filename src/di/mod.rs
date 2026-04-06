//! Dependency injection and cross-cutting wiring — port of `IronyModManager.DI` (first slice).
//!
//! ## Feature map (C# → Rust, this slice)
//!
//! | C# | Rust |
//! |----|------|
//! | `IMessageBus` / `MessageBus` | [`message_bus::InMemoryMessageBus`] |
//! | `IMessageBusEvent` | [`message_bus::MessageBusEvent`] (marker) |
//! | `Bootstrap.Setup` / static container | [`bootstrap::DiBootstrap`] + explicit construction |
//! | `DIResolver.Get<T>()` | Pass [`InMemoryMessageBus`] / resources by value, not service location |
//! | Assembly loading, JSON DI, interceptors | **Deferred** — no reflection in Rust |
//!
//! ## Error model
//!
//! Library-style errors live in [`DiError`]. The current message-bus path uses `expect` only for
//! invariant violations (wrong `Any` downcast); public APIs return `()` or async `()`.

pub mod bootstrap;
pub mod error;
pub mod message_bus;

pub use bootstrap::{DiBootstrap, MessageBusResource};
pub use error::DiError;
pub use message_bus::{InMemoryMessageBus, MessageBusEvent, SubscriptionGuard};
