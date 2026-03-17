# Clone IronyModManager into Examples/IronyModManager for documentation workflow

$ErrorActionPreference = "Stop"
$scriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$workspaceRoot = Resolve-Path (Join-Path $scriptDir "..\..\..\..")

New-Item -ItemType Directory -Force -Path (Join-Path $workspaceRoot "Examples") | Out-Null
Set-Location $workspaceRoot

if (Test-Path "Examples\IronyModManager") {
    Write-Host "Examples/IronyModManager already exists. Pulling latest..."
    git -C Examples/IronyModManager pull
} else {
    git clone https://github.com/bcssov/IronyModManager.git Examples/IronyModManager
}

Write-Host "Done. Run: /document-codebase Examples/IronyModManager/src Examples/IronyModManager/docs"
