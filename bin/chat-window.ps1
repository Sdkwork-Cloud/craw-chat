param(
    [Alias("base-url")]
    [string]$BaseUrl,
    [Alias("tenant-id")]
    [string]$TenantId = "100001",
    [Alias("conversation-id")]
    [string]$ConversationId,
    [Alias("user-id")]
    [string]$UserId,
    [Alias("session-id")]
    [string]$SessionId,
    [Alias("device-id")]
    [string]$DeviceId,
    [Alias("bearer-token")]
    [string]$BearerToken,
    [string]$Login,
    [string]$Password,
    [string]$Label,
    [Alias("message-prefix")]
    [string]$MessagePrefix,
    [switch]$Release,
    [switch]$Help
)

$ErrorActionPreference = 'Stop'

function Read-ConfigValue {
    param(
        [Parameter(Mandatory = $true)]
        [string]$ConfigFile,
        [Parameter(Mandatory = $true)]
        [string]$Key
    )

    if (-not (Test-Path $ConfigFile)) {
        return $null
    }

    foreach ($line in Get-Content -Path $ConfigFile) {
        $trimmed = $line.Trim()
        if ($trimmed.Length -eq 0 -or $trimmed.StartsWith('#')) {
            continue
        }

        $parts = $trimmed -split '=', 2
        if ($parts.Count -eq 2 -and $parts[0].Trim() -eq $Key) {
            return $parts[1].Trim()
        }
    }

    return $null
}

function Resolve-BaseUrl {
    param(
        [string]$RequestedBaseUrl
    )

    if (-not [string]::IsNullOrWhiteSpace($RequestedBaseUrl)) {
        return $RequestedBaseUrl
    }

    $configFile = Join-Path (Split-Path -Parent $PSScriptRoot) "configs\topology\self-hosted.split-services.development.env"
    $httpUrl = Read-ConfigValue -ConfigFile $configFile -Key "SDKWORK_IM_APPLICATION_PUBLIC_HTTP_URL"
    if (-not [string]::IsNullOrWhiteSpace($httpUrl)) {
        return $httpUrl.TrimEnd('/')
    }
    $bindAddress = Read-ConfigValue -ConfigFile $configFile -Key "SDKWORK_IM_APPLICATION_PUBLIC_INGRESS_BIND"
    if ([string]::IsNullOrWhiteSpace($bindAddress)) {
        return "http://127.0.0.1:18079"
    }

    $segments = $bindAddress -split ':'
    $port = $segments[-1]
    $resolvedHost = ($segments[0..($segments.Length - 2)] -join ':').Trim()
    if ([string]::IsNullOrWhiteSpace($resolvedHost) -or $resolvedHost -eq "0.0.0.0" -or $resolvedHost -eq "::" -or $resolvedHost -eq "[::]") {
        $resolvedHost = "127.0.0.1"
    }
    return "http://$resolvedHost`:$port"
}

function Resolve-SeededImPassword {
    param(
        [Parameter(Mandatory = $true)]
        [string]$ResolvedLogin
    )

    switch ($ResolvedLogin) {
        "1" { return "Owner#2026" }
        "owner" { return "Owner#2026" }
        "2" { return "Guest#2026" }
        "guest" { return "Guest#2026" }
        "3" { return "Demo#2026" }
        "demo" { return "Demo#2026" }
        default { return $null }
    }
}

function Resolve-ImLogin {
    param(
        [Parameter(Mandatory = $true)]
        [string]$RequestedUserId,
        [string]$RequestedLogin
    )

    if (-not [string]::IsNullOrWhiteSpace($RequestedLogin)) {
        return $RequestedLogin
    }

    return $RequestedUserId
}

function Resolve-ImPassword {
    param(
        [Parameter(Mandatory = $true)]
        [string]$ResolvedLogin,
        [string]$RequestedPassword
    )

    if (-not [string]::IsNullOrWhiteSpace($RequestedPassword)) {
        return $RequestedPassword
    }

    $seededPassword = Resolve-SeededImPassword -ResolvedLogin $ResolvedLogin
    if (-not [string]::IsNullOrWhiteSpace($seededPassword)) {
        return $seededPassword
    }

    throw "No password was provided for login '$ResolvedLogin'. Supply -Login/-Password for non-seeded accounts."
}

function Quote-ProcessArgument {
    param(
        [AllowNull()]
        [string]$Value
    )

    if ($null -eq $Value) {
        return '""'
    }

    if ($Value -notmatch '[\s"]') {
        return $Value
    }

    return '"' + ($Value -replace '"', '\"') + '"'
}

