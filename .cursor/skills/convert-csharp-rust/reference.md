# C# to Rust Migration — Detailed Reference

Companion to `SKILL.md`. Read on demand for deep patterns, crate tables, and authoritative links.

---

## Authoritative References

### C# / .NET Official Docs
| Topic | URL |
|---|---|
| C# Language Reference | https://learn.microsoft.com/dotnet/csharp/language-reference/ |
| C# Language Specification | https://learn.microsoft.com/dotnet/csharp/language-reference/language-specification/ |
| Types (classes/structs/records) | https://learn.microsoft.com/dotnet/csharp/fundamentals/types/ |
| Records | https://learn.microsoft.com/dotnet/csharp/language-reference/builtin-types/record |
| Interfaces | https://learn.microsoft.com/dotnet/csharp/fundamentals/types/interfaces |
| Inheritance / Polymorphism | https://learn.microsoft.com/dotnet/csharp/fundamentals/object-oriented/inheritance |
| Generics | https://learn.microsoft.com/dotnet/csharp/fundamentals/types/generics |
| Async / Await | https://learn.microsoft.com/dotnet/csharp/asynchronous-programming/ |
| LINQ | https://learn.microsoft.com/dotnet/csharp/linq/ |
| Iterators (`yield`) | https://learn.microsoft.com/dotnet/csharp/language-reference/statements/yield |
| Delegates | https://learn.microsoft.com/dotnet/csharp/programming-guide/delegates/ |
| Events | https://learn.microsoft.com/dotnet/csharp/programming-guide/events/ |
| Exceptions | https://learn.microsoft.com/dotnet/csharp/fundamentals/exceptions/ |
| Nullable reference types | https://learn.microsoft.com/dotnet/csharp/nullable-references |
| Unsafe code / pointers | https://learn.microsoft.com/dotnet/csharp/language-reference/unsafe-code |
| `Span<T>` / `Memory<T>` | https://learn.microsoft.com/dotnet/standard/memory-and-spans/ |
| `IDisposable` / `using` | https://learn.microsoft.com/dotnet/standard/garbage-collection/using-objects |
| Reflection | https://learn.microsoft.com/dotnet/csharp/advanced-topics/reflection-and-attributes/ |
| Pattern matching | https://learn.microsoft.com/dotnet/csharp/fundamentals/functional/pattern-matching |

### Rust Official Docs
| Topic | URL |
|---|---|
| The Rust Book | https://doc.rust-lang.org/book/ |
| Rust Reference | https://doc.rust-lang.org/reference/ |
| Standard Library | https://doc.rust-lang.org/std/ |
| Ownership / Borrowing | https://doc.rust-lang.org/book/ch04-00-understanding-ownership.html |
| Enums / Pattern matching | https://doc.rust-lang.org/book/ch06-00-enums.html |
| Traits | https://doc.rust-lang.org/book/ch10-02-traits.html |
| Error handling (`Option`, `Result`) | https://doc.rust-lang.org/book/ch09-00-error-handling.html |
| Iterators | https://doc.rust-lang.org/book/ch13-02-iterators.html |
| Lifetimes | https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html |
| Async Book | https://rust-lang.github.io/async-book/ |
| Sync primitives | https://doc.rust-lang.org/std/sync/ |
| Channels (`mpsc`) | https://doc.rust-lang.org/std/sync/mpsc/ |
| Rustonomicon (unsafe) | https://doc.rust-lang.org/nomicon/ |
| Unsafe Code Guidelines | https://rust-lang.github.io/unsafe-code-guidelines/ |
| Rust API Guidelines | https://rust-lang.github.io/api-guidelines/ |

### Migration-Specific Guides
| Guide | URL |
|---|---|
| Microsoft: Rust for C#/.NET Devs | https://microsoft.github.io/rust-for-dotnet-devs/latest/ |
| OOP Migration Patterns | https://www.slingacademy.com/article/migrating-oop-design-patterns-to-rust/ |
| Tokio Cancellation Patterns | https://cybernetist.com/2024/04/19/rust-tokio-task-cancellation-patterns/ |

---

## LINQ → Rust Iterator Mapping

### Lazy (maps cleanly)

