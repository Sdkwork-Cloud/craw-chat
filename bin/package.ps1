param(
    [string]$PackageId,
    [string]$Version,
    [string]$StagingRoot,
    [string]$OutputDir,
    [switch]$All,
    [switch]$Stage,
    [switch]$Check,
    [switch]$DryRun,
    [switch]$Json,
    [switch]$Help
)

$ErrorActionPreference = "Stop"

if ($Help) {
    Write-Host "Usage: powershell -ExecutionPolicy Bypass -File bin/package.ps1 [-PackageId <id>] [-Version <value>] [-StagingRoot <dir>] [-OutputDir <dir>] [-All] [-Stage] [-Check] [-DryRun] [-Json]"
    Write-Host "Stage and/or package Sdkwork IM release archives. Use -Stage to stage production outputs before packaging."
    exit 0
}

$root = Split-Path -Parent $PSScriptRoot
Set-Location $root

if ($Stage) {
    $stageArgs = @("scripts/release/stage-sdkwork-im-release-package.mjs")
    if ($All) { $stageArgs += "--all" }
    if (-not [string]::IsNullOrWhiteSpace($PackageId)) { $stageArgs += @("--package-id", $PackageId) }
    if (-not [string]::IsNullOrWhiteSpace($Version)) { $stageArgs += @("--version", $Version) }
    if (-not [string]::IsNullOrWhiteSpace($StagingRoot)) { $stageArgs += @("--staging-root", $StagingRoot) }
    if ($Check) { $stageArgs += "--check" }
    if ($DryRun) { $stageArgs += "--dry-run" }
    if ($Json) { $stageArgs += "--json" }
    & node @stageArgs
    if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }
}

$packageArgs = @("scripts/release/build-sdkwork-im-install-package.mjs")
if ($All) { $packageArgs += "--all" }
if (-not [string]::IsNullOrWhiteSpace($PackageId)) { $packageArgs += @("--package-id", $PackageId) }
if (-not [string]::IsNullOrWhiteSpace($Version)) { $packageArgs += @("--version", $Version) }
if (-not [string]::IsNullOrWhiteSpace($StagingRoot)) { $packageArgs += @("--staging-root", $StagingRoot) }
if (-not [string]::IsNullOrWhiteSpace($OutputDir)) { $packageArgs += @("--output-dir", $OutputDir) }
if ($Check) { $packageArgs += "--check" }
if ($DryRun) { $packageArgs += "--dry-run" }
if ($Json) { $packageArgs += "--json" }

& node @packageArgs
exit $LASTEXITCODE
