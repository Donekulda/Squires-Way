//! In-process message bus — Rust counterpart to `IronyModManager.Shared.MessageBus.IMessageBus`
//! and the internal `IronyModManager.DI.MessageBus.MessageBus` wrapper around SlimMessageBus.
//!
//! C# maps to Rust (first slice):
//! - `IMessageBusEvent` → [`MessageBusEvent`]
//! - `IMessageBus.Publish` / `PublishAsync` → [`InMemoryMessageBus::publish`] / [`InMemoryMessageBus::publish_async`]
//! - Types are “registered” when at least one [`InMemoryMessageBus::subscribe`] exists (mirrors consumer discovery).
//! - Publishing an unregistered type is a no-op (same as C# when `registeredTypes` lacks `T`).

use std::any::{Any, TypeId};
use std::collections::{HashMap, HashSet};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, RwLock, Weak};

/// Marker for events carried on the bus (C# `IMessageBusEvent`).
pub trait MessageBusEvent: Send + Sync + 'static {}

impl<T: Send + Sync + 'static> MessageBusEvent for T {}

struct HandlerSlot {
    id: u64,
    invoke: Arc<dyn Fn(Box<dyn Any + Send>) + Send + Sync>,
}

struct Inner {
    handlers: RwLock<HashMap<TypeId, Vec<HandlerSlot>>>,
    registered: RwLock<HashSet<TypeId>>,
    next_id: AtomicU64,
}

/// Subscription handle; dropping unsubscribes (C# `IDisposable` from `Subscribe`).
pub struct SubscriptionGuard {
    id: u64,
    tid: TypeId,
    inner: Weak<Inner>,
}

impl Drop for SubscriptionGuard {
    fn drop(&mut self) {
        let Some(inner) = self.inner.upgrade() else {
            return;
        };
        let mut map = inner.handlers.write().expect("message bus lock poisoned");
        if let Some(v) = map.get_mut(&self.tid) {
            v.retain(|s| s.id != self.id);
            if v.is_empty() {
                map.remove(&self.tid);
                inner
                    .registered
                    .write()
                    .expect("message bus lock poisoned")
                    .remove(&self.tid);
            }
        }
    }
}

/// In-memory pub/sub bus with type-safe subscribe and type-erased dispatch.
#[derive(Clone)]
pub struct InMemoryMessageBus {
    inner: Arc<Inner>,
}

impl InMemoryMessageBus {
    /// Creates an empty bus (no registered message types).
    pub fn new() -> Self {
        Self {
            inner: Arc::new(Inner {
                handlers: RwLock::new(HashMap::new()),
                registered: RwLock::new(HashSet::new()),
                next_id: AtomicU64::new(1),
            }),
        }
    }

    /// Registers a synchronous handler for `T`. The message type becomes publishable until all
    /// subscriptions for `T` are dropped.
    pub fn subscribe<T, F>(&self, handler: F) -> SubscriptionGuard
    where
        T: MessageBusEvent + Clone + Send + 'static,
        F: Fn(T) + Send + Sync + 'static,
    {
        let id = self.inner.next_id.fetch_add(1, Ordering::Relaxed);
        let invoke: Arc<dyn Fn(Box<dyn Any + Send>) + Send + Sync> = Arc::new(move |any| {
            let msg = *any
                .downcast::<T>()
                .expect("message bus: handler type must match subscription type");
            handler(msg);
        });
        let tid = TypeId::of::<T>();
        {
            let mut map = self
                .inner
                .handlers
                .write()
                .expect("message bus lock poisoned");
            map.entry(tid).or_default().push(HandlerSlot { id, invoke });
            self.inner
                .registered
                .write()
                .expect("message bus lock poisoned")
                .insert(tid);
        }
        SubscriptionGuard {
            id,
            tid,
            inner: Arc::downgrade(&self.inner),
        }
    }