| LINQ | Rust | Notes |
|---|---|---|
| `.Where(pred)` | `.filter(pred)` | Pred receives `&&T` for `.iter()` |
| `.Select(f)` | `.map(f)` | |
| `.SelectMany(f)` | `.flat_map(f)` | |
| `.Take(n)` | `.take(n)` | |
| `.Skip(n)` | `.skip(n)` | |
| `.TakeWhile(pred)` | `.take_while(pred)` | |
| `.SkipWhile(pred)` | `.skip_while(pred)` | |
| `.Zip(other)` | `.zip(other)` | |
| `.Concat(other)` | `.chain(other)` | |
| `.Cast<T>()` | `.map(\|x\| x as T)` | |

### Eager / Terminal

| LINQ | Rust | Notes |
|---|---|---|
| `.First()` | `.next()` | On the iterator directly |
| `.FirstOrDefault()` | `.next()` → `Option<T>` | |
| `.First(pred)` | `.find(pred)` | |
| `.Any(pred)` | `.any(pred)` | |
| `.All(pred)` | `.all(pred)` | |
| `.Count()` | `.count()` | Consumes iterator |
| `.Sum()` | `.sum()` | Requires `Sum` trait |
| `.Min()` / `.Max()` | `.min()` / `.max()` | Returns `Option` |
| `.Aggregate(seed, f)` | `.fold(seed, f)` | |
| `.Aggregate(f)` | `.reduce(f)` | Returns `Option` |
| `.ToList()` / `.ToArray()` | `.collect::<Vec<_>>()` | |
| `.ToDictionary(k,v)` | `.map(\|x\| (k(x),v(x))).collect::<HashMap<_,_>>()` | |
| `.ToHashSet()` | `.collect::<HashSet<_>>()` | |
| `.Reverse()` | `.rev()` | Needs `DoubleEndedIterator` |

### Requires Collection (not lazy)

| LINQ | Rust | Why |
|---|---|---|
| `.OrderBy(key)` | collect → `.sort_by_key()` → re-iterate | Sorting is inherently non-lazy |
| `.GroupBy(key)` | collect into `HashMap<K, Vec<V>>` | SQL-style grouping needs all items |
| `.Distinct()` | `itertools::unique()` or `HashSet` | Must track seen items |

### Generators / Factories

| LINQ | Rust |
|---|---|
| `Enumerable.Range(start, count)` | `(start..start+count)` |
| `Enumerable.Repeat(val, count)` | `std::iter::repeat(val).take(count)` |
| `Enumerable.Empty<T>()` | `std::iter::empty::<T>()` |

---

## Serialization Attribute Mapping

| C# (`System.Text.Json`) | Rust (`serde`) |
|---|---|
| `[JsonPropertyName("name")]` | `#[serde(rename = "name")]` |
| `[JsonIgnore]` | `#[serde(skip)]` |
| `WhenWritingNull` | `#[serde(skip_serializing_if = "Option::is_none")]` |
| `[JsonConverter]` | `#[serde(with = "module")]` |
| `[JsonConstructor]` | `#[serde(default)]` |
| `CamelCase` policy | `#[serde(rename_all = "camelCase")]` |
| `SnakeCaseLower` policy | `#[serde(rename_all = "snake_case")]` |
| `[JsonDerivedType]` | `#[serde(tag = "type")]` |
| `[JsonStringEnumConverter]` | Derive `Serialize`/`Deserialize` on enum |

### Format Ecosystem

| Format | C# Library | Rust Crate |
|---|---|---|
| JSON | System.Text.Json / Newtonsoft | `serde_json` |
| YAML | YamlDotNet | `serde_yaml` |
| TOML | Tomlyn | `toml` |
| MessagePack | MessagePack-CSharp | `rmp-serde` |
| XML | System.Xml | `quick-xml` + serde |
| CSV | CsvHelper | `csv` |
| Binary | BinaryFormatter | `bincode` |

---

## Crate Equivalents for Common .NET Libraries

### Web / HTTP

