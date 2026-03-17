# C# to Rust Migration Best Practices: Research Report

**Date:** 2025-03-17  
**Scope:** Incremental migration strategies, conversion phases, ordering, tooling, Paradox mod manager architecture

---

## Executive Summary

For large C# codebases (including Paradox game mod managers like Irony Mod Manager), **incremental module-by-module migration with FFI bridges** is the consensus approach. Use the **Strangler Fig pattern** (transform → coexist → eliminate) with **dependency-first ordering** informed by dependency graphs. Prefer **core infrastructure and leaf modules first**, then peripheral modules. Tools: **NDepend** for dependency analysis, **GitHub Copilot Modernization** for .NET upgrade assessment, **csbindgen** for FFI interop. Paradox mod managers follow a layered architecture: mod discovery → load-order resolution → conflict analysis → merge/patch generation.

---

## 1. Incremental C# to Rust Migration: Module-by-Module vs Full Rewrite

### Consensus

**Module-by-module (incremental) is strongly preferred for large codebases.** Full rewrite is recommended only for small projects where risk and downtime are acceptable.

### Incremental Approach (Recommended)

| Aspect | Recommendation |
|--------|----------------|
| **FFI Bridge** | Create Foreign Function Interface bridges between C# and Rust; keep both systems functional during transition |
| **Phased Rollout** | Replace modules one by one, prioritizing: good test coverage, non-critical paths, self-contained utilities |
| **Verification** | Run both versions with same inputs; compare outputs before full switchover |
| **Risk** | Lower risk; enables rollback; allows team learning curve |

**Sources:** Markaicode C-to-Rust migration guide (2025), convert-csharp-rust skill (SquiresWay)

### Full Rewrite Considerations

- **When:** Smaller projects, greenfield feasibility
- **Automated tools:** C2Rust, RustScript AI, Rustine—produce working but often non-idiomatic code; manual refactoring typically required
- **Risk:** Higher; potential downtime; no parallel verification

### Actionable Recommendation

> **Use incremental migration with FFI boundaries.** Start with leaf modules (few dependencies), add `extern "C" fn` bridges, call from C# via P/Invoke (e.g., csbindgen-generated bindings), validate parity, then retire C# modules.

---

## 2. Dividing a Large C# Codebase into Conversion Phases

### Strangler Fig Pattern (Primary Strategy)

The **Strangler Fig pattern** (Martin Fowler, AWS, Azure) is the standard for phased migration:

1. **Transform** – Create modernized components in parallel with the legacy application
2. **Coexist** – Use a façade/proxy layer to intercept and route traffic between old and new systems
3. **Eliminate** – Retire legacy functionality as traffic migrates to the new system

