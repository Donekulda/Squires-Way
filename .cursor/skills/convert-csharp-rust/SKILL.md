---
name: convert-csharp-rust
description: Convert C# code to idiomatic Rust. Use when migrating C# projects to Rust, translating C# OOP/async/LINQ/exception patterns to Rust ownership and trait-based design, restructuring class hierarchies, or planning incremental C# to Rust rewrites with behavior parity and safety. Triggers on C# migration, .NET to Rust, CSharp conversion.
---

# Convert C# to Rust

Behavior-parity-first conversion from C# / .NET to idiomatic Rust. Covers OOP restructuring, async migration, error model replacement, LINQ translation, GC-to-ownership shift, and library ecosystem mapping.

## Quick Start

1. Classify migration mode: **full rewrite**, **module-by-module**, or **interop boundary**.
2. Build a feature map (C# construct -> Rust construct) before editing code.
3. Preserve behavior first (golden tests), then refactor to idiomatic Rust.
4. Prefer explicit ownership and typed errors over hidden runtime behavior.
5. Keep `unsafe` isolated and documented; avoid it unless FFI requires it.

## This Skill Covers

- OOP hierarchy redesign (classes/interfaces/inheritance -> enums/traits/composition)
- Async/await migration (`Task<T>` -> Futures, runtime selection, cancellation)
- Error model replacement (exceptions -> `Result<T, E>`)
- LINQ-to-iterator translation
- GC-to-ownership memory model shift
- Generics constraint mapping (covariance/contravariance)
- String handling (UTF-16 -> UTF-8)
- Collections, properties, serialization, DI, and testing migration
- C#/Rust interop via C ABI when needed
- Crate equivalents for common .NET libraries

## This Skill Does NOT Cover

- C# language primer — see https://learn.microsoft.com/dotnet/csharp/
- Rust language primer — see https://doc.rust-lang.org/book/
- C to Rust conversion — see `convert-c-rust`
- General conversion methodology — see `meta-convert-dev`

---

## Migration Workflow (APTV)

### 1) Analyze
- Inventory C# features: inheritance depth, reflection, async, events, nullable refs, unsafe code.
- Identify hot paths, public API surface, and behavior-critical invariants.
- Decide what must remain API-compatible vs. what can be redesigned.

### 2) Plan
- Produce a per-module feature mapping table.
- Define error model (`Result<T, E>` boundaries) and ownership model.
- Pick concurrency model: threads + channels vs. async runtime (Tokio recommended).
- Identify crate substitutions for .NET libraries used.

### 3) Transform
- Port by module with tests alongside.
- Replace nullable and exception flows early (`Option`, `Result`).
- Replace inheritance with composition + traits + enums (see decision tree below).
- Translate LINQ to iterators or explicit loops for readability/perf.

### 4) Validate
- Run parity tests against original C# behavior.
- `cargo fmt --check && cargo clippy -- -D warnings && cargo test`
- Benchmark critical paths with `criterion`.
- Refactor only after behavior is stable.

---

## OOP Hierarchy Decision Tree

When encountering a C# class hierarchy, choose the Rust model:

```
Is the set of subtypes closed (known at compile time)?
├── YES → enum with variants
│         (state machines, message types, AST nodes, command types)
├── NO  → Must consumers extend subtypes?
│   ├── YES → trait (+ dyn Trait for heterogeneous collections)
│   │         (plugin systems, handler registries, middleware)
│   └── NO  → struct composition
│             (shared state/behavior reuse, entity hierarchies)
```

**Rules:**
- Prefer enums when the variant set is closed — exhaustive matching catches new variants at compile time.
- Prefer static dispatch (`impl Trait` / generics) over `dyn Trait` unless you need heterogeneous containers.
- Never emulate deep C# inheritance; flatten and compose.

---

## Feature Mapping (Default Choices)

| C# | Rust | Notes |
|---|---|---|
| `class` | `struct` + `impl` | Ownership-aware methods |
| `record` | `struct` + derives (`Debug, Clone, Eq, PartialEq, Hash`) | Add `Copy` only for small plain-value types |
| `interface` | `trait` | Static dispatch default; `dyn Trait` when needed |
| inheritance hierarchy | enum / traits / composition | See decision tree above |
| `Nullable<T>` / nullable refs | `Option<T>` | Pattern-match, propagate with `?` |
| exceptions | `Result<T, E>` + `?` | `thiserror` for libs, `anyhow` for apps |
| delegates (`Func`, `Action`) | closures / `Fn`, `FnMut`, `FnOnce` | Pick trait intentionally |
| events | callback registry or channels | Keep ownership explicit |
| LINQ | iterator adapters / explicit loops | See LINQ table in reference.md |
| `Task<T>` | `async fn` -> `impl Future<Output=T>` | Futures are lazy (cold) |
| `CancellationToken` | `tokio_util::sync::CancellationToken` / `select!` | Drop future = cancel |
| `IDisposable` + `using` | RAII + `Drop` + lexical scope | `drop(x)` for early release |
| `Span<T>` / `Memory<T>` | `&[T]`, `&mut [T]`, `Vec<T>` | Lifetime-checked windows |
| `lock` / `Monitor` | `Mutex`, `RwLock`, channels | Minimize lock scope |
| `StringBuilder` | `String` (mutable) | `push_str`, `format!` |
| `string` (UTF-16) | `String` / `&str` (UTF-8) | `.len()` = bytes, not chars |
| reflection | traits / enums / macros / registries | Compile-time preferred |
| DI container | manual wiring / generics / trait objects | See DI section below |
| `System.Text.Json` attrs | `serde` derive + attributes | See serialization mapping in reference.md |

## Async Migration Key Differences

| Aspect | C# | Rust |
|---|---|---|
| Runtime | Built into CLR | Explicit: Tokio / async-std |
| Task start | Hot (starts immediately) | Cold (lazy until polled) |
| Cancellation | `CancellationToken` | Drop the future / `select!` |
| Blocking offload | `Task.Run()` | `tokio::task::spawn_blocking()` |
| Heap cost | Every `Task<T>` heap-allocated | Zero-cost until spawned |

**Never** use `std::thread::sleep()` in async; use `tokio::time::sleep().await`.

## Collections Mapping

| C# | Rust | Notes |
|---|---|---|
| `List<T>` | `Vec<T>` | |
| `Dictionary<K,V>` | `HashMap<K,V>` | `K: Eq + Hash` |
| `SortedDictionary<K,V>` | `BTreeMap<K,V>` | `K: Ord` |
| `HashSet<T>` | `HashSet<T>` | |
| `Queue<T>` | `VecDeque<T>` | |
| `Stack<T>` | `Vec<T>` (`push`/`pop`) | |
| `PriorityQueue<T>` | `BinaryHeap<T>` | Max-heap; `Reverse` for min |
| `ConcurrentDictionary` | `dashmap::DashMap` | |
| `Array` (fixed) | `[T; N]` | Stack-allocated |

## Generics Constraint Mapping

| C# | Rust | Notes |
|---|---|---|
| `where T : class` | (no direct equiv) | |
| `where T : struct` | `T: Copy` (approx) | |
| `where T : new()` | `T: Default` | `T::default()` |
| `where T : IComparable<T>` | `T: Ord` / `T: PartialOrd` | |
| `where T : IEquatable<T>` | `T: Eq` / `T: PartialEq` | |
| `where T : IEnumerable<U>` | `T: IntoIterator<Item=U>` | |
| `where T : SomeInterface` | `T: SomeTrait` | |
| Multiple | `T: TraitA + TraitB + Clone` | `+` syntax |

Variance is automatic in Rust (no `in`/`out` keywords needed).

## Dependency Injection

C# DI containers -> Rust alternatives (in order of preference):

1. **Manual wiring** (most idiomatic): construct and pass dependencies in `main()`.
2. **Generics** (zero-cost): `struct App<R: UserRepo, E: EmailService>`.
3. **Trait objects**: `Arc<dyn Service>` for runtime polymorphism.
4. **DI crates** (rare): `shaku`, `ferrunix` when runtime resolution needed.

## Interop Strategy

| Scenario | Strategy |
|---|---|
| Perf-critical inner loop | Extract to Rust lib, call via P/Invoke FFI |
| Gradual migration | Module-by-module with C ABI boundary |
| Complete rewrite feasible | Skip interop, rewrite entirely |

**Tooling:** `csbindgen` (auto-generates C# P/Invoke bindings from Rust `extern "C" fn`).

---

## Non-Negotiable Rules

- Preserve observable behavior before redesigning internals.
- Do not emulate deep class inheritance directly; redesign the data model.
- Avoid blanket `Arc<Mutex<_>>`; choose ownership first, locks second.
- Do not replace exceptions with `panic!` except for true invariants.
- Keep lock scope small; never hold locks across `.await`.
- Treat `unsafe` as boundary code; explain invariants inline.
- Use `thiserror` for library errors, `anyhow` for application errors.
- No `.unwrap()` / `.expect()` in production paths unless invariant-justified.

## Output Template

When converting code, produce:

1. **Migration mode**: rewrite / incremental / interop.
2. **Feature map**: concise C# -> Rust mapping for this module.
3. **Rust implementation**: behavior-preserving first.
4. **Risk notes**: nullability, error-flow, async/concurrency hazards.
5. **Validation commands**:
   - `cargo fmt --check`
   - `cargo clippy -- -D warnings`
   - `cargo test`

## Pitfalls Checklist

- [ ] Inheritance copied literally into trait-object sprawl.
- [ ] `Option` immediately unwrapped instead of propagated.
- [ ] `panic!` used as normal error handling.
- [ ] Locks held across `.await`.
- [ ] Excess allocations from naive LINQ-to-iterator translation.
- [ ] Reflection ported blindly instead of compile-time redesign.
- [ ] Excess cloning to fight borrow checker instead of API redesign.
- [ ] `Arc<Mutex<_>>` everywhere instead of ownership-first design.
- [ ] UTF-16 string assumptions (indexing, `.Length`) carried into UTF-8.
- [ ] GC timing assumptions (finalizer order, lazy cleanup) carried over.

## Additional Resources

- Detailed references and crate mapping: [reference.md](reference.md)
- Before/after code examples: [examples.md](examples.md)
- Microsoft official guide: https://microsoft.github.io/rust-for-dotnet-devs/latest/
