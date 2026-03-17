# C# → Rust Conversion Examples

Before/after examples for common migration patterns. Read on demand from `SKILL.md`.

---

## 1. Abstract Hierarchy → Enum (Closed Set)

**C#:**
```csharp
public abstract class Shape {
    public abstract double Area();
}
public class Circle : Shape {
    public double Radius { get; }
    public Circle(double r) => Radius = r;
    public override double Area() => Math.PI * Radius * Radius;
}
public class Rect : Shape {
    public double W { get; }
    public double H { get; }
    public Rect(double w, double h) { W = w; H = h; }
    public override double Area() => W * H;
}
```

**Rust:**
```rust
enum Shape {
    Circle { radius: f64 },
    Rect { w: f64, h: f64 },
}

impl Shape {
    fn area(&self) -> f64 {
        match self {
            Shape::Circle { radius } => std::f64::consts::PI * radius * radius,
            Shape::Rect { w, h } => w * h,
        }
    }
}
```

---

## 2. Interface → Trait (Open Extension)

**C#:**
```csharp
public interface IHandler {
    bool CanHandle(Request req);
    Response Handle(Request req);
}

public class Pipeline {
    private readonly List<IHandler> _handlers;
    public Pipeline(IEnumerable<IHandler> handlers) => _handlers = handlers.ToList();

    public Response? Process(Request req) =>
        _handlers.FirstOrDefault(h => h.CanHandle(req))?.Handle(req);
}
```

**Rust:**
```rust
trait Handler: Send + Sync {
    fn can_handle(&self, req: &Request) -> bool;
    fn handle(&self, req: &Request) -> Response;
}

struct Pipeline {
    handlers: Vec<Box<dyn Handler>>,
}

impl Pipeline {
    fn process(&self, req: &Request) -> Option<Response> {
        self.handlers.iter()
            .find(|h| h.can_handle(req))
            .map(|h| h.handle(req))
    }
}
```

---

## 3. Deep Inheritance → Composition

**C#:**
```csharp
public class Entity { public int Id { get; set; } public string Name { get; set; } }
public class LivingEntity : Entity { public int Health { get; set; } }
public class Player : LivingEntity { public int Score { get; set; } }
```

**Rust:**
```rust
struct Entity {
    id: u32,
    name: String,
}

struct LivingStats {
    health: i32,
}

struct Player {
    entity: Entity,
    stats: LivingStats,
    score: u32,
}

trait Named {
    fn name(&self) -> &str;
}

impl Named for Player {
    fn name(&self) -> &str { &self.entity.name }
}
```

---

## 4. Exception Hierarchy → Error Enum

**C#:**
```csharp
public class AppException : Exception { }
public class ValidationException : AppException {
    public string Field { get; }
    public ValidationException(string field, string msg) : base(msg) { Field = field; }
}
public class NotFoundException : AppException {
    public string ResourceId { get; }
}

try {
    var user = await GetUser(id);
    Validate(user);
} catch (NotFoundException ex) {
    return NotFound(ex.ResourceId);
} catch (ValidationException ex) {
    return BadRequest(ex.Field, ex.Message);
} catch (Exception ex) {
    logger.LogError(ex, "Unexpected");
    return InternalServerError();
}
```

**Rust:**
```rust
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("validation failed on '{field}': {message}")]
    Validation { field: String, message: String },

    #[error("not found: {resource_id}")]
    NotFound { resource_id: String },

    #[error("internal error")]
    Internal(#[from] anyhow::Error),
}

fn handle_request(id: &str) -> Result<Response, AppError> {
    let user = get_user(id)?;
    validate(&user)?;
    Ok(Response::ok(user))
}

match handle_request("123") {
    Ok(resp) => resp,
    Err(AppError::NotFound { resource_id }) => not_found_response(&resource_id),
    Err(AppError::Validation { field, message }) => bad_request(&field, &message),
    Err(e) => {
        tracing::error!(?e, "unexpected error");
        internal_error_response()
    }
}
```

---

## 5. try/catch/finally → Result + RAII

**C#:**
```csharp
try {
    using var conn = OpenConnection();
    var result = conn.Query(sql);
    return result;
} catch (SqlException ex) {
    logger.Log(ex);
    throw;
}
// finally / using ensures conn.Dispose() runs
```

