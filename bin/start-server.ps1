param(
    [string]$InstanceName = "default",
    [string]$InstallRoot = ([System.IO.Path]::Combine([Environment]::GetFolderPath("ProgramFiles"), "sdkwork", "chat")),
    [string]$ConfigDir = ([System.IO.Path]::Combine([Environment]::GetFolderPath("CommonApplicationData"), "sdkwork", "chat")),
    [string]$LogDir = ([System.IO.Path]::Combine([Environment]::GetFolderPath("CommonApplicationData"), "sdkwork", "chat", "Logs")),
    [string]$RunDir = ([System.IO.Path]::Combine([Environment]::GetFolderPath("CommonApplicationData"), "sdkwork", "chat", "Run")),
    [string]$EnvFile,
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
    Write-Host "Usage: powershell -ExecutionPolicy Bypass -File bin/start-server.ps1 [-InstanceName <name>] [-InstallRoot <path>] [-ConfigDir <path>] [-LogDir <path>] [-RunDir <path>] [-EnvFile <path>] [-BinaryPath <path>] [-Release] [-Foreground] [-HealthUrl <url>] [-SkipHealthCheck]"
    Write-Host "Usage: cmd /c .\bin\start-server.cmd [--instance <name>] [--install-root <path>] [--config-dir <path>] [--log-dir <path>] [--run-dir <path>] [--env-file <path>] [--binary-path <path>] [--release] [--foreground] [--health-url <url>] [--skip-health-check]"
    Write-Host "Start the sdkwork-im-server runtime service for an instance with config loading, binary resolution, log and run directory management, health checks, and status-friendly foreground or background execution."
    exit 0
}

function Read-ConfigValue {
    param([string]$ConfigFile, [string]$Key)
    if (-not (Test-Path $ConfigFile)) { return $null }
    foreach ($line in Get-Content -Path $ConfigFile) {
        if ($line -match "^\s*${Key}:\s*(.+?)\s*$") {
            return $Matches[1].Trim().Trim('"')
        }
        if ($line -match "^\s*${Key}\s*=\s*(.+?)\s*$") {
            return $Matches[1].Trim().Trim('"').Trim("'")
        }
    }
    return $null
}

function Resolve-ServerEnvFilePath {
    param([string]$ExplicitEnvFile, [string]$ResolvedConfigDir)

    if (-not [string]::IsNullOrWhiteSpace($ExplicitEnvFile)) {
        return $ExplicitEnvFile
    }

    return (Join-Path $ResolvedConfigDir "server.env")
}

function Get-FirstEnvValue {
    param([string[]]$Names)

    foreach ($name in $Names) {
        $value = [Environment]::GetEnvironmentVariable($name)
        if (-not [string]::IsNullOrWhiteSpace($value)) {
            return $value
        }
    }
    return $null
}

function Read-ServerEnvFile {
    param([string]$EnvFilePath)

    $values = @{}
    if (-not (Test-Path $EnvFilePath)) {
        return $values
    }

    foreach ($line in Get-Content -Path $EnvFilePath) {
        $trimmed = $line.Trim()
        if ($trimmed.Length -eq 0 -or $trimmed.StartsWith('#')) {
            continue
        }
        if ($trimmed.StartsWith('export ')) {
            $trimmed = $trimmed.Substring(7).Trim()
        }

        $parts = $trimmed -split '=', 2
        if ($parts.Count -ne 2) {
            continue
        }

        $key = $parts[0].Trim()
        if ([string]::IsNullOrWhiteSpace($key)) {
            continue
        }

        $value = $parts[1].Trim()
        if ($value.Length -ge 2) {
            if (($value.StartsWith('"') -and $value.EndsWith('"')) -or ($value.StartsWith("'") -and $value.EndsWith("'"))) {
                $value = $value.Substring(1, $value.Length - 2)
            }
        }

        $values[$key] = $value
    }

    return $values
}

function Import-ServerEnvFile {
    param([string]$EnvFilePath)

    foreach ($entry in (Read-ServerEnvFile -EnvFilePath $EnvFilePath).GetEnumerator()) {
        if ([string]::IsNullOrWhiteSpace([Environment]::GetEnvironmentVariable($entry.Key))) {
            Set-Item -Path ("Env:" + $entry.Key) -Value $entry.Value
        }
    }
}

