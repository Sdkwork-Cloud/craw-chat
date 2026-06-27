param(
    [string]$InstanceName = "default",
    [string]$ConfigDir = ([System.IO.Path]::Combine([Environment]::GetFolderPath("CommonApplicationData"), "sdkwork", "chat")),
    [string]$ReleaseGatePath = "",
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
    Write-Host "Usage: powershell -ExecutionPolicy Bypass -File bin/verify-server.ps1 [-InstanceName <name>] [-ConfigDir <path>] [-ReleaseGatePath <path-to-release-gate.json>] [-OutputFormat <text|json>]"
    Write-Host "Validate config, storage wiring, and ready state for sdkwork-im-server, and optionally audit the machine-readable release-gate bundle for semantic contract drift, decisionStatus, and semanticIssues."
    exit 0
}

function Get-ReleaseContractReport {
    param(
        [Parameter(Mandatory = $true)]
        [string]$GatePath
    )

    $verifierPath = Join-Path $PSScriptRoot "verify-server-release-contracts.mjs"
    $json = & node $verifierPath --release-gate-path $GatePath --format json
    if ($LASTEXITCODE -ne 0) {
        throw "verify-server-release-contracts.mjs failed for gate path: $GatePath"
    }

    return $json | ConvertFrom-Json
}

function Get-FirstExistingPath {
    param([string[]]$Paths)

    foreach ($path in $Paths) {
        if (Test-Path $path) {
            return $path
        }
    }

    return $null
}

function Get-YamlScalarValue {
    param(
        [string]$Content,
        [string]$Key
    )

    foreach ($line in ($Content -split "\r?\n")) {
        $trimmed = $line.Trim()
        if ($trimmed.StartsWith("#")) {
            continue
        }
        $prefix = "$Key`:"
        if ($trimmed.StartsWith($prefix)) {
            return $trimmed.Substring($prefix.Length).Trim().Trim('"').Trim("'")
        }
    }

    return $null
}

function Test-PasswordFileExists {
    param(
        [string]$PasswordFile,
        [string]$ConfigRoot,
        [string]$StorageConfigPath
    )

    if ([string]::IsNullOrWhiteSpace($PasswordFile)) {
        return $false
    }

    if ([System.IO.Path]::IsPathRooted($PasswordFile)) {
        return Test-Path $PasswordFile
    }

    $storageRoot = Split-Path -Parent $StorageConfigPath
    foreach ($candidate in @(
            (Join-Path $ConfigRoot $PasswordFile),
            (Join-Path $storageRoot $PasswordFile)
        )) {
        if (Test-Path $candidate) {
            return $true
        }
    }

    return $false
}

$missing = New-Object System.Collections.Generic.List[string]
$configMissing = New-Object System.Collections.Generic.List[string]
$storageMissing = New-Object System.Collections.Generic.List[string]

$serverConfigPath = Get-FirstExistingPath @(
    (Join-Path $ConfigDir "server.yaml"),
    (Join-Path $ConfigDir "chat.toml")
)
$postgresqlPath = Get-FirstExistingPath @(
    (Join-Path $ConfigDir "postgresql.yaml"),
    (Join-Path $ConfigDir "storage\postgresql.yaml")
)

if ([string]::IsNullOrWhiteSpace($serverConfigPath)) {
    $configMissing.Add("server.yaml")
}
if ([string]::IsNullOrWhiteSpace($postgresqlPath)) {
    $storageMissing.Add("postgresql.yaml")
}

$serverContent = if (-not [string]::IsNullOrWhiteSpace($serverConfigPath)) { Get-Content -Path $serverConfigPath -Raw } else { "" }
$storageContent = if (-not [string]::IsNullOrWhiteSpace($postgresqlPath)) { Get-Content -Path $postgresqlPath -Raw } else { "" }

if (-not [string]::IsNullOrWhiteSpace($serverConfigPath)) {
    if ((Split-Path -Leaf $serverConfigPath) -eq "server.yaml") {
        foreach ($contract in @("instance:", "name:", "network:", "bindAddress:", "publicEndpoints:", "baseUrl:", "runtime:", "dataRoot:")) {
            if (-not $serverContent.Contains($contract)) { $configMissing.Add($contract) }
        }
    }
    else {
        foreach ($contract in @("[runtime]", "deployment_mode = ""server""", "app_code = ""chat""", "[server]", "bind_address =")) {
            if (-not $serverContent.Contains($contract)) { $configMissing.Add($contract) }
        }
    }
}

if (-not [string]::IsNullOrWhiteSpace($postgresqlPath)) {
    foreach ($contract in @("provider: postgresql", "passwordFile:", "migrationMode:")) {
        if (-not $storageContent.Contains($contract)) { $storageMissing.Add($contract) }
    }

    $passwordFile = Get-YamlScalarValue -Content $storageContent -Key "passwordFile"
    if (-not (Test-PasswordFileExists -PasswordFile $passwordFile -ConfigRoot $ConfigDir -StorageConfigPath $postgresqlPath)) {
        $storageMissing.Add("passwordFile target")
    }
}

foreach ($item in $configMissing) { $missing.Add($item) }
foreach ($item in $storageMissing) { $missing.Add($item) }

$releaseContracts = [ordered]@{
    enabled = $false
}

if (-not [string]::IsNullOrWhiteSpace($ReleaseGatePath)) {
    $releaseContracts = Get-ReleaseContractReport -GatePath $ReleaseGatePath
}

$configValid = ($configMissing.Count -eq 0)
$storageValid = ($storageMissing.Count -eq 0)
$ready = ($configValid -and $storageValid)
if ($releaseContracts.enabled) {
    $ready = $ready -and [bool]$releaseContracts.contractsValid
}

$result = [ordered]@{
    product = "sdkwork-im-server"
    instance = $InstanceName
    config = $ConfigDir
    configValid = $configValid
    storageValid = $storageValid
    ready = $ready
    output = $OutputFormat
    missing = @($missing)
    releaseContracts = $releaseContracts
}

$json = $result | ConvertTo-Json -Depth 5
if ($OutputFormat -eq "json") {
    Write-Output $json
}
else {
    Write-Host "sdkwork-im-server verify report"
    Write-Host "config: $ConfigDir"
    Write-Host "configValid: $($result.configValid)"
    Write-Host "storageValid: $($result.storageValid)"
    Write-Host "ready: $($result.ready)"
    if ($missing.Count -gt 0) {
        Write-Host "missing: $($missing -join ', ')"
    }
    if ($releaseContracts.enabled) {
        Write-Host "releaseGate: $($releaseContracts.gatePath)"
        Write-Host "releaseBundle: $($releaseContracts.bundleId)"
        Write-Host "releaseDecisionStatus: $($releaseContracts.decisionStatus)"
        Write-Host "releasePlatforms: $($releaseContracts.platforms -join ', ')"
        Write-Host "releaseContractsValid: $($releaseContracts.contractsValid)"
        Write-Host "releaseSemanticIssueCount: $($releaseContracts.semanticIssueCount)"
        if ($releaseContracts.missing.Count -gt 0) {
            Write-Host "releaseMissing: $($releaseContracts.missing -join ', ')"
        }
        if ($releaseContracts.semanticIssues.Count -gt 0) {
            Write-Host "releaseSemanticIssues: $($releaseContracts.semanticIssues -join ', ')"
        }
    }
}
