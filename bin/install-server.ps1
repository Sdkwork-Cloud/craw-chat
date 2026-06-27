param(
    [string]$InstanceName = "default",
    [string]$InstallRoot = ([System.IO.Path]::Combine([Environment]::GetFolderPath("ProgramFiles"), "sdkwork", "chat")),
    [string]$ConfigDir = ([System.IO.Path]::Combine([Environment]::GetFolderPath("CommonApplicationData"), "sdkwork", "chat")),
    [string]$DataDir = ([System.IO.Path]::Combine([Environment]::GetFolderPath("CommonApplicationData"), "sdkwork", "chat", "Data")),
    [string]$LogDir = ([System.IO.Path]::Combine([Environment]::GetFolderPath("CommonApplicationData"), "sdkwork", "chat", "Logs")),
    [string]$RunDir = ([System.IO.Path]::Combine([Environment]::GetFolderPath("CommonApplicationData"), "sdkwork", "chat", "Run")),
    [switch]$NonInteractive,
    [switch]$Force,
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
if ($PSBoundParameters.ContainsKey("InstanceName") -and -not $PSBoundParameters.ContainsKey("DataDir")) {
    $DataDir = Get-ServerPathForInstance $programDataRoot $InstanceName "Data"
}
if ($PSBoundParameters.ContainsKey("InstanceName") -and -not $PSBoundParameters.ContainsKey("LogDir")) {
    $LogDir = Get-ServerPathForInstance $programDataRoot $InstanceName "Logs"
}
if ($PSBoundParameters.ContainsKey("InstanceName") -and -not $PSBoundParameters.ContainsKey("RunDir")) {
    $RunDir = Get-ServerPathForInstance $programDataRoot $InstanceName "Run"
}

if ($Help) {
    Write-Host "Usage: powershell -ExecutionPolicy Bypass -File bin/install-server.ps1 [-InstanceName <name>] [-InstallRoot <path>] [-ConfigDir <path>] [-DataDir <path>] [-LogDir <path>] [-RunDir <path>] [-NonInteractive] [-Force]"
    Write-Host "Usage: cmd /c .\bin\install-server.cmd [--instance <name>] [--install-root <path>] [--config-dir <path>] [--data-dir <path>] [--log-dir <path>] [--run-dir <path>] [--non-interactive] [--force]"
    Write-Host "Create the sdkwork-im-server install/config/data/log/run directory skeleton and stage canonical payload examples."
    exit 0
}

$root = Split-Path -Parent $PSScriptRoot
$installMetadataPath = Join-Path $ConfigDir "install.json"

function Resolve-ServerTemplatePath {
    param(
        [string[]]$PackagedRelativePaths,
        [string]$SourceRelativePath
    )

    foreach ($relativePath in $PackagedRelativePaths) {
        $packagedPath = Join-Path $root $relativePath
        if (Test-Path $packagedPath) {
            return $packagedPath
        }
    }

    $sourcePath = Join-Path $root $SourceRelativePath
    if (Test-Path $sourcePath) {
        return $sourcePath
    }

    $packagedCandidates = $PackagedRelativePaths -join ', '
    throw "Missing sdkwork-im-server template. Expected packaged path '$packagedCandidates' or source path '$sourcePath'."
}

foreach ($path in @($InstallRoot, $ConfigDir, $DataDir, $LogDir, $RunDir)) {
    if (-not (Test-Path $path)) {
        New-Item -ItemType Directory -Path $path -Force | Out-Null
    }
}

$copies = @(
    @{
        Source = (Resolve-ServerTemplatePath -PackagedRelativePaths @("config\chat.toml.example", "config\server.yaml.example") -SourceRelativePath "deployments\templates\chat.toml.example")
        Destination = (Join-Path $ConfigDir "chat.toml.example")
    },
    @{
        Source = (Resolve-ServerTemplatePath -PackagedRelativePaths @("config\server.env.example") -SourceRelativePath "deployments\templates\server.env.example")
        Destination = (Join-Path $ConfigDir "server.env.example")
    },
    @{
        Source = (Resolve-ServerTemplatePath -PackagedRelativePaths @("config\postgresql.yaml.example", "config\storage\postgresql.yaml.example") -SourceRelativePath "deployments\templates\postgresql.yaml.example")
        Destination = (Join-Path $ConfigDir "postgresql.yaml.example")
    }
)

foreach ($copy in $copies) {
    if ((-not (Test-Path $copy.Destination)) -or $Force) {
        Copy-Item -LiteralPath $copy.Source -Destination $copy.Destination -Force
    }
}

$installMetadata = [ordered]@{
    product = "chat"
    appCode = "chat"
    instance = $InstanceName
    installRoot = $InstallRoot
    configDir = $ConfigDir
    dataDir = $DataDir
    logDir = $LogDir
    runDir = $RunDir
    nonInteractive = [bool]$NonInteractive
}

$installMetadata | ConvertTo-Json -Depth 4 | Set-Content -Path $installMetadataPath -Encoding utf8

Write-Host "Prepared sdkwork-im-server directories for instance '$InstanceName'."
Write-Host "ConfigDir: $ConfigDir"
Write-Host "DataDir: $DataDir"
Write-Host "LogDir: $LogDir"
Write-Host "RunDir: $RunDir"