function Resolve-ServerBinaryPath {
    param([string]$Root, [string]$InstallRoot, [string]$ExplicitBinaryPath, [bool]$PreferRelease)
    if (-not [string]::IsNullOrWhiteSpace($ExplicitBinaryPath) -and (Test-Path $ExplicitBinaryPath)) {
        return $ExplicitBinaryPath
    }
    $envBinaryPath = [Environment]::GetEnvironmentVariable("SDKWORK_IM_SERVER_BINARY_PATH")
    if (-not [string]::IsNullOrWhiteSpace($envBinaryPath) -and (Test-Path $envBinaryPath)) {
        return $envBinaryPath
    }

    $installCandidates = @(
        (Join-Path $InstallRoot "bin\sdkwork-im-server.exe"),
        (Join-Path $InstallRoot "bin\sdkwork-im-cloud-gateway.exe")
    )
    foreach ($candidate in $installCandidates) {
        if (Test-Path $candidate) { return $candidate }
    }

    $releaseCandidate = Join-Path $Root "target\release\sdkwork-im-server.exe"
    $debugCandidate = Join-Path $Root "target\debug\sdkwork-im-server.exe"
    $legacyReleaseCandidate = Join-Path $Root "target\release\web-gateway.exe"
    $legacyDebugCandidate = Join-Path $Root "target\debug\web-gateway.exe"
    $candidates = if ($PreferRelease) {
        @($releaseCandidate, $debugCandidate, $legacyReleaseCandidate, $legacyDebugCandidate)
    }
    else {
        @($debugCandidate, $releaseCandidate, $legacyDebugCandidate, $legacyReleaseCandidate)
    }
    foreach ($candidate in $candidates) {
        if (Test-Path $candidate) { return $candidate }
    }

    $cargo = Get-Command cargo -ErrorAction SilentlyContinue
    if ($null -ne $cargo) {
        if ($PreferRelease) {
            cargo build --release -p sdkwork-im-cloud-gateway --offline | Out-Host
        }
        else {
            cargo build -p sdkwork-im-cloud-gateway --offline | Out-Host
        }
        foreach ($candidate in $candidates) {
            if (Test-Path $candidate) { return $candidate }
        }
    }

    return $null
}

function Resolve-HealthUrl {
    param([string]$ExplicitHealthUrl, [string]$ResolvedBindAddress)
    if (-not [string]::IsNullOrWhiteSpace($ExplicitHealthUrl)) { return $ExplicitHealthUrl }
    $segments = $ResolvedBindAddress -split ':'
    if ($segments.Length -lt 2) { return "http://127.0.0.1:18079/healthz" }
    $port = $segments[-1]
    $bindHost = ($segments[0..($segments.Length - 2)] -join ':').Trim()
    if ([string]::IsNullOrWhiteSpace($bindHost) -or $bindHost -eq "0.0.0.0" -or $bindHost -eq "::" -or $bindHost -eq "[::]") {
        $bindHost = "127.0.0.1"
    }
    return "http://$bindHost`:$port/healthz"
}

function Get-ManagedProcess {
    param([string]$PidFile, [string]$ExpectedProcessName)
    if (-not (Test-Path $PidFile)) { return $null }
    $raw = Get-Content -Path $PidFile -ErrorAction SilentlyContinue | Select-Object -First 1
    if ([string]::IsNullOrWhiteSpace($raw)) { return $null }
    try {
        $process = Get-Process -Id ([int]$raw.Trim()) -ErrorAction Stop
        if (-not [string]::IsNullOrWhiteSpace($ExpectedProcessName) -and $process.ProcessName -ine $ExpectedProcessName) {
            return $null
        }
        return $process
    }
    catch { return $null }
}

$root = Split-Path -Parent $PSScriptRoot
$serverEnvPath = Resolve-ServerEnvFilePath -ExplicitEnvFile $EnvFile -ResolvedConfigDir $ConfigDir
Import-ServerEnvFile -EnvFilePath $serverEnvPath
$standardConfigFile = Get-FirstEnvValue @("SDKWORK_IM_CONFIG_FILE")
if ([string]::IsNullOrWhiteSpace($standardConfigFile)) {
    $serverYamlPath = Join-Path $ConfigDir "chat.toml"
    if (-not (Test-Path $serverYamlPath)) {
        $serverYamlPath = Join-Path $ConfigDir "server.yaml"
    }
}
else {
    $serverYamlPath = $standardConfigFile
}
if (-not (Test-Path $serverYamlPath)) {
    $chatTomlPath = Join-Path $ConfigDir "chat.toml"
    if (Test-Path $chatTomlPath) {
        $serverYamlPath = $chatTomlPath
    }
    else {
        throw "Missing server config. Run init-config-server first: $serverYamlPath"
    }
}

