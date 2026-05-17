# Code Architecture Guide

This document defines how to structure the Mentem API codebase to remain **modular, testable, and easily migrable to a workspace** as the project grows.

**Core Principle:** All code is organized for eventual crate separation. If you follow these rules, migrating from lib → workspace crates requires only moving files and adjusting visibility, not refactoring logic.

---

## Project Structure

```
src/
├── lib.rs                 ← Library entry point (public API)
├── main.rs                ← Binary entry point (server bootstrap only)
├── config.rs              ← Shared configuration (env loading)
├── state.rs               ← AppState definition
├── error.rs               ← Shared error types (domain errors)
├── modules/               ← Feature modules (domain logic)
│   ├── mod.rs
│   ├── {feature}/
│   │   ├── mod.rs         ← Module entry point
│   │   ├── domain.rs      ← Data models and types
│   │   ├── ports.rs       ← Trait definitions (interfaces)
│   │   ├── service.rs     ← Business logic (uses ports)
│   │   ├── adapters/      ← Infrastructure implementations
│   │   │   ├── mod.rs
│   │   │   ├── repository.rs     (example: database impl)
│   │   │   ├── external.rs       (example: external service impl)
│   │   │   └── cache.rs          (example: cache impl)
│   │   └── http/          ← HTTP layer (private to module)
│   │       ├── mod.rs
│   │       ├── handler.rs        (HTTP handlers)
│   │       └── extractor.rs      (custom extractors)
│   └── {other-features}/
│       └── mod.rs
└── shared/                ← Code used by multiple modules
    ├── mod.rs
    └── utils.rs
```

Replace `{feature}` with your actual feature names (e.g., `auth`, `users`, `posts`, `notifications`, etc.)

---

## Core Concepts

### 1. **Hexagonal Architecture (Ports & Adapters)**

Domain logic never depends on infrastructure. Instead, it depends on **traits (ports)** that infrastructure implements (adapters).

```
Domain (Service)
    ↓ depends on
Ports (Traits)
    ↑ implemented by
Adapters (Concrete implementations)
    ↓ use
Infrastructure (Database, external APIs, caches)
```

**Why?** 
- Services are testable without hitting real infrastructure
- Easy to swap implementations (mock, real, etc.)
- When you split into crates, services move unchanged; adapters move separately

---

### 2. **Layering Within a Module**

Each feature (auth, users, posts) follows this layer:

| Layer | Location | Responsibility | Example |
|-------|----------|---|---|
| **Domain** | `domain.rs` | Data models, value objects | `struct User`, `struct Email` |
| **Ports** | `ports.rs` | Trait definitions (what the domain needs) | `trait Repository`, `trait ExternalService` |
| **Service** | `service.rs` | Business logic (uses ports, not concrete types) | `FeatureService::create()` |
| **Adapters** | `adapters/*.rs` | Implementations of ports | `PgRepository`, `HttpExternalService` |
| **HTTP** | `http/*.rs` | Request/response mapping (private) | `POST /users` → `FeatureService::create()` |

---

## Rules for Migration Readiness

### Rule 1: **Services Use Ports, Never Concrete Types**

**GOOD** — Service depends on trait (port):
```rust
// src/modules/{feature}/service.rs
pub struct FeatureService {
    repository: Arc<dyn Repository>,     // Trait, not concrete
    external_api: Arc<dyn ExternalAPI>,
    cache: Arc<dyn Cache>,
}

impl FeatureService {
    pub async fn perform_action(&self, input: String) -> Result<Output, Error> {
        // Calls trait methods, doesn't know about implementation
        let cached = self.cache.get(&input)?;
        if let Some(result) = cached {
            return Ok(result);
        }
        
        let data = self.repository.fetch(&input).await?;
        let result = self.external_api.process(data).await?;
        self.cache.set(&input, &result).await?;
        Ok(result)
    }
}
```

**BAD** — Service depends on concrete adapter:
```rust
// DON'T DO THIS
pub struct FeatureService {
    repository: PgRepository,  // ❌ Concrete type
}
// Now you can't swap implementations, can't test easily
```

### Rule 2: **Ports Live in `ports.rs`, Adapters in `adapters/`**

Ports define what the domain needs. Adapters implement them. This physical separation makes migration trivial.