function Invoke-ChatCliCaptured {
    param(
        [Parameter(Mandatory = $true)]
        [string[]]$Arguments,
        [switch]$AllowEmpty,
        [ValidateRange(1, 300)]
        [int]$TimeoutSeconds = 30
    )

    $root = Split-Path -Parent $PSScriptRoot
    $startInfo = New-Object System.Diagnostics.ProcessStartInfo
    $startInfo.FileName = "powershell.exe"
    $startInfo.WorkingDirectory = $root
    $startInfo.UseShellExecute = $false
    $startInfo.RedirectStandardOutput = $true
    $startInfo.RedirectStandardError = $true
    $startInfo.CreateNoWindow = $true
    $startInfo.StandardOutputEncoding = [System.Text.Encoding]::UTF8
    $startInfo.StandardErrorEncoding = [System.Text.Encoding]::UTF8

    $allArgs = @(
        "-NoProfile",
        "-ExecutionPolicy", "Bypass",
        "-File", (Join-Path $PSScriptRoot "chat-cli.ps1")
    )
    if ($Release) {
        $allArgs += "-Release"
    }
    $allArgs += $Arguments
    $startInfo.Arguments = (($allArgs | ForEach-Object { Quote-ProcessArgument $_ }) -join ' ')

    $process = New-Object System.Diagnostics.Process
    $process.StartInfo = $startInfo

    [void]$process.Start()
    $stdoutTask = $process.StandardOutput.ReadToEndAsync()
    $stderrTask = $process.StandardError.ReadToEndAsync()

    if (-not $process.WaitForExit($TimeoutSeconds * 1000)) {
        try {
            Stop-Process -Id $process.Id -Force -ErrorAction Stop
            $process.WaitForExit()
        }
        catch {
        }
        $stdout = $stdoutTask.GetAwaiter().GetResult()
        $stderr = $stderrTask.GetAwaiter().GetResult()
        throw "chat-cli invocation timed out after $TimeoutSeconds seconds: $($Arguments -join ' ')`n$stderr`n$stdout"
    }

    $process.WaitForExit()
    $stdout = $stdoutTask.GetAwaiter().GetResult()
    $stderr = $stderrTask.GetAwaiter().GetResult()
    $exitCode = if ($null -eq $process.ExitCode) { 0 } else { [int]$process.ExitCode }
    if ($exitCode -ne 0) {
        throw "chat-cli invocation failed with exit code ${exitCode}: $($Arguments -join ' ')`n$stderr`n$stdout"
    }

    if ([string]::IsNullOrWhiteSpace($stdout) -and -not $AllowEmpty) {
        throw "chat-cli invocation returned empty output: $($Arguments -join ' ')"
    }

    return [pscustomobject]@{
        Stdout = $stdout
        Stderr = $stderr
    }
}

function Invoke-ChatCliJson {
    param(
        [Parameter(Mandatory = $true)]
        [string[]]$Arguments
    )

    $result = Invoke-ChatCliCaptured -Arguments $Arguments
    return $result.Stdout | ConvertFrom-Json
}

function Invoke-ImUserLogin {
    param(
        [Parameter(Mandatory = $true)]
        [string]$ResolvedBaseUrl,
        [Parameter(Mandatory = $true)]
        [string]$RequestedUserId,
        [Parameter(Mandatory = $true)]
        [string]$ResolvedLogin,
        [Parameter(Mandatory = $true)]
        [string]$ResolvedPassword,
        [Parameter(Mandatory = $true)]
        [string]$ResolvedSessionId,
        [Parameter(Mandatory = $true)]
        [string]$ResolvedDeviceId
    )

    $tokenResponse = Invoke-ChatCliJson -Arguments @(
        "--base-url", $ResolvedBaseUrl,
        "--tenant-id", $TenantId,
        "--user-id", $RequestedUserId,
        "--session-id", $ResolvedSessionId,
        "--device-id", $ResolvedDeviceId,
        "token",
        "--token-only"
    )

    $accessToken = [string]$tokenResponse.token
    if ([string]::IsNullOrWhiteSpace($accessToken)) {
        throw "token response did not include token for '$ResolvedLogin'"
    }

    return [pscustomobject]@{
        UserId = $RequestedUserId
        SessionId = $ResolvedSessionId
        DeviceId = $ResolvedDeviceId
        BearerToken = $accessToken
        AuthMode = "real-login"
    }
}

