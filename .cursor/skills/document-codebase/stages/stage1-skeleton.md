# Stage 1: Skeleton

Create the documentation folder structure and initial artifacts.

## Checklist

- [ ] List directory tree of source folder (exclude: bin, obj, .git, *.dll, node_modules)
- [ ] Create docs/ with subdirectories: skeleton/, overview/, detailed/, combined/, architecture/
- [ ] Write skeleton/folder-tree.md mirroring source structure (Markdown code block or bullet list)
- [ ] Create root toc.yml with entries for skeleton, overview, detailed, combined, architecture
- [ ] Create index.md with project name and link to skeleton/folder-tree.md

## Output Structure

```
docs/
├── index.md
├── toc.yml
└── skeleton/
    └── folder-tree.md
```

## toc.yml Template

```yaml
- name: Overview
  href: index.md
- name: Skeleton
  href: skeleton/
  items:
    - name: Folder Tree
      href: skeleton/folder-tree.md
- name: Overview
  href: overview/
- name: Detailed
  href: detailed/
- name: Combined
  href: combined/
- name: Architecture
  href: architecture/
```

## folder-tree.md Format

Use a Markdown code block or bullet list:

```markdown
# Source Folder Structure

\`\`\`
src/
├── FolderA/
│   ├── Subfolder1/
│   └── Subfolder2/
├── FolderB/
└── ...
\`\`\`
```
