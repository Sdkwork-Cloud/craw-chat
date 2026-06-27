@echo off
setlocal
call "%~dp0chat-cli-local.cmd" %*
exit /b %ERRORLEVEL%
