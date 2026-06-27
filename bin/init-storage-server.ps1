param(
    [string]$InstanceName = "default",
    [string]$ConfigDir = ([System.IO.Path]::Combine([Environment]::GetFolderPath("CommonApplicationData"), "sdkwork", "chat")),
    [ValidateSet("verify-only", "bootstrap-schema", "create-db-and-schema")]
    [string]$Mode = "verify-only",
    [ValidateSet("text", "json")]
    [string]$OutputFormat = "text",
    [switch]$Help
)

$ErrorActionPreference = "Stop"

function Get-ServerConfigDirForInstance {
    param([string]$Name)

    $root = [System.IO.Path]::Combine([Environment]::GetFolderPath("CommonApplicationData"), "sdkwork", "chat")
    if ($Name -eq "default") {
        return $root
    }
    return [System.IO.Path]::Combine($root, "instances", $Name)
}

if ($PSBoundParameters.ContainsKey("InstanceName") -and -not $PSBoundParameters.ContainsKey("ConfigDir")) {
    $ConfigDir = Get-ServerConfigDirForInstance $InstanceName
}

if ($Help) {
    Write-Host "Usage: powershell -ExecutionPolicy Bypass -File bin/init-storage-server.ps1 [-InstanceName <name>] [-ConfigDir <path>] [-Mode <verify-only|bootstrap-schema|create-db-and-schema>] [-OutputFormat <text|json>]"
    Write-Host "Validate the file-based postgresql storage contract, summarize the selected storage mode, and write a storage report."
    exit 0
}

$postgresqlPath = Join-Path $ConfigDir "postgresql.yaml"
$reportPath = Join-Path $ConfigDir "storage-init-report.json"
$missing = New-Object System.Collections.Generic.List[string]

if (-not (Test-Path $postgresqlPath)) {
    $missing.Add("postgresql.yaml")
    $postgresqlContent = ""
}
else {
    $postgresqlContent = Get-Content -Path $postgresqlPath -Raw
}

foreach ($contract in @("provider: postgresql", "connection:", "database:", "username:", "passwordFile:", "migrationMode:")) {
    if (-not $postgresqlContent.Contains($contract)) {
        $missing.Add($contract)
    }
}

$migrationMode = $null
if ($postgresqlContent -match 'migrationMode:\s*([^\r\n]+)') {
    $migrationMode = $Matches[1].Trim()
}

$report = [ordered]@{
    product = "sdkwork-im-server"
    instance = $InstanceName
    mode = $Mode
    storage = "postgresql"
    configPath = $postgresqlPath
    report = $reportPath
    configValid = ($missing.Count -eq 0)
    missing = @($missing)
    migrationMode = $migrationMode
    ready = ($missing.Count -eq 0)
    note = "First landing validates the file-based PostgreSQL contract and writes a truthful report. Live database connectivity checks are the next stage."
}

$json = $report | ConvertTo-Json -Depth 5
$json | Set-Content -Path $reportPath -Encoding utf8

if ($OutputFormat -eq "json") {
    Write-Output $json
}
else {
    Write-Host "sdkwork-im-server storage report"
    Write-Host "mode: $Mode"
    Write-Host "storage: postgresql"
    Write-Host "configValid: $($report.configValid)"
    Write-Host "ready: $($report.ready)"
    Write-Host "report: $reportPath"
    if ($missing.Count -gt 0) {
        Write-Host "missing: $($missing -join ', ')"
    }
}
