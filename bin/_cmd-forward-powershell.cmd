@echo off
setlocal EnableExtensions EnableDelayedExpansion

if "%~1"=="" (
    echo Missing PowerShell script path. 1>&2
    exit /b 1
)

set "_script=%~1"
shift

set "_args="

:arg_loop
if "%~1"=="" goto invoke

set "_normalized=%1"

if /I "%~1"=="/release" set "_normalized=-Release"
if /I "%~1"=="/foreground" set "_normalized=-Foreground"
if /I "%~1"=="/force" set "_normalized=-Force"
if /I "%~1"=="/help" set "_normalized=-Help"
if /I "%~1"=="-h" set "_normalized=-Help"
if /I "%~1"=="--help" set "_normalized=-Help"
if /I "%~1"=="/skipSmoke" set "_normalized=-SkipSmoke"
if /I "%~1"=="/skipsmoke" set "_normalized=-SkipSmoke"
if /I "%~1"=="--skip-smoke" set "_normalized=-SkipSmoke"
if /I "%~1"=="--skipSmoke" set "_normalized=-SkipSmoke"
if /I "%~1"=="/bindAddress" set "_normalized=-BindAddress"
if /I "%~1"=="/bindaddress" set "_normalized=-BindAddress"
if /I "%~1"=="--bind-addr" set "_normalized=-BindAddress"
if /I "%~1"=="--bindAddress" set "_normalized=-BindAddress"
if /I "%~1"=="/runtimeDir" set "_normalized=-RuntimeDir"
if /I "%~1"=="/runtimedir" set "_normalized=-RuntimeDir"
if /I "%~1"=="--runtime-dir" set "_normalized=-RuntimeDir"
if /I "%~1"=="--runtimeDir" set "_normalized=-RuntimeDir"
if /I "%~1"=="/backupDir" set "_normalized=-BackupDir"
if /I "%~1"=="/backupdir" set "_normalized=-BackupDir"
if /I "%~1"=="--backup-dir" set "_normalized=-BackupDir"
if /I "%~1"=="--backupDir" set "_normalized=-BackupDir"
if /I "%~1"=="/expectedPreviewFingerprint" set "_normalized=-ExpectedPreviewFingerprint"
if /I "%~1"=="/expectedpreviewfingerprint" set "_normalized=-ExpectedPreviewFingerprint"
if /I "%~1"=="--expected-preview-fingerprint" set "_normalized=-ExpectedPreviewFingerprint"
if /I "%~1"=="--expectedPreviewFingerprint" set "_normalized=-ExpectedPreviewFingerprint"
if /I "%~1"=="/retentionDays" set "_normalized=-RetentionDays"
if /I "%~1"=="/retentiondays" set "_normalized=-RetentionDays"
if /I "%~1"=="--retention-days" set "_normalized=-RetentionDays"
if /I "%~1"=="--retentionDays" set "_normalized=-RetentionDays"
if /I "%~1"=="/legalHold" set "_normalized=-LegalHold"
if /I "%~1"=="/legalhold" set "_normalized=-LegalHold"
if /I "%~1"=="--legal-hold" set "_normalized=-LegalHold"
if /I "%~1"=="--legalHold" set "_normalized=-LegalHold"
if /I "%~1"=="/json" set "_normalized=-Json"
if /I "%~1"=="--json" set "_normalized=-Json"
if /I "%~1"=="/profile" set "_normalized=-ProfileName"
if /I "%~1"=="/profilename" set "_normalized=-ProfileName"
if /I "%~1"=="--profile" set "_normalized=-ProfileName"
if /I "%~1"=="--profile-name" set "_normalized=-ProfileName"
if /I "%~1"=="--profileName" set "_normalized=-ProfileName"
if /I "%~1"=="/smokeBaseUrl" set "_normalized=-SmokeBaseUrl"
if /I "%~1"=="/smokebaseurl" set "_normalized=-SmokeBaseUrl"
if /I "%~1"=="--smoke-base-url" set "_normalized=-SmokeBaseUrl"
if /I "%~1"=="--smokeBaseUrl" set "_normalized=-SmokeBaseUrl"

if defined _args (
    set "_args=!_args! !_normalized!"
) else (
    set "_args=!_normalized!"
)

shift
goto arg_loop

:invoke
call powershell -NoProfile -ExecutionPolicy Bypass -File "%_script%" !_args!
set "_exit_code=%ERRORLEVEL%"
endlocal & exit /b %_exit_code%