    /// Fire-and-forget publish: schedules [`publish_async`] on the current Tokio runtime when one
    /// exists; otherwise dispatches synchronously (e.g. tests without a runtime).
    ///
    /// C# `Publish` calls `PublishAsync(...).ConfigureAwait(false)` without awaiting — same idea.
    pub fn publish<T: MessageBusEvent + Clone + Send + 'static>(&self, msg: T) {
        match tokio::runtime::Handle::try_current() {
            Ok(handle) => {
                let bus = self.clone();
                drop(handle.spawn(async move {
                    bus.publish_async(msg).await;
                }));
            }
            Err(_) => self.dispatch_sync(msg),
        }
    }

    /// Awaits publication to all subscribers. Handlers run synchronously in order; the future
    /// completes when they finish.
    pub async fn publish_async<T: MessageBusEvent + Clone + Send + 'static>(&self, msg: T) {
        self.dispatch_sync(msg);
    }

    fn dispatch_sync<T: MessageBusEvent + Clone + Send + 'static>(&self, msg: T) {
        let tid = TypeId::of::<T>();
        if !self
            .inner
            .registered
            .read()
            .expect("message bus lock poisoned")
            .contains(&tid)
        {
            return;
        }
        let invokers: Vec<_> = {
            let map = self
                .inner
                .handlers
                .read()
                .expect("message bus lock poisoned");
            map.get(&tid)
                .map(|slots| slots.iter().map(|s| s.invoke.clone()).collect())
                .unwrap_or_default()
        };
        for invoke in invokers {
            invoke(Box::new(msg.clone()) as Box<dyn Any + Send>);
        }
    }
}

impl Default for InMemoryMessageBus {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use std::sync::atomic::{AtomicU32, Ordering as AtomicOrdering};
    use tokio::sync::oneshot;

    #[derive(Clone, Debug, PartialEq, Eq)]
    struct Ping(u32);

    #[tokio::test]
    async fn publish_delivers_to_subscriber() {
        let bus = InMemoryMessageBus::new();
        let hits = Arc::new(AtomicU32::new(0));
        let h = hits.clone();
        let _sub = bus.subscribe(move |p: Ping| {
            h.fetch_add(p.0, AtomicOrdering::SeqCst);
        });
        bus.publish_async(Ping(3)).await;
        assert_eq!(hits.load(AtomicOrdering::SeqCst), 3);
    }

    #[tokio::test]
    async fn publish_unregistered_is_noop() {
        let bus = InMemoryMessageBus::new();
        bus.publish_async(Ping(1)).await;
    }

    #[tokio::test]
    async fn drop_subscription_unregisters_type() {
        let bus = InMemoryMessageBus::new();
        let hits = Arc::new(AtomicU32::new(0));
        let h = hits.clone();
        let sub = bus.subscribe(move |_p: Ping| {
            h.fetch_add(1, AtomicOrdering::SeqCst);
        });
        bus.publish_async(Ping(1)).await;
        assert_eq!(hits.load(AtomicOrdering::SeqCst), 1);
        drop(sub);
        bus.publish_async(Ping(1)).await;
        assert_eq!(hits.load(AtomicOrdering::SeqCst), 1);
    }

    #[test]
    fn publish_without_runtime_runs_sync() {
        let bus = InMemoryMessageBus::new();
        let hits = Arc::new(AtomicU32::new(0));
        let h = hits.clone();
        let _sub = bus.subscribe(move |p: Ping| {
            h.fetch_add(p.0, AtomicOrdering::SeqCst);
        });
        bus.publish(Ping(5));
        assert_eq!(hits.load(AtomicOrdering::SeqCst), 5);
    }

    #[tokio::test]
    async fn publish_spawns_async_on_runtime() {
        let bus = InMemoryMessageBus::new();
        let (tx, rx) = oneshot::channel();
        let gate = std::sync::Mutex::new(Some(tx));
        let gate = Arc::new(gate);
        let g = Arc::clone(&gate);
        let _sub = bus.subscribe(move |_p: Ping| {
            if let Some(sender) = g.lock().expect("lock poisoned").take() {
                let _ = sender.send(());
            }
        });
        bus.publish(Ping(0));
        rx.await.expect("handler should run");
    }
}
