param(
    [string]$InstanceName = "default",
    [string]$InstallRoot,
    [string]$ConfigDir,
    [string]$LogDir,
    [string]$RunDir,
    [string]$EnvFile,
    [string]$BinaryPath,
    [switch]$Foreground,
    [string]$HealthUrl,
    [switch]$SkipHealthCheck,
    [switch]$Help
)

$ErrorActionPreference = "Stop"

if ($Help) {
    Write-Host "Usage: powershell -ExecutionPolicy Bypass -File bin/start-prod.ps1 [-InstanceName <name>] [-InstallRoot <path>] [-ConfigDir <path>] [-LogDir <path>] [-RunDir <path>] [-EnvFile <path>] [-BinaryPath <path>] [-Foreground] [-HealthUrl <url>] [-SkipHealthCheck]"
    Write-Host "Start packaged Sdkwork IM server in production/release mode."
    exit 0
}

$argsList = @("-InstanceName", $InstanceName, "-Release")
if (-not [string]::IsNullOrWhiteSpace($InstallRoot)) { $argsList += @("-InstallRoot", $InstallRoot) }
if (-not [string]::IsNullOrWhiteSpace($ConfigDir)) { $argsList += @("-ConfigDir", $ConfigDir) }
if (-not [string]::IsNullOrWhiteSpace($LogDir)) { $argsList += @("-LogDir", $LogDir) }
if (-not [string]::IsNullOrWhiteSpace($RunDir)) { $argsList += @("-RunDir", $RunDir) }
if (-not [string]::IsNullOrWhiteSpace($EnvFile)) { $argsList += @("-EnvFile", $EnvFile) }
if (-not [string]::IsNullOrWhiteSpace($BinaryPath)) { $argsList += @("-BinaryPath", $BinaryPath) }
if ($Foreground) { $argsList += "-Foreground" }
if (-not [string]::IsNullOrWhiteSpace($HealthUrl)) { $argsList += @("-HealthUrl", $HealthUrl) }
if ($SkipHealthCheck) { $argsList += "-SkipHealthCheck" }

& (Join-Path $PSScriptRoot "start-server.ps1") @argsList
exit $LASTEXITCODE
