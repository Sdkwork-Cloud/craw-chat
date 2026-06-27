@echo off
setlocal

if not defined npm_node_execpath (
  echo npm_node_execpath is not defined. Run this command through npm or pnpm. 1>&2
  exit /b 1
)

"%npm_node_execpath%" %*
