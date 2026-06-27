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
    Write-Host "Usage: powershell -ExecutionPolicy Bypass -File bin/status-server.ps1 [-InstanceName <name>] [-ConfigDir <path>] [-ReleaseGatePath <path-to-release-gate.json>] [-OutputFormat <text|json>]"
    Write-Host "Show sdkwork-im-server status, generated service contracts, storage report paths, and optionally summarize the machine-readable release-gate bundle, decisionStatus, contractsValid, platforms, and semanticIssues."
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

$generatedUnitPath = Join-Path $ConfigDir "generated\sdkwork-im-server.service"
$generatedLaunchdPath = Join-Path $ConfigDir "generated\com.sdkwork.im.server.plist"
$generatedWindowsServiceXmlPath = Join-Path $ConfigDir "generated\SdkworkImServer.xml"
$generatedWindowsServiceInstallScriptPath = Join-Path $ConfigDir "generated\install-SdkworkImServer.ps1"
$generatedWindowsServiceUninstallScriptPath = Join-Path $ConfigDir "generated\uninstall-SdkworkImServer.ps1"
$verifyReportPath = Join-Path $ConfigDir "storage-init-report.json"

$releaseContracts = [ordered]@{
    enabled = $false
}

if (-not [string]::IsNullOrWhiteSpace($ReleaseGatePath)) {
    $releaseContracts = Get-ReleaseContractReport -GatePath $ReleaseGatePath
}

$serviceContracts = [ordered]@{
    systemd = [ordered]@{
        path = $generatedUnitPath
        exists = (Test-Path -LiteralPath $generatedUnitPath)
    }
    launchd = [ordered]@{
        path = $generatedLaunchdPath
        label = "com.sdkwork.im.server"
        exists = (Test-Path -LiteralPath $generatedLaunchdPath)
    }
    windowsService = [ordered]@{
        path = $generatedWindowsServiceXmlPath
        target = "SdkworkImServer"
        installScriptPath = $generatedWindowsServiceInstallScriptPath
        uninstallScriptPath = $generatedWindowsServiceUninstallScriptPath
        exists = (Test-Path -LiteralPath $generatedWindowsServiceXmlPath)
        installScriptExists = (Test-Path -LiteralPath $generatedWindowsServiceInstallScriptPath)
        uninstallScriptExists = (Test-Path -LiteralPath $generatedWindowsServiceUninstallScriptPath)
    }
}

$storageReport = [ordered]@{
    path = $verifyReportPath
    exists = (Test-Path -LiteralPath $verifyReportPath)
}

$result = [ordered]@{
    product = "sdkwork-im-server"
    instance = $InstanceName
    config = $ConfigDir
    status = "configuration-only skeleton"
    output = $OutputFormat
    serviceContracts = $serviceContracts
    storageReport = $storageReport
    releaseContracts = $releaseContracts
}

if ($OutputFormat -eq "json") {
    $result | ConvertTo-Json -Depth 6
    exit 0
}

Write-Host "sdkwork-im-server status"
Write-Host "instance: $InstanceName"
Write-Host "config: $ConfigDir"
Write-Host "status: configuration-only skeleton"
Write-Host "systemd contract: $generatedUnitPath"
Write-Host "launchd contract: $generatedLaunchdPath"
Write-Host "launchd label: com.sdkwork.im.server"
Write-Host "windows service contract: $generatedWindowsServiceXmlPath"
Write-Host "windows service install script: $generatedWindowsServiceInstallScriptPath"
Write-Host "windows service uninstall script: $generatedWindowsServiceUninstallScriptPath"
Write-Host "windows service target: SdkworkImServer"
Write-Host "storage report: $verifyReportPath"
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
