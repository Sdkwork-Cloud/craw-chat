param(
    [ValidateSet("server", "desktop", "all")]
    [string]$Target = "all",
    [string]$TargetTriple,
    [string]$Platform,
    [string]$Arch,
    [switch]$DryRun,
    [switch]$Json,
    [switch]$Help
)

$ErrorActionPreference = "Stop"

if ($Help) {
    Write-Host "Usage: powershell -ExecutionPolicy Bypass -File bin/build.ps1 [-Target server|desktop|all] [-TargetTriple <triple>] [-Platform <platform>] [-Arch <arch>] [-DryRun] [-Json]"
    Write-Host "Build Sdkwork IM production server binary/web assets and/or desktop installer artifacts."
    exit 0
}

$root = Split-Path -Parent $PSScriptRoot
Set-Location $root
$argsList = @("scripts/release/build-sdkwork-im-production.mjs", "--target", $Target)
if (-not [string]::IsNullOrWhiteSpace($TargetTriple)) { $argsList += @("--target-triple", $TargetTriple) }
if (-not [string]::IsNullOrWhiteSpace($Platform)) { $argsList += @("--platform", $Platform) }
if (-not [string]::IsNullOrWhiteSpace($Arch)) { $argsList += @("--arch", $Arch) }
if ($DryRun) { $argsList += "--dry-run" }
if ($Json) { $argsList += "--json" }

& node @argsList
exit $LASTEXITCODE
