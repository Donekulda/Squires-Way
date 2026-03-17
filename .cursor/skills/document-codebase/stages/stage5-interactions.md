# Stage 5: Interaction Mapping

Map how components interact across folders. Generate dependency and flow diagrams.

## Checklist

- [ ] Read combined/ and detailed/ docs
- [ ] Analyze source code for cross-folder imports, references, dependencies
- [ ] Identify main data flows and call patterns
- [ ] Write architecture/interactions.md (narrative)
- [ ] Write architecture/dependencies.md (Mermaid diagrams)
- [ ] Create architecture/toc.yml

## interactions.md Format

```markdown
# Component Interactions

## Overview
How the major modules interact at a high level.

## Key Relationships
- **ModuleA** → **ModuleB**: Description of the relationship (e.g., A calls B for X)
- **ModuleB** → **ModuleC**: ...

## Data Flow
Describe main data paths (e.g., user input → validation → persistence).

## Entry Points
List main entry points (e.g., Program.cs, Startup, controllers).
```

## dependencies.md Format (Mermaid)

```markdown
# Dependency Diagrams

## Module Dependencies

\`\`\`mermaid
flowchart LR
    A[FolderA] --> B[FolderB]
    B --> C[FolderC]
    A --> C
\`\`\`

## Call Flow (Example)

\`\`\`mermaid
sequenceDiagram
    participant UI
    participant Service
    participant Data
    UI->>Service: Request
    Service->>Data: Query
    Data-->>Service: Result
    Service-->>UI: Response
\`\`\`
```

## Analysis Guidelines

- Use namespace/using statements, project references, and imports
- For C#: check .csproj references, using directives
- Keep diagrams readable; group related nodes
- Prefer high-level flows over exhaustive detail
