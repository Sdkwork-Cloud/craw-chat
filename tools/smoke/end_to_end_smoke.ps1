param(
    [string]$BaseUrl = "http://127.0.0.1:18079"
)

$ErrorActionPreference = 'Stop'

$tenantId = "100001"
$userId = "1"
$actorKind = "user"
$sessionId = "s_demo"
$deviceId = "d_demo"
$permissionScope = @("automation.execute", "automation.read", "audit.read", "ops.read")

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

function Wait-Healthy {
    param([string]$Url)

    $healthUrl = "$Url/healthz"
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

$suffix = [DateTimeOffset]::UtcNow.ToUnixTimeSeconds()
$conversationId = "c_e2e_$suffix"
$executionId = "ae_e2e_$suffix"

$createConversationBody = @{
    conversationId = $conversationId
    conversationType = "group"
} | ConvertTo-Json -Depth 6

$null = Invoke-RestMethod -Method Post -Uri "$BaseUrl/im/v3/api/chat/conversations" -Headers $headers -Body $createConversationBody

$messageBody = @{
    clientMsgId = "e2e_client_$suffix"
    summary = "e2e hello"
    text = "e2e hello"
} | ConvertTo-Json -Depth 6

$null = Invoke-RestMethod -Method Post -Uri "$BaseUrl/im/v3/api/chat/conversations/$conversationId/messages" -Headers $headers -Body $messageBody

$notifications = Invoke-RestMethod -Method Get -Uri "$BaseUrl/app/v3/api/notifications" -Headers $headers
if ($notifications.items.Count -lt 1) {
    throw "Expected at least one notification"
}
if ($notifications.items[0].sourceEventType -ne "message.posted") {
    throw "Unexpected notification source event type"
}

$automationBody = @{
    executionId = $executionId
    triggerType = "webhook.manual"
    targetKind = "workflow"
    targetRef = "wf_e2e_demo"
    inputPayload = "{""conversationId"":""$conversationId""}"
} | ConvertTo-Json -Depth 6

$automation = Invoke-RestMethod -Method Post -Uri "$BaseUrl/app/v3/api/automation/executions" -Headers $headers -Body $automationBody
if ($automation.state -ne "succeeded") {
    throw "Unexpected automation execution state"
}

$auditExport = Invoke-RestMethod -Method Get -Uri "$BaseUrl/backend/v3/api/audit/export" -Headers $headers
if ($auditExport.total -lt 2) {
    throw "Expected at least two audit records"
}

$opsCluster = Invoke-RestMethod -Method Get -Uri "$BaseUrl/backend/v3/api/ops/cluster" -Headers $headers
if ($opsCluster.nodes[0].profile -ne "self-hosted.split-services.development") {
    throw "Unexpected ops cluster profile"
}

$opsDiagnostics = Invoke-RestMethod -Method Get -Uri "$BaseUrl/backend/v3/api/ops/diagnostics" -Headers $headers
if (-not $opsDiagnostics.nodeId) {
    throw "Missing ops diagnostic nodeId"
}

Write-Host "end-to-end smoke check passed."
