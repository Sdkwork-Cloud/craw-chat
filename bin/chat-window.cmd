@echo off
setlocal
call powershell -NoProfile -ExecutionPolicy Bypass -File "%~dp0chat-window.ps1" %*
exit /b %ERRORLEVEL%
