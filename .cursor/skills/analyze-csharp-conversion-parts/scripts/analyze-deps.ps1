# Parse .csproj references to build dependency graph for C# to Rust conversion planning.
# Usage: .\analyze-deps.ps1 -SourcePath "Examples/IronyModManager/src"
# Output: Writes dependency info to stdout; can be piped to file.

param(
    [Parameter(Mandatory = $true)]
    [string]$SourcePath
)

$ErrorActionPreference = "Stop"

if (-not (Test-Path $SourcePath)) {
    Write-Error "Source path not found: $SourcePath"
    exit 1
}

$csprojFiles = Get-ChildItem -Path $SourcePath -Recurse -Filter "*.csproj" -File
$projectRefs = @{}

foreach ($csproj in $csprojFiles) {
    $projectName = [System.IO.Path]::GetFileNameWithoutExtension($csproj.Name)
    $content = Get-Content $csproj.FullName -Raw

    # Match ProjectReference Include="..\ProjectName\ProjectName.csproj" or similar
    $refs = [regex]::Matches($content, 'ProjectReference\s+Include="[^"]*\\([^\\]+)\.csproj"') |
        ForEach-Object { $_.Groups[1].Value }

    $projectRefs[$projectName] = @($refs)
}

# Output as simple dependency list (project -> depends on)
Write-Output "# C# Project Dependencies (parsed from .csproj)"
Write-Output ""
foreach ($proj in ($projectRefs.Keys | Sort-Object)) {
    $deps = $projectRefs[$proj]
    if ($deps.Count -gt 0) {
        Write-Output "$proj -> $($deps -join ', ')"
    } else {
        Write-Output "$proj -> (none)"
    }
}
