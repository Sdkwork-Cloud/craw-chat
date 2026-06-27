$ErrorActionPreference = 'Stop'

& "$PSScriptRoot\chat-cli-local.ps1" @args
exit $LASTEXITCODE
