param(
    [string]$InstanceName = "default",
    [string]$InstallRoot = ([System.IO.Path]::Combine([Environment]::GetFolderPath("ProgramFiles"), "sdkwork", "chat")),
    [string]$ConfigDir = ([System.IO.Path]::Combine([Environment]::GetFolderPath("CommonApplicationData"), "sdkwork", "chat")),
    [string]$LogDir = ([System.IO.Path]::Combine([Environment]::GetFolderPath("CommonApplicationData"), "sdkwork", "chat", "Logs")),
    [ValidateSet("auto", "systemd", "launchd", "windows-service")]
    [string]$ServiceMode = "auto",
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

if ($Help) {
    Write-Host "Usage: powershell -ExecutionPolicy Bypass -File bin/install-service-server.ps1 [-InstanceName <name>] [-InstallRoot <path>] [-ConfigDir <path>] [-LogDir <path>] [-ServiceMode <auto|systemd|launchd|windows-service>]"
    Write-Host "Render the sdkwork-im-server service contract, generate systemd and launchd targets, generate Windows Service wrapper targets, and report install status."
    exit 0
}

$root = Split-Path -Parent $PSScriptRoot
$systemdTemplate = Join-Path $root "deployments\systemd\sdkwork-im-server.service"
$launchdTemplate = Join-Path $root "deployments\launchd\com.sdkwork.im.server.plist"
$windowsServiceTemplate = Join-Path $root "deployments\windows-service\SdkworkImServer.xml"
$generatedDir = Join-Path $ConfigDir "generated"
if (-not (Test-Path $generatedDir)) {
    New-Item -ItemType Directory -Path $generatedDir -Force | Out-Null
}
$null = New-Item -ItemType Directory -Path $LogDir -Force -ErrorAction SilentlyContinue
$generatedUnitPath = Join-Path $generatedDir "sdkwork-im-server.service"
$generatedLaunchdPath = Join-Path $generatedDir "com.sdkwork.im.server.plist"
$generatedWindowsServiceXmlPath = Join-Path $generatedDir "SdkworkImServer.xml"
$generatedWindowsServiceInstallScriptPath = Join-Path $generatedDir "install-SdkworkImServer.ps1"
$generatedWindowsServiceUninstallScriptPath = Join-Path $generatedDir "uninstall-SdkworkImServer.ps1"
$serviceReportPath = Join-Path $generatedDir "service-install-report.json"

$normalizedInstallRoot = $InstallRoot.TrimEnd('\', '/')
$normalizedConfigDir = $ConfigDir.TrimEnd('\', '/')
$normalizedLogDir = $LogDir.TrimEnd('\', '/')
$environmentFile = Join-Path $normalizedConfigDir "server.env"
$serverConfigPath = Join-Path $normalizedConfigDir "server.yaml"
$serviceBinaryPath = "$normalizedInstallRoot/bin/sdkwork-im-server"
$windowsServiceWrapperExePath = Join-Path (Join-Path $normalizedInstallRoot "bin") "SdkworkImServer.exe"
$windowsServiceWrapperXmlTargetPath = Join-Path (Join-Path $normalizedInstallRoot "bin") "SdkworkImServer.xml"
$stdoutLogPath = Join-Path $normalizedLogDir "sdkwork-im-server.out.log"
$stderrLogPath = Join-Path $normalizedLogDir "sdkwork-im-server.err.log"

if (Test-Path $systemdTemplate) {
    $unitContent = Get-Content -Path $systemdTemplate -Raw
    $rendered = $unitContent.
        Replace('WorkingDirectory=/opt/sdkwork/chat', "WorkingDirectory=$normalizedInstallRoot").
        Replace('EnvironmentFile=/etc/sdkwork/chat/server.env', "EnvironmentFile=$environmentFile").
        Replace('ExecStart=/opt/sdkwork/chat/bin/sdkwork-im-server --config /etc/sdkwork/chat/server.yaml', "ExecStart=$serviceBinaryPath --config $serverConfigPath")
    $rendered | Set-Content -Path $generatedUnitPath -Encoding utf8
}

if (Test-Path $launchdTemplate) {
    $plistContent = Get-Content -Path $launchdTemplate -Raw
    $rendered = $plistContent.
        Replace('__INSTALL_ROOT__/bin/sdkwork-im-server', $serviceBinaryPath).
        Replace('__CONFIG_DIR__/server.yaml', $serverConfigPath).
        Replace('__LOG_DIR__/sdkwork-im-server.out.log', $stdoutLogPath).
        Replace('__LOG_DIR__/sdkwork-im-server.err.log', $stderrLogPath).
        Replace('__INSTALL_ROOT__', $normalizedInstallRoot).
        Replace('__CONFIG_DIR__', $normalizedConfigDir).
        Replace('__LOG_DIR__', $normalizedLogDir)
    $rendered | Set-Content -Path $generatedLaunchdPath -Encoding utf8
}

if (Test-Path $windowsServiceTemplate) {
    $xmlContent = Get-Content -Path $windowsServiceTemplate -Raw
    $rendered = $xmlContent.
        Replace('__INSTALL_ROOT__', $normalizedInstallRoot).
        Replace('__CONFIG_DIR__', $normalizedConfigDir).
        Replace('__LOG_DIR__', $normalizedLogDir)
    $rendered | Set-Content -Path $generatedWindowsServiceXmlPath -Encoding utf8
}

@"
`$ErrorActionPreference = "Stop"
`$wrapperExePath = "$windowsServiceWrapperExePath"
`$wrapperConfigSourcePath = "$generatedWindowsServiceXmlPath"
`$wrapperConfigTargetPath = "$windowsServiceWrapperXmlTargetPath"

if (-not (Test-Path `$wrapperExePath)) {
    throw "Missing Windows Service wrapper executable: `$wrapperExePath. Bundle a dedicated service-host wrapper before registration."
}
if (-not (Test-Path `$wrapperConfigSourcePath)) {
    throw "Missing generated Windows Service wrapper config: `$wrapperConfigSourcePath"
}

Copy-Item -LiteralPath `$wrapperConfigSourcePath -Destination `$wrapperConfigTargetPath -Force
& `$wrapperExePath install
"@ | Set-Content -Path $generatedWindowsServiceInstallScriptPath -Encoding utf8

@"
`$ErrorActionPreference = "Stop"
`$wrapperExePath = "$windowsServiceWrapperExePath"
`$wrapperConfigTargetPath = "$windowsServiceWrapperXmlTargetPath"

if (Test-Path `$wrapperExePath) {
    & `$wrapperExePath uninstall
}
if (Test-Path `$wrapperConfigTargetPath) {
    Remove-Item -LiteralPath `$wrapperConfigTargetPath -Force
}
"@ | Set-Content -Path $generatedWindowsServiceUninstallScriptPath -Encoding utf8

$serviceReport = [ordered]@{
    product = "sdkwork-im-server"
    instance = $InstanceName
    installRoot = $InstallRoot
    configDir = $ConfigDir
    logDir = $LogDir
    serviceMode = $ServiceMode
    systemdUnit = $generatedUnitPath
    launchdPlist = $generatedLaunchdPath
    launchdLabel = "com.sdkwork.im.server"
    windowsServiceHostMode = "wrapper-required"
    windowsServiceName = "SdkworkImServer"
    windowsServiceWrapperExe = $windowsServiceWrapperExePath
    windowsServiceWrapperConfig = $generatedWindowsServiceXmlPath
    windowsServiceInstallScript = $generatedWindowsServiceInstallScriptPath
    windowsServiceUninstallScript = $generatedWindowsServiceUninstallScriptPath
}
$serviceReport | ConvertTo-Json -Depth 4 | Set-Content -Path $serviceReportPath -Encoding utf8

Write-Host "sdkwork-im-server service install summary"
Write-Host "instance: $InstanceName"
Write-Host "install: $InstallRoot"
Write-Host "config: $ConfigDir"
Write-Host "status: generated service contract"
Write-Host "systemd template: $systemdTemplate"
Write-Host "systemd unit: $generatedUnitPath"
Write-Host "launchd template: $launchdTemplate"
Write-Host "launchd plist: $generatedLaunchdPath"
Write-Host "windows service template: $windowsServiceTemplate"
Write-Host "windows service wrapper config: $generatedWindowsServiceXmlPath"
Write-Host "windows service install script: $generatedWindowsServiceInstallScriptPath"
Write-Host "windows service uninstall script: $generatedWindowsServiceUninstallScriptPath"
Write-Host "launchd target: com.sdkwork.im.server"
Write-Host "windows service target: SdkworkImServer"