**Sources:** [AWS Prescriptive Guidance](https://docs.aws.amazon.com/prescriptive-guidance/latest/modernization-decomposing-monoliths/strangler-fig.html), [Martin Fowler](https://martinfowler.com/bliki/StranglerFigApplication.html), [Azure Architecture Center](https://learn.microsoft.com/en-us/azure/architecture/patterns/strangler-fig)

### Phase Structure

| Phase | Focus | Activities |
|-------|-------|------------|
| **Phase 1: Preparation** | Core infrastructure | Request routing, API/facade layer, monitoring, define "slices" (smallest migratable units) |
| **Phase 2: Core Modules** | Low-risk, high-value | Extract one module at a time; dual-run validation; maintain behavioral contracts |
| **Phase 3: Peripheral Modules** | Remaining functionality | Sequential migration; progressive traffic shifting; tight monitoring |

### Core vs Peripheral

- **Core:** Business-critical flows, shared utilities, cross-cutting concerns (DI, auth, logging)
- **Peripheral:** Feature-specific modules, UI layers, integrations
- **Order:** Start with low-risk core utilities, then core business logic, then peripheral features

### Actionable Recommendation

> **Define "slices"**—smallest independently migratable units. Map dependencies; identify cycles; break cycles before migration. Use a facade (e.g., Rust library called via FFI) to route calls during coexistence.

---

## 3. Dependency-First vs Feature-First Conversion Order

### Dependency-First (Bottom-Up) — Recommended

**Dependency-first** aligns with Strangler Fig and reduces risk:

- **Map dependencies first** using dependency graphs (NDepend, jdeps, or similar)
- **Migrate leaf modules** (few or no dependents) first—they have minimal impact on the rest of the system
- **Break cycles** before modularization; cyclic dependencies block clean extraction
- **Research support:** MigrationExp (ML-based migration order) outperforms ad-hoc strategies by predicting empirically observed migration orders

**Sources:** Understand Legacy Code (dependency graphs), Overcast (mapping legacy dependencies), academic migration order research

### Feature-First

- **When:** When a feature is self-contained and can be isolated
- **Risk:** May require extracting dependencies first anyway; can create temporary duplication

### Actionable Recommendation

> **Prefer dependency-first (bottom-up).** Build a dependency graph; identify leaf modules and cycles. Migrate in topological order: leaves first, then modules that depend only on migrated code. Use NDepend or similar for C# dependency visualization.

---

## 4. Tools and Methodologies for C# Codebase Analysis

### Primary Tools (2024–2026)

| Tool | Purpose | Tier |
|------|---------|------|
| **NDepend** | Dependency graphs, DSM, cycle detection, architecture diagrams, CQLinq rules | Tier 2 (commercial) |
| **GitHub Copilot Modernization** | AI-powered assessment, planning, execution; .NET Framework → modern .NET | Tier 1 (Microsoft) |
| **.NET Upgrade Assistant** | Codebase scan, upgrade challenges, severity categorization; *deprecated* but still useful for analysis | Tier 1 (Microsoft, deprecated) |

### NDepend Capabilities (Migration Planning)

- **Dependency Structure Matrix (DSM)** – Map project/namespace/class dependencies
- **Cycle detection** – Identify circular dependencies to break
- **Application Map** – Visual project dependency diagrams (PNG/SVG export)
- **Code metrics** – Cyclomatic complexity, coupling, LOC
- **CQLinq** – Custom architecture rules

**Source:** [NDepend](https://www.ndepend.com/features/dependency-graph-matrix-architecture)

### GitHub Copilot Modernization

- Three-stage workflow: assessment, planning, execution
- Analyzes projects and dependencies; generates upgrade docs
- Identifies breaking changes and API compatibility issues
- Supports .NET Framework → modern .NET, Azure migrations

**Source:** [Microsoft Learn](https://learn.microsoft.com/en-us/dotnet/core/porting/github-copilot-app-modernization/overview)

### FFI / Interop for Incremental Migration

| Tool | Purpose |
|------|---------|
| **csbindgen** | Auto-generates C# P/Invoke bindings from Rust `extern "C" fn`; Cdecl; .NET and Unity support |
| **csharpbindgen** | Alternative crate for C# bindings |

**Source:** [csbindgen GitHub](https://github.com/Cysharp/csbindgen), [crates.io](https://crates.io/crates/csbindgen)

### Actionable Recommendation

> **Workflow:** (1) Use NDepend or GitHub Copilot to assess dependencies and identify migration slices. (2) Map dependency graph; break cycles. (3) Use csbindgen in `build.rs` to generate C# bindings for Rust modules. (4) Migrate leaf modules first; validate with parity tests.

---

## 5. Paradox Game Mod Manager Architecture Patterns

### Irony Mod Manager (Reference Implementation)

Irony Mod Manager is a C# mod manager for Paradox games (Stellaris, HOI4, CK3, EU4, V3, etc.). It serves as the reference for SquiresWay (Rust mod loader).

### Architectural Layers

| Layer | Responsibility |
|-------|----------------|
| **Mod Discovery** | Scan game directories, parse mod descriptors |
| **Load Order** | Deterministic ordering based on game-specific rules (FIOS/LIOS) |
| **Conflict Analysis** | Game-dependent conflict detection (full for Stellaris, analysis mode for HOI4) |
| **Conflict Resolution** | Patch mod auto-generation, ignore rules, override rules, patch instructions |
| **Merge** | Multiple merge strategies; binary merge viewer for complex files |
| **Database / Search** | Searchable database view for conflict inspection |

### Common Patterns

- **Game-specific adapters** – Different engines (Clausewitz, Jomini) require different conflict/load-order logic
- **Deterministic ordering** – Reproducible mod load order for stability
- **Conflict hierarchy** – "Conflicted Objects" view with game logic hierarchy in dropdowns
- **External tool integration** – Merge tools, diff tools
- **Platform-specific rendering** – Windows, macOS, Linux with performance options

### Actionable Recommendation for SquiresWay

> **Mirror Irony's layered design:** (1) Mod discovery and descriptor parsing, (2) Load-order engine with game-specific rules, (3) Conflict analysis (start with one game, e.g., Stellaris), (4) Patch/merge generation. Port in dependency order: parsing and data models first, then load order, then conflict analysis.

---

## Summary: Actionable Checklist

1. **Strategy:** Incremental module-by-module with FFI; avoid full rewrite for large codebases.
2. **Phases:** Strangler Fig (transform → coexist → eliminate); core infrastructure → core modules → peripheral.
3. **Order:** Dependency-first (bottom-up); map graph, break cycles, migrate leaves first.
4. **Tools:** NDepend or GitHub Copilot for analysis; csbindgen for FFI; parity tests for validation.
5. **Paradox mod managers:** Layered architecture (discovery → load order → conflict analysis → merge); game-specific adapters; deterministic ordering.

---

## Sources

| # | Source | Tier | URL / Reference |
|---|--------|------|-----------------|
| 1 | Microsoft Rust for .NET Developers | Tier 1 | https://microsoft.github.io/rust-for-dotnet-devs/latest/ |
| 2 | AWS Strangler Fig Pattern | Tier 1 | https://docs.aws.amazon.com/prescriptive-guidance/latest/modernization-decomposing-monoliths/strangler-fig.html |
| 3 | Martin Fowler Strangler Fig | Tier 1 | https://martinfowler.com/bliki/StranglerFigApplication.html |
| 4 | Azure Strangler Fig | Tier 1 | https://learn.microsoft.com/en-us/azure/architecture/patterns/strangler-fig |
| 5 | GitHub Copilot Modernization | Tier 1 | https://learn.microsoft.com/en-us/dotnet/core/porting/github-copilot-app-modernization/overview |
| 6 | NDepend Dependency Analysis | Tier 2 | https://www.ndepend.com/features/dependency-graph-matrix-architecture |
| 7 | csbindgen | Tier 2 | https://github.com/Cysharp/csbindgen |
| 8 | Irony Mod Manager | Tier 2 | https://github.com/bcssov/IronyModManager, https://bcssov.github.io/IronyModManager/ |
| 9 | Markaicode C-to-Rust Migration 2025 | Tier 3 | Migration strategies (incremental vs rewrite) |
| 10 | Understand Legacy Code | Tier 2 | Dependency graphs for restructuring |
| 11 | Convert C# to Rust skill (SquiresWay) | Internal | Module-by-module, FFI, APTV workflow |