```rust
// src/modules/{feature}/ports.rs — DEFINES THE CONTRACT
#[async_trait]
pub trait Repository: Send + Sync {
    async fn create(&self, input: CreateInput) -> Result<Entity, Error>;
    async fn fetch(&self, id: Id) -> Result<Option<Entity>, Error>;
    async fn update(&self, id: Id, input: UpdateInput) -> Result<Entity, Error>;
    async fn delete(&self, id: Id) -> Result<(), Error>;
}

#[async_trait]
pub trait ExternalAPI: Send + Sync {
    async fn process(&self, data: Data) -> Result<ProcessedData, Error>;
    async fn verify(&self, input: VerifyInput) -> Result<bool, Error>;
}

#[async_trait]
pub trait Cache: Send + Sync {
    async fn get(&self, key: &str) -> Result<Option<CachedValue>, Error>;
    async fn set(&self, key: &str, value: &CachedValue) -> Result<(), Error>;
}

// src/modules/{feature}/adapters/repository.rs — IMPLEMENTS IT
pub struct PgRepository {
    pool: PgPool,
}

#[async_trait]
impl Repository for PgRepository {
    async fn create(&self, input: CreateInput) -> Result<Entity, Error> {
        // DB logic here
        todo!()
    }
}
```

**Why?** When you split into crates:
- `core` crate defines ports (no dependencies on infrastructure)
- `infrastructure` crate implements adapters (depends on `core`)
- `api` crate wires them together

### Rule 3: **Module Exports (pub) Define the Public API**

Each module's `mod.rs` declares what's public. Everything else stays private.

```rust
// src/modules/{feature}/mod.rs
pub use domain::{Entity, DomainError};
pub use service::FeatureService;

// Ports, adapters, HTTP handlers stay PRIVATE
mod domain;
mod ports;
mod service;
mod adapters;
mod http;
```

This means:
- Other modules can use `FeatureService` and `Entity`
- Other modules **cannot** see `Repository` (infrastructure detail)
- Other modules **cannot** see HTTP handlers (server detail)

**Why?** Clear boundary between "API" (what external code uses) and "implementation" (how it works).

### Rule 4: **No Cross-Module Private Access**

If module A needs something from module B, it must be `pub` in B's `mod.rs`.

**GOOD:**
```rust
// src/modules/feature_a/mod.rs
pub use domain::Entity;  // ✅ Exported, other modules can use it
use service::Service;     // ❌ Private, only this module uses it

// src/modules/feature_b/service.rs
use crate::modules::feature_a::Entity;  // ✅ OK, it's pub
```

**BAD:**
```rust
// src/modules/feature_a/mod.rs
use domain::Entity;  // ❌ Private, not exported

// src/modules/feature_b/service.rs
use crate::modules::feature_a::domain::Entity;  // ❌ Can't access private modules
```

**Why?** Defines what each module exposes. When you split to crates, crate visibility replaces module visibility automatically.

### Rule 5: **Adapters Receive Concrete Types, Services Receive Traits**

Only the "wiring" layer knows about concrete implementations.

```rust
// src/modules/{feature}/adapters/mod.rs — WIRING LAYER
use crate::modules::{feature}::{FeatureService, Repository};

pub fn build_service(pool: PgPool) -> FeatureService {
    FeatureService::new(
        Arc::new(PgRepository::new(pool)),     // ← Concrete here
        Arc::new(HttpExternalAPI::new()),
        Arc::new(InMemoryCache::new()),
    )
}

// src/init/mod.rs — ALSO WIRING
fn build_services(pool: PgPool) -> AppState {
    AppState {
        feature: adapters::build_service(pool),
    }
}
```

**Why?** All concrete knowledge is in one place. When you split to crates, wiring moves to the `api` crate's main function.

### Rule 6: **HTTP Handlers Are Private to Modules**

Handlers translate HTTP ↔ domain. They're not reusable, so keep them private.

```rust
// src/modules/{feature}/http/handler.rs — PRIVATE FILE
async fn create(
    State(service): State<Arc<FeatureService>>,
    Json(payload): Json<CreateRequest>,
) -> Result<Json<Entity>, Error> {
    let entity = service.perform_action(payload.input).await?;
    Ok(Json(entity))
}

// src/modules/{feature}/http/mod.rs — PRIVATE MODULE
pub fn routes() -> Router {
    Router::new()
        .route("/create", post(handler::create))
        .route("/list", get(handler::list))
}

// src/modules/{feature}/mod.rs
pub use domain::Entity;
pub use service::FeatureService;

mod http;  // ❌ PRIVATE, not exported
```