$standardBindAddress = Get-FirstEnvValue @("SDKWORK_IM_APPLICATION_PUBLIC_INGRESS_BIND")
$resolvedBindAddress = if (-not [string]::IsNullOrWhiteSpace($standardBindAddress)) {
    $standardBindAddress
}
else {
    $fromToml = Read-ConfigValue -ConfigFile $serverYamlPath -Key "bind_address"
    if ([string]::IsNullOrWhiteSpace($fromToml)) {
        Read-ConfigValue -ConfigFile $serverYamlPath -Key "bindAddress"
    }
    else {
        $fromToml
    }
}
if ([string]::IsNullOrWhiteSpace($resolvedBindAddress)) {
    $resolvedBindAddress = "127.0.0.1:18079"
}
$resolvedBinaryPath = Resolve-ServerBinaryPath -Root $root -InstallRoot $InstallRoot -ExplicitBinaryPath $BinaryPath -PreferRelease:$Release
if ([string]::IsNullOrWhiteSpace($resolvedBinaryPath)) {
    throw "Unable to resolve sdkwork-im-server binary. Set -BinaryPath, install a packaged binary under $InstallRoot, or build sdkwork-im-cloud-gateway."
}

$resolvedHealthUrl = Resolve-HealthUrl -ExplicitHealthUrl $HealthUrl -ResolvedBindAddress $resolvedBindAddress
$stdoutLog = Join-Path $LogDir "sdkwork-im-server.out.log"
$stderrLog = Join-Path $LogDir "sdkwork-im-server.err.log"
$pidFile = Join-Path $RunDir "sdkwork-im-server.pid"
$processInfoPath = Join-Path $RunDir "sdkwork-im-server.process.json"
foreach ($path in @($LogDir, $RunDir)) {
    if (-not (Test-Path $path)) {
        New-Item -ItemType Directory -Path $path -Force | Out-Null
    }
}

$expectedProcessName = [System.IO.Path]::GetFileNameWithoutExtension($resolvedBinaryPath)
$existing = Get-ManagedProcess -PidFile $pidFile -ExpectedProcessName $expectedProcessName
if ($null -ne $existing) {
    throw "sdkwork-im-server is already running with PID $($existing.Id)."
}

$env:SDKWORK_IM_APPLICATION_PUBLIC_INGRESS_BIND = $resolvedBindAddress
$env:SDKWORK_IM_WEB_GATEWAY_BIND = $resolvedBindAddress
$serverArguments = @("--config", $serverYamlPath)

if ($Foreground) {
    & $resolvedBinaryPath @serverArguments
    exit $LASTEXITCODE
}

$process = Start-Process -FilePath $resolvedBinaryPath -ArgumentList $serverArguments -WorkingDirectory $root -PassThru -RedirectStandardOutput $stdoutLog -RedirectStandardError $stderrLog
$process.Id | Set-Content -Path $pidFile -Encoding utf8
([ordered]@{
    binaryPath = $resolvedBinaryPath
    processName = $expectedProcessName
    bindAddress = $resolvedBindAddress
    healthUrl = $resolvedHealthUrl
} | ConvertTo-Json -Depth 4) | Set-Content -Path $processInfoPath -Encoding utf8

if (-not $SkipHealthCheck) {
    for ($i = 0; $i -lt 30; $i++) {
        Start-Sleep -Seconds 1
        if ($null -eq (Get-Process -Id $process.Id -ErrorAction SilentlyContinue)) {
            Remove-Item -Path $pidFile -Force -ErrorAction SilentlyContinue
            throw "sdkwork-im-server exited before becoming healthy. Check logs: $stderrLog"
        }
        try {
            $response = Invoke-WebRequest -Uri $resolvedHealthUrl -UseBasicParsing -TimeoutSec 2
            if ($response.StatusCode -eq 200) {
                Write-Host "Started sdkwork-im-server in background on $resolvedHealthUrl"
                exit 0
            }
        }
        catch { }
    }
    Stop-Process -Id $process.Id -Force -ErrorAction SilentlyContinue
    Remove-Item -Path $pidFile -Force -ErrorAction SilentlyContinue
    throw "sdkwork-im-server did not become healthy within 30 seconds: $resolvedHealthUrl"
}

Write-Host "Started sdkwork-im-server in background without health wait."
