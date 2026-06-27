param(
    [switch]$Postgres,
    [switch]$Desktop,
    [switch]$Help
)

$ErrorActionPreference = "Stop"

if ($Help) {
    Write-Host "Usage: powershell -ExecutionPolicy Bypass -File bin/dev.ps1 [-Postgres] [-Desktop]"
    Write-Host "Start Sdkwork IM development mode. Browser is default; -Desktop starts the Tauri desktop dev flow; -Postgres loads .env.postgres."
    exit 0
}

$root = Split-Path -Parent $PSScriptRoot
Set-Location $root
$pnpm = if ($IsWindows) { "pnpm.cmd" } else { "pnpm" }
$scriptName = if ($Desktop) {
    if ($Postgres) { "tauri:dev:postgres" } else { "tauri:dev" }
} else {
    if ($Postgres) { "dev:postgres" } else { "dev" }
}

& $pnpm $scriptName
exit $LASTEXITCODE
