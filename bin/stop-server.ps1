param(
    [string]$InstanceName = "default",
    [string]$ConfigDir = ([System.IO.Path]::Combine([Environment]::GetFolderPath("CommonApplicationData"), "sdkwork", "chat")),
    [string]$RunDir = ([System.IO.Path]::Combine([Environment]::GetFolderPath("CommonApplicationData"), "sdkwork", "chat", "Run")),
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
if ($PSBoundParameters.ContainsKey("InstanceName") -and -not $PSBoundParameters.ContainsKey("RunDir")) {
    $RunDir = Get-ServerPathForInstance $programDataRoot $InstanceName "Run"
}

if ($Help) {
    Write-Host "Usage: powershell -ExecutionPolicy Bypass -File bin/stop-server.ps1 [-InstanceName <name>] [-ConfigDir <path>] [-RunDir <path>]"
    Write-Host "Stop the sdkwork-im-server runtime service for an instance by using the pid file under the run directory, honoring config ownership, and reporting status."
    exit 0
}

$pidFile = Join-Path $RunDir "sdkwork-im-server.pid"
$processInfoPath = Join-Path $RunDir "sdkwork-im-server.process.json"
if (-not (Test-Path $pidFile)) {
    Write-Host "sdkwork-im-server is not running."
    exit 0
}

$rawPid = Get-Content -Path $pidFile -ErrorAction SilentlyContinue | Select-Object -First 1
if ([string]::IsNullOrWhiteSpace($rawPid)) {
    Remove-Item -Path $pidFile -Force -ErrorAction SilentlyContinue
    Write-Host "sdkwork-im-server pid file was empty and has been cleared."
    exit 0
}

try {
    $pid = [int]$rawPid.Trim()
    $process = Get-Process -Id $pid -ErrorAction Stop
    Stop-Process -Id $pid -ErrorAction Stop
    try { Wait-Process -Id $pid -Timeout 30 -ErrorAction Stop } catch { }
    Write-Host "Stopped sdkwork-im-server PID $pid"
}
catch {
    Write-Host "sdkwork-im-server process from pid file is not running."
}

Remove-Item -Path $pidFile -Force -ErrorAction SilentlyContinue
Remove-Item -Path $processInfoPath -Force -ErrorAction SilentlyContinue
