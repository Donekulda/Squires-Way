# IronyModManager Setup

IronyModManager is the reference codebase for the document-codebase workflow. To use it, clone the repository into the Examples folder.

## Prerequisites

- Git

## Setup Steps

### 1. Create Examples directory

```bash
mkdir -p Examples
```

### 2. Clone IronyModManager

```bash
git clone https://github.com/bcssov/IronyModManager.git Examples/IronyModManager
```

### 3. Verify structure

After cloning, you should have:

```
Examples/
└── IronyModManager/
    ├── src/          # Source to document
    ├── ...           # Other project files
```

### 4. Run documentation workflow

Invoke the skill:

```
/document-codebase Examples/IronyModManager/src Examples/IronyModManager/docs
```

Or with default output path:

```
/document-codebase Examples/IronyModManager/src
```

Output will be written to `Examples/IronyModManager/docs/`.

## Using Setup Scripts

**Bash** (from workspace root):
```bash
.cursor/skills/document-codebase/scripts/setup-irony.sh
```

**PowerShell** (from workspace root):
```powershell
.\.cursor\skills\document-codebase\scripts\setup-irony.ps1
```

## Windows (PowerShell, manual)

```powershell
New-Item -ItemType Directory -Force -Path Examples
git clone https://github.com/bcssov/IronyModManager.git Examples/IronyModManager
```

## Notes

- IronyModManager is a C# Paradox game mod manager
- SquiresWay (this project) is a Rust mod loader based on IronyModManager
- The workflow supports any source folder; IronyModManager is used as the primary example
