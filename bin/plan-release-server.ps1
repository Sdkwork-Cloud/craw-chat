param(
    [string]$ReleaseGatePath = "",
    [ValidateSet("all", "linux", "macos", "windows")]
    [string]$Platform = "all",
    [ValidateSet("text", "json")]
    [string]$OutputFormat = "text",
    [switch]$Help
)

$ErrorActionPreference = "Stop"

if ($Help) {
    Write-Host "Usage: powershell -ExecutionPolicy Bypass -File bin/plan-release-server.ps1 [-ReleaseGatePath <path-to-release-gate.json>] [-Platform <all|linux|macos|windows>] [-OutputFormat <text|json>]"
    Write-Host "Summarize the sdkwork-im-server release plan from the machine-readable release-gate, package-catalog, and release-execution contracts."
    Write-Host "The emitted plan keeps checksum and artifact-file-list contract pointers visible for operators and automation."
    exit 0
}

if ([string]::IsNullOrWhiteSpace($ReleaseGatePath)) {
    throw "ReleaseGatePath is required. Point it at artifacts/releases/<bundle-id>/server/release-gate.json."
}

$helperPath = Join-Path $PSScriptRoot "plan-release-server-contracts.mjs"
& node $helperPath --release-gate-path $ReleaseGatePath --platform $Platform --format $OutputFormat
if ($LASTEXITCODE -ne 0) {
    throw "plan-release-server-contracts.mjs failed for gate path: $ReleaseGatePath"
}
