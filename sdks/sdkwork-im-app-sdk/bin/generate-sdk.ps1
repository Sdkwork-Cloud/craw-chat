param(
  [string[]]$Languages = @("typescript", "flutter", "rust", "java", "csharp", "swift", "kotlin", "go", "python"),
  [string]$FixedSdkVersion,
  [string]$BaseUrl = "http://127.0.0.1:18079",
  [string]$SchemaUrl,
  [switch]$RefreshLive
)

$ErrorActionPreference = "Stop"
$ScriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$Arguments = @()

foreach ($Value in $Languages) {
  foreach ($Segment in ($Value -split ",")) {
    $Language = $Segment.Trim()
    if (-not [string]::IsNullOrWhiteSpace($Language)) {
      $Arguments += @("--language", $Language)
    }
  }
}

if (-not [string]::IsNullOrWhiteSpace($FixedSdkVersion)) {
  $Arguments += @("--fixed-sdk-version", $FixedSdkVersion.Trim())
}
if (-not [string]::IsNullOrWhiteSpace($BaseUrl)) {
  $Arguments += @("--base-url", $BaseUrl.Trim())
}
if (-not [string]::IsNullOrWhiteSpace($SchemaUrl)) {
  $Arguments += @("--schema-url", $SchemaUrl.Trim())
}
if ($RefreshLive) {
  $Arguments += "--refresh-live"
}

& node (Join-Path $ScriptDir "generate-sdk.mjs") @Arguments
if ($LASTEXITCODE -ne 0) {
  exit $LASTEXITCODE
}
