@echo off
setlocal

if defined npm_node_execpath (
  "%npm_node_execpath%" "%~dp0run-docs-task.mjs" %*
  exit /b %errorlevel%
)

if defined NODE (
  "%NODE%" "%~dp0run-docs-task.mjs" %*
  exit /b %errorlevel%
)

node "%~dp0run-docs-task.mjs" %*
