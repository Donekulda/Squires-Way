# Stage 3: Detailed

Document each subfolder with key files, classes, and responsibilities.

## Checklist

- [ ] Read overview/folder-purposes.md
- [ ] For each top-level folder: dispatch one subagent (or process sequentially)
- [ ] For each subfolder: list key files, classes/types, responsibilities
- [ ] Write detailed/[FolderName]/README.md per top-level folder
- [ ] Add detailed/toc.yml with links to each folder README

## Output Format (per detailed/[FolderName]/README.md)

```markdown
# FolderName

## Overview
One paragraph summarizing this folder's role.

## SubfolderName
- **Purpose**: ...
- **Key files**: file1.cs, file2.cs
- **Key types**: ClassName, InterfaceName
- **Responsibilities**: ...

## AnotherSubfolder
...
```

## Subagent Pattern

When using subagents (one per top-level folder):
- Provide full path to source folder
- Provide overview/folder-purposes.md content for context
- Ask subagent to produce detailed/[FolderName]/README.md
- Run subagents in parallel only if folders are independent
