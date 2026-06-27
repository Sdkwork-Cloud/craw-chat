$scriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path

if ($env:npm_node_execpath) {
  & $env:npm_node_execpath (Join-Path $scriptDir 'run-docs-task.mjs') @args
  exit $LASTEXITCODE
}

if ($env:NODE) {
  & $env:NODE (Join-Path $scriptDir 'run-docs-task.mjs') @args
  exit $LASTEXITCODE
}

node (Join-Path $scriptDir 'run-docs-task.mjs') @args
exit $LASTEXITCODE
