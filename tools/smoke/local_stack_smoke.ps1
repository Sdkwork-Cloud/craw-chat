param(
    [string]$BaseUrl = "http://127.0.0.1:18079"
)

$ErrorActionPreference = 'Stop'

$tenantId = "100001"
$userId = "1"
$actorKind = "user"
$sessionId = "s_demo"
$deviceId = "d_demo"
$permissionScope = @("chat.write")

function ConvertTo-Base64Url {
    param([Parameter(Mandatory = $true)][string]$Value)

    return ([Convert]::ToBase64String([System.Text.Encoding]::UTF8.GetBytes($Value))).TrimEnd('=').Replace('+', '-').Replace('/', '_')
}

function New-LocalJwt {
    param([Parameter(Mandatory = $true)][hashtable]$Claims)

    $header = ConvertTo-Base64Url -Value (@{ alg = "none"; typ = "JWT" } | ConvertTo-Json -Compress)
    $payload = ConvertTo-Base64Url -Value ($Claims | ConvertTo-Json -Depth 8 -Compress)
    return "$header.$payload.local"
}

$authToken = New-LocalJwt -Claims @{
    tenant_id = $tenantId
    login_scope = "TENANT"
    user_id = $userId
    session_id = $sessionId
    app_id = "sdkwork-im"
    auth_level = "password"
    subject_type = $actorKind
}

$accessToken = New-LocalJwt -Claims @{
    tenant_id = $tenantId
    login_scope = "TENANT"
    user_id = $userId
    session_id = $sessionId
    app_id = "sdkwork-im"
    environment = "dev"
    deployment_mode = "saas"
    auth_level = "password"
    actor_id = $userId
    actor_kind = $actorKind
    device_id = $deviceId
    data_scope = @("tenant")
    permission_scope = $permissionScope
    subject_type = $actorKind
}

$headers = @{
    "Authorization" = "Bearer $authToken"
    "Access-Token" = $accessToken
    "Content-Type" = "application/json"
}

$defaultBaseUrl = "http://127.0.0.1:18079"
$defaultHealthUrl = "http://127.0.0.1:18079/healthz"

function Wait-Healthy {
    param([string]$Url)

    $healthUrl = "$Url/healthz"
    if ($Url -eq $defaultBaseUrl) {
        $healthUrl = $defaultHealthUrl
    }
    for ($attempt = 0; $attempt -lt 20; $attempt++) {
        try {
            $response = Invoke-WebRequest -UseBasicParsing $healthUrl
            if ($response.StatusCode -eq 200) {
                return
            }
        }
        catch {
            Start-Sleep -Seconds 2
        }
    }

    throw "Timed out waiting for $Url/healthz"
}

Wait-Healthy -Url $BaseUrl

$conversationId = "c_smoke_$([DateTimeOffset]::UtcNow.ToUnixTimeSeconds())"

$createBody = @{
    conversationId = $conversationId
    conversationType = "group"
} | ConvertTo-Json -Depth 6

$null = Invoke-RestMethod -Method Post -Uri "$BaseUrl/im/v3/api/chat/conversations" -Headers $headers -Body $createBody

$messageBody = @{
    clientMsgId = "smoke_client"
    summary = "smoke"
    text = "smoke"
} | ConvertTo-Json -Depth 6

$null = Invoke-RestMethod -Method Post -Uri "$BaseUrl/im/v3/api/chat/conversations/$conversationId/messages" -Headers $headers -Body $messageBody

$summary = Invoke-RestMethod -Method Get -Uri "$BaseUrl/im/v3/api/chat/conversations/$conversationId" -Headers $headers
if ($summary.lastSummary -ne "smoke") {
    throw "Unexpected conversation summary payload"
}

Write-Host "local stack smoke check passed."
