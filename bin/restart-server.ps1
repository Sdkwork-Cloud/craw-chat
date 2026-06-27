param(
    [string]$InstanceName = "default",
    [string]$InstallRoot = ([System.IO.Path]::Combine([Environment]::GetFolderPath("ProgramFiles"), "sdkwork", "chat")),
    [string]$ConfigDir = ([System.IO.Path]::Combine([Environment]::GetFolderPath("CommonApplicationData"), "sdkwork", "chat")),
    [string]$LogDir = ([System.IO.Path]::Combine([Environment]::GetFolderPath("CommonApplicationData"), "sdkwork", "chat", "Logs")),
    [string]$RunDir = ([System.IO.Path]::Combine([Environment]::GetFolderPath("CommonApplicationData"), "sdkwork", "chat", "Run")),
    [string]$BinaryPath,
    [switch]$Release,
    [switch]$Foreground,
    [string]$HealthUrl,
    [switch]$SkipHealthCheck,
    [switch]$Help
)

$ErrorActionPreference = "Stop"

function Get-ServerPathForInstance {
    param([string]$Root, [string]$Name, [string]$Leaf)

    if ($Name -eq "default") {
        if ([string]::IsNullOrWhiteSpace($Leaf)) {
            return $Root
        }
        return [System.IO.Path]::Combine($Root, $Leaf)
    }
    if ([string]::IsNullOrWhiteSpace($Leaf)) {
        return [System.IO.Path]::Combine($Root, "instances", $Name)
    }
    return [System.IO.Path]::Combine($Root, "instances", $Name, $Leaf)
}

$programDataRoot = [System.IO.Path]::Combine([Environment]::GetFolderPath("CommonApplicationData"), "sdkwork", "chat")
if ($PSBoundParameters.ContainsKey("InstanceName") -and -not $PSBoundParameters.ContainsKey("ConfigDir")) {
    $ConfigDir = Get-ServerPathForInstance $programDataRoot $InstanceName ""
}
if ($PSBoundParameters.ContainsKey("InstanceName") -and -not $PSBoundParameters.ContainsKey("LogDir")) {
    $LogDir = Get-ServerPathForInstance $programDataRoot $InstanceName "Logs"
}
if ($PSBoundParameters.ContainsKey("InstanceName") -and -not $PSBoundParameters.ContainsKey("RunDir")) {
    $RunDir = Get-ServerPathForInstance $programDataRoot $InstanceName "Run"
}

if ($Help) {
    Write-Host "Usage: powershell -ExecutionPolicy Bypass -File bin/restart-server.ps1 [-InstanceName <name>] [-InstallRoot <path>] [-ConfigDir <path>] [-LogDir <path>] [-RunDir <path>] [-BinaryPath <path>] [-Release] [-Foreground] [-HealthUrl <url>] [-SkipHealthCheck]"
    Write-Host "Restart sdkwork-im-server using the stop/start runtime service scripts and preserve instance/config/status semantics."
    exit 0
}

$stopScript = Join-Path $PSScriptRoot "stop-server.ps1"
$startScript = Join-Path $PSScriptRoot "start-server.ps1"
& $stopScript -InstanceName $InstanceName -ConfigDir $ConfigDir -RunDir $RunDir | Out-Host
if ($null -ne $LASTEXITCODE -and $LASTEXITCODE -ne 0) {
    exit $LASTEXITCODE
}
& $startScript -InstanceName $InstanceName -InstallRoot $InstallRoot -ConfigDir $ConfigDir -LogDir $LogDir -RunDir $RunDir -BinaryPath $BinaryPath -Release:$Release -Foreground:$Foreground -HealthUrl $HealthUrl -SkipHealthCheck:$SkipHealthCheck