| C# | Rust Crate | Description |
|---|---|---|
| `HttpClient` | **reqwest** | Async/sync HTTP client, cookies, JSON, proxy |
| ASP.NET Core | **axum** | Tokio-native web framework, extractors, routing |
| ASP.NET Core | **actix-web** | Actor-based, high throughput |
| SignalR | **tokio-tungstenite** | WebSocket support |

### Database

| C# | Rust Crate | Description |
|---|---|---|
| Entity Framework | **diesel** | ORM, compile-time SQL validation |
| EF / Dapper | **sqlx** | Async-first, raw SQL, compile-time checked |
| EF (code-first) | **sea-orm** | Async ORM, ActiveRecord, migrations |

### Infrastructure

| C# | Rust Crate | Description |
|---|---|---|
| `ILogger` / Serilog | **tracing** + **tracing-subscriber** | Structured logging with spans |
| `IConfiguration` | **config** or **figment** | Layered file/env/arg config |
| System.CommandLine | **clap** (derive) | CLI argument parser |
| AutoMapper | `From`/`Into` traits | Built-in language feature |
| FluentValidation | **validator** | Derive-based validation |

### Data / Serialization

| C# | Rust Crate | Description |
|---|---|---|
| System.Text.Json | **serde** + **serde_json** | Derive-macro serialization |
| `DateTime` / NodaTime | **chrono** or **time** or **jiff** | Date/time handling |
| `Regex` | **regex** | Linear-time, no backtracking |
| `string` operations | `String`/`&str` + **regex** | UTF-8 based |

### Concurrency / Async

| C# | Rust Crate | Description |
|---|---|---|
| `Task` / TPL / async | **tokio** | Async runtime, I/O, timers, channels |
| `Parallel.For` | **rayon** | Data-parallelism, `par_iter()` |
| `Channel<T>` | **tokio::sync::mpsc** / **crossbeam::channel** | Message passing |
| `SemaphoreSlim` | **tokio::sync::Semaphore** | Async semaphore |

### Error Handling

| C# | Rust Crate | Description |
|---|---|---|
| Custom exceptions (libs) | **thiserror** | Derive `Error` enum |
| Catch-all (apps) | **anyhow** | Ergonomic error context |
| `InnerException` chain | **eyre** / **color-eyre** | Custom error reports |

### Testing

| C# | Rust Crate | Description |
|---|---|---|
| xUnit / NUnit | **built-in `#[test]`** + **rstest** | Unit + parameterized tests |
| Moq / NSubstitute | **mockall** | `#[automock]` for traits |
| Bogus | **fake** | Fake data generation |
| FluentAssertions | **pretty_assertions** | Better diff output |
| Benchmark.NET | **criterion** | Statistics-driven benchmarking |
| coverlet | **cargo-llvm-cov** | Code coverage |
| WireMock.Net | **wiremock** | HTTP mock server |

### Crypto / Security

| C# | Rust Crate | Description |
|---|---|---|
| System.Security.Cryptography | **ring** | AES, SHA, ECDSA, RSA |
| `SslStream` | **rustls** | Pure-Rust TLS 1.2/1.3 |
| X509 generation | **rcgen** | Certificate/CSR generation |

### Messaging / Caching

| C# | Rust Crate | Description |
|---|---|---|
| RabbitMQ.Client | **lapin** | Async AMQP 0.9.1 |
| Confluent.Kafka | **rdkafka** | Kafka client |
| `IMemoryCache` | **moka** | Concurrent in-memory cache |
| `IDistributedCache` (Redis) | **redis** | Async Redis client |
| Polly (resilience) | **backoff** / **retry** | Retry with backoff |
| gRPC | **tonic** + **prost** | Async gRPC over HTTP/2 |

### Recommended Default Stack

| Layer | Crate |
|---|---|
| Web framework | **axum** |
| HTTP client | **reqwest** |
| Database | **sqlx** or **sea-orm** |
| Serialization | **serde** + **serde_json** |
| Async runtime | **tokio** |
| Logging | **tracing** |
| Errors (libs) | **thiserror** |
| Errors (apps) | **anyhow** |
| CLI | **clap** |
| Config | **config** |
| Testing | built-in + **mockall** + **proptest** |

---

## String Handling: UTF-16 → UTF-8