**Rust:**
```rust
fn query(sql: &str) -> Result<QueryResult, DbError> {
    let conn = open_connection()?; // auto-closed via Drop on any exit path
    let result = conn.query(sql)?;
    Ok(result)
}
```

---

## 6. Async/Await + Cancellation

**C#:**
```csharp
async Task ProcessAsync(CancellationToken ct) {
    while (!ct.IsCancellationRequested) {
        var result = await DoWorkAsync(ct);
        await Task.Delay(1000, ct);
    }
}
```

**Rust:**
```rust
use tokio_util::sync::CancellationToken;

async fn process(cancel: CancellationToken) {
    loop {
        tokio::select! {
            _ = cancel.cancelled() => break,
            result = do_work() => {
                // use result
            }
        }
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    }
}
```

---

## 7. LINQ Chain → Iterator Pipeline

**C#:**
```csharp
var result = users
    .Where(u => u.IsActive)
    .OrderBy(u => u.Name)
    .Select(u => new { u.Name, u.Email })
    .Take(10)
    .ToList();
```

**Rust:**
```rust
let mut active: Vec<_> = users.iter()
    .filter(|u| u.is_active)
    .collect();
active.sort_by_key(|u| &u.name);

let result: Vec<_> = active.into_iter()
    .map(|u| (u.name.clone(), u.email.clone()))
    .take(10)
    .collect();
```

Note: `OrderBy` breaks the lazy chain — sorting requires collection first.

---

## 8. GroupBy

**C#:**
```csharp
var groups = people
    .GroupBy(p => p.City)
    .Select(g => new { City = g.Key, Count = g.Count() });
```

**Rust:**
```rust
use std::collections::HashMap;

let mut groups: HashMap<&str, usize> = HashMap::new();
for person in &people {
    *groups.entry(&person.city).or_default() += 1;
}
let summary: Vec<_> = groups.into_iter().collect();
```

---

## 9. Dependency Injection → Manual Wiring

**C#:**
```csharp
// Startup.cs
services.AddScoped<IUserRepo, PgUserRepo>();
services.AddScoped<IEmailService, SmtpEmailService>();
services.AddScoped<UserService>();

// UserService.cs
public class UserService {
    private readonly IUserRepo _repo;
    private readonly IEmailService _email;
    public UserService(IUserRepo repo, IEmailService email) {
        _repo = repo; _email = email;
    }
}
```

**Rust (manual wiring — most idiomatic):**
```rust
trait UserRepo: Send + Sync {
    fn find(&self, id: u64) -> Option<User>;
}
trait EmailService: Send + Sync {
    fn send(&self, to: &str, body: &str) -> Result<(), String>;
}

struct UserService {
    repo: Arc<dyn UserRepo>,
    email: Arc<dyn EmailService>,
}

// Composition root in main()
fn main() {
    let repo: Arc<dyn UserRepo> = Arc::new(PgUserRepo::new(pool));
    let email: Arc<dyn EmailService> = Arc::new(SmtpEmailService::new(config));
    let svc = UserService { repo, email };
}
```

**Rust (generics — zero-cost, testable):**
```rust
struct UserService<R: UserRepo, E: EmailService> {
    repo: R,
    email: E,
}

// Production
let svc = UserService::new(PgUserRepo::new(pool), SmtpEmailService::new(config));
// Test
let svc = UserService::new(MockUserRepo::new(), MockEmailService::new());
```

---

## 10. Serialization: JSON

**C#:**
```csharp
public class Person {
    [JsonPropertyName("full_name")]
    public string Name { get; set; }

    [JsonIgnore(Condition = JsonIgnoreCondition.WhenWritingNull)]
    public string? Nickname { get; set; }

    public int Age { get; set; }
}

var json = JsonSerializer.Serialize(person);
var person = JsonSerializer.Deserialize<Person>(json);
```

**Rust:**
```rust
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
struct Person {
    #[serde(rename = "full_name")]
    name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    nickname: Option<String>,

    age: u32,
}

let json = serde_json::to_string(&person)?;
let person: Person = serde_json::from_str(&json)?;
```

---

## 11. Properties → Accessor Methods