function Resolve-ChatAuthContext {
    param(
        [Parameter(Mandatory = $true)]
        [string]$ResolvedBaseUrl,
        [Parameter(Mandatory = $true)]
        [string]$RequestedUserId,
        [Parameter(Mandatory = $true)]
        [string]$ResolvedSessionId,
        [Parameter(Mandatory = $true)]
        [string]$ResolvedDeviceId
    )

    if (-not [string]::IsNullOrWhiteSpace($BearerToken)) {
        return [pscustomobject]@{
            UserId = $RequestedUserId
            SessionId = $ResolvedSessionId
            DeviceId = $ResolvedDeviceId
            BearerToken = $BearerToken
            AuthMode = "provided-bearer"
        }
    }

    $resolvedLogin = Resolve-ImLogin -RequestedUserId $RequestedUserId -RequestedLogin $Login
    $seededPassword = Resolve-SeededImPassword -ResolvedLogin $resolvedLogin
    $shouldUseRealLogin = -not [string]::IsNullOrWhiteSpace($Login) `
        -or -not [string]::IsNullOrWhiteSpace($Password) `
        -or -not [string]::IsNullOrWhiteSpace($seededPassword)

    if ($shouldUseRealLogin) {
        $resolvedPassword = if (-not [string]::IsNullOrWhiteSpace($Password)) {
            $Password
        }
        else {
            Resolve-ImPassword -ResolvedLogin $resolvedLogin -RequestedPassword $Password
        }
        return Invoke-ImUserLogin `
            -ResolvedBaseUrl $ResolvedBaseUrl `
            -RequestedUserId $RequestedUserId `
            -ResolvedLogin $resolvedLogin `
            -ResolvedPassword $resolvedPassword `
            -ResolvedSessionId $ResolvedSessionId `
            -ResolvedDeviceId $ResolvedDeviceId
    }

    return [pscustomobject]@{
        UserId = $RequestedUserId
        SessionId = $ResolvedSessionId
        DeviceId = $ResolvedDeviceId
        BearerToken = $null
        AuthMode = "implicit-cli-default"
    }
}

if ($Help -or [string]::IsNullOrWhiteSpace($ConversationId) -or [string]::IsNullOrWhiteSpace($UserId)) {
    Write-Host "Usage: powershell -ExecutionPolicy Bypass -File bin/chat-window.ps1 -ConversationId <id> -UserId <id> [-BaseUrl <url>] [-TenantId <id>] [-SessionId <id>] [-DeviceId <id>] [-BearerToken <token>] [-Login <id>] [-Password <secret>] [-Label <name>] [-MessagePrefix <prefix>] [-Release]"
    Write-Host "Usage: cmd /c .\bin\chat-window.cmd --conversation-id <id> --user-id <id> [--base-url <url>] [--tenant-id <id>] [--session-id <id>] [--device-id <id>] [--bearer-token <token>] [--login <id>] [--password <secret>] [--label <name>] [--message-prefix <prefix>] [--release]"
    Write-Host "Open one interactive chat terminal backed by bin/chat-cli.ps1 chat-session. Default seeded IM users prefer real login before shared-secret fallback."
    if ($Help) {
        exit 0
    }
    exit 1
}

$resolvedLabel = if ([string]::IsNullOrWhiteSpace($Label)) { $UserId } else { $Label }
$resolvedBaseUrl = Resolve-BaseUrl -RequestedBaseUrl $BaseUrl
$resolvedSessionId = if ([string]::IsNullOrWhiteSpace($SessionId)) { "s_$UserId" } else { $SessionId }
$resolvedDeviceId = if ([string]::IsNullOrWhiteSpace($DeviceId)) { "d_$UserId" } else { $DeviceId }
$resolvedMessagePrefix = if ($PSBoundParameters.ContainsKey('MessagePrefix')) { $MessagePrefix } else { "[$resolvedLabel] " }
$resolvedAuthContext = Resolve-ChatAuthContext `
    -ResolvedBaseUrl $resolvedBaseUrl `
    -RequestedUserId $UserId `
    -ResolvedSessionId $resolvedSessionId `
    -ResolvedDeviceId $resolvedDeviceId

try {
    $host.UI.RawUI.WindowTitle = "sdkwork-im [$resolvedLabel] [$ConversationId]"
}
catch {
}

$cliArgs = @()
if ($Release) {
    $cliArgs += "-Release"
}
$cliArgs += @(
    "--base-url", $resolvedBaseUrl,
    "--tenant-id", $TenantId,
    "--user-id", $resolvedAuthContext.UserId,
    "--session-id", $resolvedAuthContext.SessionId,
    "--device-id", $resolvedAuthContext.DeviceId
)

if (-not [string]::IsNullOrWhiteSpace([string]$resolvedAuthContext.BearerToken)) {
    $cliArgs += @("--bearer-token", [string]$resolvedAuthContext.BearerToken)
}

$cliArgs += @(
    "chat-session",
    "--conversation-id", $ConversationId,
    "--label", $resolvedLabel
)

if (-not [string]::IsNullOrWhiteSpace($resolvedMessagePrefix)) {
    $cliArgs += @("--message-prefix", $resolvedMessagePrefix)
}

Write-Host "Opening chat session: conversation=$ConversationId user=$($resolvedAuthContext.UserId) label=$resolvedLabel baseUrl=$resolvedBaseUrl authMode=$($resolvedAuthContext.AuthMode)"
Write-Host "Type /quit to exit."

& "$PSScriptRoot\chat-cli.ps1" @cliArgs
exit $LASTEXITCODE
