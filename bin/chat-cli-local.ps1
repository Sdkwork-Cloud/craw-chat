$ErrorActionPreference = 'Stop'

$release = $false
$cliArgs = @()

foreach ($argument in $args) {
    if ($argument -eq "--") {
        continue
    }

    if ($argument -eq "-Release" -or $argument -eq "/release" -or $argument -eq "--release") {
        $release = $true
        continue
    }

    $cliArgs += $argument
}

$root = Split-Path -Parent $PSScriptRoot
Set-Location $root

$profileDir = if ($release) { "release" } else { "debug" }
$exePath = Join-Path $root "target\$profileDir\sdkwork-im-cli.exe"

function Get-ChatCliBuildInputs {
    param(
        [Parameter(Mandatory = $true)]
        [string]$Root
    )

    @(
        (Join-Path $Root "Cargo.lock"),
        (Join-Path $Root "tools\chat-cli\Cargo.toml"),
        (Join-Path $Root "tools\chat-cli\src")
    )
}

function Test-ChatCliExecutableNeedsBuild {
    param(
        [Parameter(Mandatory = $true)]
        [string]$Root,
        [Parameter(Mandatory = $true)]
        [string]$ExePath
    )

    if (-not (Test-Path $ExePath)) {
        return $true
    }

    $exeTimestamp = (Get-Item -LiteralPath $ExePath).LastWriteTimeUtc
    foreach ($inputPath in Get-ChatCliBuildInputs -Root $Root) {
        if (-not (Test-Path -LiteralPath $inputPath)) {
            continue
        }

        $item = Get-Item -LiteralPath $inputPath
        if ($item.PSIsContainer) {
            $newerSource = Get-ChildItem -LiteralPath $inputPath -File -Recurse |
                Where-Object { $_.LastWriteTimeUtc -gt $exeTimestamp } |
                Select-Object -First 1
            if ($null -ne $newerSource) {
                return $true
            }
            continue
        }

        if ($item.LastWriteTimeUtc -gt $exeTimestamp) {
            return $true
        }
    }

    return $false
}

if (Test-ChatCliExecutableNeedsBuild -Root $root -ExePath $exePath) {
    $cargoArgs = @("build", "-p", "sdkwork-im-cli")
    if ($release) {
        $cargoArgs += "--release"
    }

    & cargo @cargoArgs
    if ($LASTEXITCODE -ne 0) {
        exit $LASTEXITCODE
    }
}

if (-not (Test-Path $exePath)) {
    throw "sdkwork-im-cli binary was not found after build: $exePath"
}

& $exePath @cliArgs
exit $LASTEXITCODE
