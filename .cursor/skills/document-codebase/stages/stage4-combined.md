# Stage 4: Combined

Aggregate subfolder documentation into parent-level summaries for complex folders.

## Checklist

- [ ] Read all detailed/[FolderName]/README.md files
- [ ] Identify folders with 2+ subfolders (complex folders)
- [ ] For each complex folder: create combined/[FolderName]/README.md
- [ ] Aggregate subfolder docs into a coherent summary
- [ ] Update toc.yml in combined/

## Output Format (per combined/[FolderName]/README.md)

```markdown
# FolderName - Combined Summary

## Overview
High-level summary of the entire folder and how subfolders work together.

## Subfolder Summaries

### Subfolder1
Brief summary + link to detailed/[FolderName]/README.md#subfolder1

### Subfolder2
Brief summary + link

## Cross-cutting Concerns
(If applicable) Shared patterns, dependencies between subfolders.
```

## When to Create Combined Docs

- Only for folders that have 2+ subfolders
- Skip leaf folders (no subfolders)
- Focus on folders that represent major modules or subsystems