**C#:**
```csharp
public class Temperature {
    private double _celsius;
    public double Celsius {
        get => _celsius;
        set {
            if (value < -273.15)
                throw new ArgumentOutOfRangeException(nameof(value));
            _celsius = value;
        }
    }
    public double Fahrenheit => _celsius * 9.0 / 5.0 + 32.0;
}
```

**Rust:**
```rust
struct Temperature {
    celsius: f64,
}

impl Temperature {
    pub fn new(celsius: f64) -> Result<Self, String> {
        if celsius < -273.15 {
            return Err(format!("{celsius} below absolute zero"));
        }
        Ok(Self { celsius })
    }

    pub fn celsius(&self) -> f64 { self.celsius }

    pub fn set_celsius(&mut self, value: f64) -> Result<(), String> {
        if value < -273.15 {
            return Err(format!("{value} below absolute zero"));
        }
        self.celsius = value;
        Ok(())
    }

    pub fn fahrenheit(&self) -> f64 {
        self.celsius * 9.0 / 5.0 + 32.0
    }
}
```

---

## 12. Testing: xUnit → cargo test

**C#:**
```csharp
public class MathTests {
    [Fact]
    public void Add_ReturnsSum() => Assert.Equal(5, Math.Add(2, 3));

    [Theory]
    [InlineData(0, 0, 0)]
    [InlineData(-1, 1, 0)]
    public void Add_Parameterized(int a, int b, int expected)
        => Assert.Equal(expected, Math.Add(a, b));
}
```

**Rust:**
```rust
fn add(a: i32, b: i32) -> i32 { a + b }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_returns_sum() {
        assert_eq!(add(2, 3), 5);
    }
}

// Parameterized (with rstest):
use rstest::rstest;

#[rstest]
#[case(0, 0, 0)]
#[case(-1, 1, 0)]
fn add_parameterized(#[case] a: i32, #[case] b: i32, #[case] expected: i32) {
    assert_eq!(add(a, b), expected);
}
```

---

## 13. Mocking: Moq → mockall

**C#:**
```csharp
var mock = new Mock<IUserRepo>();
mock.Setup(r => r.GetById(42)).Returns(new User { Name = "Alice" });
var svc = new UserService(mock.Object);
```

**Rust:**
```rust
use mockall::{automock, predicate::eq};

#[automock]
trait UserRepo {
    fn get_by_id(&self, id: u64) -> Option<User>;
}

#[test]
fn returns_user() {
    let mut mock = MockUserRepo::new();
    mock.expect_get_by_id()
        .with(eq(42))
        .times(1)
        .returning(|_| Some(User { name: "Alice".into() }));

    let svc = UserService::new(Box::new(mock));
    assert_eq!(svc.get_user(42).unwrap().name, "Alice");
}
```

---

## 14. Shared Ownership / Circular References

**C#:** (GC handles cycles automatically)
```csharp
public class Node {
    public int Value;
    public Node? Parent;
    public List<Node> Children = new();
}
```

**Rust (Weak breaks cycle):**
```rust
use std::rc::{Rc, Weak};
use std::cell::RefCell;

struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}
```

**Rust (index-based — no cycles possible):**
```rust
struct Graph {
    nodes: Vec<NodeData>,
    edges: Vec<(usize, usize)>,
}
struct NodeData { value: String }
```

---

## 15. Interop: Rust Library Called from C#

**Rust (lib.rs):**
```rust
#[no_mangle]
pub extern "C" fn process_data(ptr: *const u8, len: usize) -> i32 {
    let data = unsafe { std::slice::from_raw_parts(ptr, len) };
    data.len() as i32
}
```

**Cargo.toml:**
```toml
[lib]
crate-type = ["cdylib"]

[build-dependencies]
csbindgen = "1.9"
```

**build.rs (auto-generates C# P/Invoke):**
```rust
fn main() {
    csbindgen::Builder::default()
        .input_extern_file("src/lib.rs")
        .csharp_dll_name("my_rust_lib")
        .generate_csharp_file("../dotnet/NativeMethods.g.cs")
        .unwrap();
}
```

**Generated C# (auto):**
```csharp
internal static unsafe partial class NativeMethods {
    [DllImport("my_rust_lib", CallingConvention = CallingConvention.Cdecl)]
    public static extern int process_data(byte* ptr, nuint len);
}
```
