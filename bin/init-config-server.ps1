param(
    [string]$InstanceName = "default",
    [string]$ConfigDir = ([System.IO.Path]::Combine([Environment]::GetFolderPath("CommonApplicationData"), "sdkwork", "chat")),
    [string]$DataDir = ([System.IO.Path]::Combine([Environment]::GetFolderPath("CommonApplicationData"), "sdkwork", "chat", "Data")),
    [string]$LogDir = ([System.IO.Path]::Combine([Environment]::GetFolderPath("CommonApplicationData"), "sdkwork", "chat", "Logs")),
    [string]$RunDir = ([System.IO.Path]::Combine([Environment]::GetFolderPath("CommonApplicationData"), "sdkwork", "chat", "Run")),
    [string]$BindAddress = "0.0.0.0:18080",
    [string]$BaseUrl = "http://127.0.0.1:18080",
    [string]$ApiBaseUrl = "http://127.0.0.1:18080",
    [string]$WebsocketBaseUrl = "ws://127.0.0.1:18080",
    [string]$BrowserOrigins = "http://127.0.0.1:18080,http://localhost:18080",
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
    Write-Host "Usage: powershell -ExecutionPolicy Bypass -File bin/init-config-server.ps1 [-InstanceName <name>] [-ConfigDir <path>] [-DataDir <path>] [-LogDir <path>] [-RunDir <path>] [-BindAddress <host:port>] [-BaseUrl <url>] [-ApiBaseUrl <url>] [-WebsocketBaseUrl <url>] [-BrowserOrigins <csv>] [-NonInteractive] [-Force]"
    Write-Host "Usage: cmd /c .\bin\init-config-server.cmd [--instance <name>] [--config-dir <path>] [--data-dir <path>] [--log-dir <path>] [--run-dir <path>] [--bind-address <host:port>] [--base-url <url>] [--api-base-url <url>] [--websocket-base-url <url>] [--browser-origins <csv>] [--non-interactive] [--force]"
    Write-Host "Render sdkwork-im-server configuration files for the selected instance and preserve file-based PostgreSQL settings."
    exit 0
}

function ConvertTo-TomlPath {
    param([string]$PathValue)
    return $PathValue.Replace('\', '/')
}

foreach ($path in @($ConfigDir, $DataDir, $LogDir, $RunDir)) {
    if (-not (Test-Path $path)) {
        New-Item -ItemType Directory -Path $path -Force | Out-Null
    }
}

$chatTomlPath = Join-Path $ConfigDir "chat.toml"
$serverEnvPath = Join-Path $ConfigDir "server.env"
$postgresqlPath = Join-Path $ConfigDir "postgresql.yaml"
$passwordFilePath = Join-Path $ConfigDir "database.secret"
$redisSecretPath = Join-Path $ConfigDir "redis.secret"

$tomlConfigDir = ConvertTo-TomlPath $ConfigDir
$tomlChatConfigFile = ConvertTo-TomlPath $chatTomlPath
$tomlDataDir = ConvertTo-TomlPath $DataDir
$tomlLogDir = ConvertTo-TomlPath $LogDir
$tomlRunDir = ConvertTo-TomlPath $RunDir
$tomlPasswordFilePath = ConvertTo-TomlPath $passwordFilePath
$tomlRedisSecretPath = ConvertTo-TomlPath $redisSecretPath

if ((-not (Test-Path $chatTomlPath)) -or $Force) {
    @"
[runtime]
environment = "production"
deployment_mode = "server"
app_code = "chat"

[server]
bind_address = "$BindAddress"
trust_forwarded_headers = true

[public_endpoints]
base_url = "$BaseUrl"
api_base_url = "$ApiBaseUrl"
websocket_base_url = "$WebsocketBaseUrl"
docs_base_url = "$BaseUrl/docs"

[paths]
config_directory = "$tomlConfigDir"
config_file = "$tomlChatConfigFile"
data_directory = "$tomlDataDir"
log_directory = "$tomlLogDir"
cache_directory = "$tomlDataDir/cache"
runtime_directory = "$tomlRunDir"

[database]
engine = "postgresql"
host = "127.0.0.1"
port = 5432
database = "sdkwork_ai_prod"
schema = "sdkwork_ai_prod"
username = "sdkwork_ai_prod"
password_file = "$tomlPasswordFilePath"
ssl_mode = "require"
max_connections = 20

[redis]
enabled = true
host = "redis.example.com"
port = 6379
database = 0
password_file = "$tomlRedisSecretPath"
key_prefix = "chat"
tls = false
max_connections = 16
"@ | Set-Content -Path $chatTomlPath -Encoding utf8
}

if ((-not (Test-Path $serverEnvPath)) -or $Force) {
    @"
SDKWORK_IM_DEPLOYMENT_MODE=server
SDKWORK_IM_CONFIG_FILE=$chatTomlPath
SDKWORK_IM_DATA_DIR=$DataDir
SDKWORK_IM_LOG_DIR=$LogDir
SDKWORK_IM_RUN_DIR=$RunDir
SDKWORK_IM_APPLICATION_PUBLIC_INGRESS_BIND=$BindAddress
SDKWORK_IM_APPLICATION_PUBLIC_HTTP_URL=$ApiBaseUrl
SDKWORK_IM_APPLICATION_PUBLIC_WEBSOCKET_URL=$WebsocketBaseUrl
SDKWORK_IM_PLATFORM_API_GATEWAY_HTTP_URL=http://127.0.0.1:3900
SDKWORK_IM_DATABASE_ENGINE=postgresql
SDKWORK_IM_DATABASE_HOST=127.0.0.1
SDKWORK_IM_DATABASE_PORT=5432
SDKWORK_CLAW_DATABASE_NAME=sdkwork_ai_prod
SDKWORK_CLAW_DATABASE_SCHEMA=sdkwork_ai_prod_chat_prod
SDKWORK_CLAW_DATABASE_USERNAME=sdkwork_ai_prod
SDKWORK_IM_DATABASE_PASSWORD_FILE=$passwordFilePath
SDKWORK_IM_DATABASE_SSL_MODE=require
SDKWORK_IM_DATABASE_MAX_CONNECTIONS=20
SDKWORK_IM_BROWSER_ORIGINS=$BrowserOrigins
"@ | Set-Content -Path $serverEnvPath -Encoding utf8
}

if ((-not (Test-Path $postgresqlPath)) -or $Force) {
    @"
provider: postgresql

connection:
  host: 127.0.0.1
  port: 5432
  database: sdkwork
  username: sdkwork
  passwordFile: "$passwordFilePath"
  sslmode: require
  applicationName: sdkwork-chat-server
  connectTimeoutSeconds: 10

schema:
  name: sdkwork
  provisioningMode: none
  migrationMode: apply
  expectedVersion: latest

pool:
  minConnections: 5
  maxConnections: 30
  idleTimeoutSeconds: 300
  maxLifetimeSeconds: 1800

# init-storage-server modes:
# - verify-only
# - bootstrap-schema
# - create-db-and-schema
"@ | Set-Content -Path $postgresqlPath -Encoding utf8
}

if ((-not (Test-Path $passwordFilePath)) -or $Force) {
    "replace-me" | Set-Content -Path $passwordFilePath -Encoding utf8
}

Write-Host "Rendered sdkwork-im-server configuration for instance '$InstanceName'."
Write-Host "chat.toml: $chatTomlPath"
Write-Host "server.env: $serverEnvPath"
Write-Host "postgresql.yaml: $postgresqlPath"