**Why?** HTTP is a delivery mechanism, not API. Other modules don't need to know about HTTP handlers.

---

## Code Organization Patterns

### Pattern 1: Service Layer

```rust
// src/modules/{feature}/service.rs
pub struct FeatureService {
    repository: Arc<dyn Repository>,
    external: Arc<dyn ExternalAPI>,
    cache: Arc<dyn Cache>,
}

impl FeatureService {
    pub fn new(
        repository: Arc<dyn Repository>,
        external: Arc<dyn ExternalAPI>,
        cache: Arc<dyn Cache>,
    ) -> Self {
        Self {
            repository,
            external,
            cache,
        }
    }

    pub async fn perform_action(&self, input: String) -> Result<Output, Error> {
        // Business logic: validate, process, cache, return
        let data = self.repository.fetch(&input).await?;
        let result = self.external.process(data).await?;
        self.cache.set(&input, &result).await?;
        Ok(result)
    }
}
```

**Key points:**
- Takes dependencies as traits (Arc<dyn Trait>)
- No knowledge of HTTP, database, or external services
- Can be tested with mock implementations
- Migrates to separate crate unchanged

### Pattern 2: Port Definition

```rust
// src/modules/{feature}/ports.rs
use async_trait::async_trait;

#[async_trait]
pub trait Repository: Send + Sync {
    async fn create(&self, input: CreateInput) -> Result<Entity, Error>;
    async fn fetch(&self, id: Id) -> Result<Option<Entity>, Error>;
    async fn update(&self, id: Id, input: UpdateInput) -> Result<Entity, Error>;
    async fn delete(&self, id: Id) -> Result<(), Error>;
}

#[async_trait]
pub trait ExternalAPI: Send + Sync {
    async fn process(&self, data: Data) -> Result<ProcessedData, Error>;
    async fn validate(&self, input: ValidateInput) -> Result<bool, Error>;
}

#[async_trait]
pub trait Cache: Send + Sync {
    async fn get(&self, key: &str) -> Result<Option<CachedValue>, Error>;
    async fn set(&self, key: &str, value: &CachedValue) -> Result<(), Error>;
    async fn invalidate(&self, key: &str) -> Result<(), Error>;
}
```

**Key points:**
- Defines what the domain needs (contracts)
- No implementation details
- Can live in `core` crate when you split
- Infrastructure crates implement these

### Pattern 3: Adapter Implementation

```rust
// src/modules/{feature}/adapters/repository.rs
use sqlx::PgPool;
use crate::modules::{feature}::{domain::Entity, ports::Repository};

pub struct PgRepository {
    pool: PgPool,
}

impl PgRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl Repository for PgRepository {
    async fn create(&self, input: CreateInput) -> Result<Entity, Error> {
        let entity = sqlx::query_as::<_, Entity>(
            "INSERT INTO table (col1, col2) VALUES ($1, $2) RETURNING *"
        )
        .bind(input.field1)
        .bind(input.field2)
        .fetch_one(&self.pool)
        .await?;

        Ok(entity)
    }
    
    // ... other methods
}
```

**Key points:**
- Implements the port (trait)
- Knows about concrete infrastructure (database, HTTP, etc.)
- Stays private to the module
- When you split to crates, moves to `infrastructure` crate

### Pattern 4: HTTP Handler

```rust
// src/modules/{feature}/http/handler.rs — PRIVATE
use axum::{extract::State, Json};

#[derive(serde::Deserialize)]
pub struct CreateRequest {
    field1: String,
    field2: String,
}

pub async fn create(
    State(service): State<Arc<FeatureService>>,
    Json(payload): Json<CreateRequest>,
) -> Result<Json<Entity>, Error> {
    let entity = service.perform_action(payload.field1).await?;
    Ok(Json(entity))
}

pub async fn list(
    State(service): State<Arc<FeatureService>>,
) -> Result<Json<Vec<Entity>>, Error> {
    let entities = service.list_all().await?;
    Ok(Json(entities))
}
```

**Key points:**
- Maps HTTP types → domain types
- Calls service (which uses ports)
- Private to the module (not in `pub use` in mod.rs)

### Pattern 5: Module Wiring

```rust
// src/modules/{feature}/adapters/mod.rs
use std::sync::Arc;
use sqlx::PgPool;
use crate::modules::{feature}::{FeatureService, adapters::repository::PgRepository};

pub fn build_service(pool: PgPool) -> FeatureService {
    FeatureService::new(
        Arc::new(PgRepository::new(pool)),
        Arc::new(HttpExternalAPI::new()),
        Arc::new(InMemoryCache::new()),
    )
}
```