| C# | Rust | Warning |
|---|---|---|
| `string` (UTF-16, immutable) | `String` (UTF-8, owned) / `&str` (borrowed) | Encoding difference |
| `char` (2 bytes, UTF-16 unit) | `char` (4 bytes, Unicode scalar) | Different sizes |
| `str.Length` (char count) | `.len()` (byte count) / `.chars().count()` | `.len()` is bytes! |
| `str[i]` (index by char) | `.chars().nth(i)` | No direct indexing |
| `$"interp {x}"` | `format!("interp {x}")` | |
| `StringBuilder` | `String` + `push_str` / `format!` | Pre-allocate: `String::with_capacity(n)` |
| `@"raw\string"` | `r#"raw\string"#` | |
| `"hello"u8` (C# 11) | `b"hello"` | Byte string literal |
| `str.IsNullOrEmpty()` | `.is_empty()` + `Option<String>` for null | |
| `str.Contains("sub")` | `.contains("sub")` | |
| `str.Split(',')` | `.split(',')` → lazy iterator | |
| `str.Trim()` | `.trim()` | |
| `str.ToUpper()` | `.to_uppercase()` | Returns new `String` |

---

## Property System → Accessor Methods

| C# Pattern | Rust Equivalent |
|---|---|
| `public int X { get; }` | `pub fn x(&self) -> i32` |
| `public int X { get; set; }` | `pub fn x(&self) -> i32` + `pub fn set_x(&mut self, v: i32)` |
| Auto-property, no logic | Make field `pub` directly |
| `{ get; private set; }` | Private field + `pub fn getter` only |
| `{ get; init; }` | Builder pattern or `pub` field on construction-only struct |
| Computed property | Method returning computed value |

---

## Memory Model: GC → Ownership

| C# / .NET | Rust | Notes |
|---|---|---|
| Heap allocation (implicit) | Stack default; heap via `Box`, `Vec`, `String` | Explicit choice |
| `GC.Collect()` | Not needed | Deterministic destruction |
| `WeakReference<T>` | `Weak<T>` (with `Rc` / `Arc`) | `.upgrade()` to use |
| Finalizer (`~ClassName()`) | `Drop` trait | Deterministic, on scope exit |
| `IDisposable` + `using` | RAII + `Drop` | No separate pattern needed |
| Circular references | `Weak<T>`, arena allocation, or index-based graphs | GC handled these automatically |

### Circular Reference Strategies
1. **`Weak<T>`** — break cycles with `Rc<T>` + `Weak<T>` (or `Arc` + `Weak`).
2. **Arena** — `typed-arena` or `bumpalo`: all nodes share arena lifetime.
3. **Index-based** — `Vec<Node>` + `usize` indices; `petgraph` for graphs.

---

## Interop: C# ↔ Rust via C ABI

### Tooling

| Tool | Approach | Best For |
|---|---|---|
| **csbindgen** | Auto-generates C# P/Invoke from `extern "C" fn` | Direct FFI, production-proven |
| **uniffi-bindgen-cs** | UDL definitions → C# bindings | Higher-level API, multi-language |

### FFI Safety Rules
1. All types crossing the boundary must be C-compatible (`#[repr(C)]`).
2. Strings: C# `string` (UTF-16) ↔ `CString` (null-terminated UTF-8) or raw `ptr + len`.
3. Memory allocated in Rust freed in Rust (and vice versa).
4. Keep `unsafe` in thin boundary modules with documented invariants.
5. Prefer simple types (integers, pointers, ptr+len slices) over complex structures.

---

## Validation Checklist

- [ ] Behavior parity tests against C# baseline.
- [ ] Unit + integration tests per translated module.
- [ ] Property tests for algorithmic logic (`proptest` / `quickcheck`).
- [ ] `cargo fmt --check`
- [ ] `cargo clippy -- -D warnings`
- [ ] `cargo test`
- [ ] Zero `unsafe` unless justified; each block has documented invariants.
- [ ] No `.unwrap()` / `.expect()` in production paths unless invariant-justified.
- [ ] Error enums are typed and surfaced at boundaries.
- [ ] Benchmark key paths with `criterion` if performance-sensitive.
- [ ] `cargo +nightly miri run` for unsafe-heavy code.
