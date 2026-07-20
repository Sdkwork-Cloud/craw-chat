param(
    [string]$InstanceName = "default",
    [string]$ConfigDir = ([System.IO.Path]::Combine([Environment]::GetFolderPath("CommonApplicationData"), "sdkwork", "chat")),
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
    Write-Host "Usage: powershell -ExecutionPolicy Bypass -File bin/uninstall-service-server.ps1 [-InstanceName <name>] [-ConfigDir <path>]"
    Write-Host "Remove generated sdkwork-api-im-standalone-gateway service artifacts and summarize systemd/launchd/windows-service uninstall status."
    exit 0
}

$generatedDir = Join-Path $ConfigDir "generated"
foreach ($path in @(
    (Join-Path $generatedDir "sdkwork-api-im-standalone-gateway.service"),
    (Join-Path $generatedDir "com.sdkwork.im.api-standalone-gateway.plist"),
    (Join-Path $generatedDir "sdkwork-api-im-standalone-gateway-service.xml"),
    (Join-Path $generatedDir "install-sdkwork-api-im-standalone-gateway-service.ps1"),
    (Join-Path $generatedDir "uninstall-sdkwork-api-im-standalone-gateway-service.ps1"),
    (Join-Path $generatedDir "service-install-report.json")
)) {
    if (Test-Path $path) {
        Remove-Item -Path $path -Force -ErrorAction SilentlyContinue
    }
}

Write-Host "Removed generated sdkwork-api-im-standalone-gateway service artifacts for instance '$InstanceName'."
Write-Host "systemd target: sdkwork-api-im-standalone-gateway.service"
Write-Host "launchd target: com.sdkwork.im.api-standalone-gateway"
Write-Host "windows service target: sdkwork-api-im-standalone-gateway"