**Key points:**
- Knows about concrete types (only here!)
- Builds the service with implementations
- Called from `src/init/mod.rs`
- When you split to crates, moves to the `api` crate's main function

---

## What Gets Exported from Each Module

Use this as a template for `mod.rs`:

```rust
// src/modules/{feature}/mod.rs
pub mod domain;
pub mod service;

// Re-export what external code needs
pub use domain::{
    Entity,
    EntityError,
    CreateInput,
    UpdateInput,
};
pub use service::FeatureService;

// Keep private
mod ports;
mod adapters;
mod http;
```

**Rule:** Exports are "API". Everything else is "implementation detail".

---

## Dependency Direction (One-Way Only)

```
main.rs / init/
    ↓
modules/{feature}/service
    ↓
modules/{feature}/ports (traits)
    ↑
modules/{feature}/adapters (implementations)
    ↓
external libs (sqlx, http clients, etc.)
```

**Never:** Service → concrete adapter. Always service → trait.

---

## Testing Strategy (Enabled by Ports)

Because services depend on traits, testing is straightforward:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use async_trait::async_trait;

    struct MockRepository {
        entities: std::sync::Mutex<Vec<Entity>>,
    }

    #[async_trait]
    impl Repository for MockRepository {
        async fn create(&self, input: CreateInput) -> Result<Entity, Error> {
            let entity = Entity { /* ... */ };
            self.entities.lock().unwrap().push(entity.clone());
            Ok(entity)
        }
        // ... other methods
    }

    #[tokio::test]
    async fn test_perform_action() {
        let mock_repo = Arc::new(MockRepository { entities: Default::default() });
        let mock_external = Arc::new(MockExternalAPI);
        let mock_cache = Arc::new(MockCache);

        let service = FeatureService::new(mock_repo, mock_external, mock_cache);
        let result = service.perform_action("input".to_string()).await.unwrap();

        assert_eq!(result.field, "expected_value");
    }
}
```

---

## Anti-Patterns (DON'T DO THIS)

### ❌ Anti-Pattern 1: Service Uses Concrete Types

```rust
// DON'T
pub struct FeatureService {
    repository: PgRepository,  // Concrete, not trait!
}
```

**Why bad:** Can't test without DB, can't swap implementations, hard to migrate.

### ❌ Anti-Pattern 2: HTTP Handlers Call Database Directly

```rust
// DON'T
pub async fn create(
    State(pool): State<PgPool>,
    Json(payload): Json<CreateRequest>,
) -> Json<Entity> {
    let entity = sqlx::query_as::<_, Entity>("INSERT ...")
        .fetch_one(&pool)
        .await;
    Json(entity)
}
```

**Why bad:** Business logic in HTTP layer, hard to test, can't reuse.

### ❌ Anti-Pattern 3: Cross-Module Private Access

```rust
// DON'T
use crate::modules::feature::adapters::PgRepository;
```

**Why bad:** Breaks encapsulation, prevents crate splitting.

### ❌ Anti-Pattern 4: Circular Module Dependencies

```rust
// DON'T
mod feature_a {
    use crate::feature_b::SomeType;
}
mod feature_b {
    use crate::feature_a::OtherType;  // Circular!
}
```

**Why bad:** Makes it impossible to split into separate crates.

---

## Migration Checklist (Lib → Workspace)

When you're ready to split into workspace crates, this checklist ensures code migrates cleanly:

- [ ] All services depend on traits, not concrete types
- [ ] Ports (traits) are in `ports.rs`, adapters in `adapters/`
- [ ] Adapters are private to modules (not in pub exports)
- [ ] HTTP handlers are private to modules
- [ ] No circular dependencies between modules
- [ ] Module `mod.rs` has clear public API (pub use exports)
- [ ] No private module access across feature boundaries

If all boxes check, migration is mechanical:
1. Create `core` crate, move domains + ports
2. Create `infrastructure` crate, move adapters
3. Create `api` crate, move main + http + wiring
4. Adjust visibility (module → crate)

---

## Summary

**Write code as if each feature is already a separate crate.** Follow these rules and you'll:
- Keep business logic testable
- Enforce clear boundaries
- Make workspace migration painless
- Scale from 10k to 100k+ LOC without refactoring
