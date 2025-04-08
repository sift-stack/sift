@echo off
setlocal enabledelayedexpansion

REM Set release tag
set TAG=v0.1.0-dev.1

REM Define variables
set REPO_OWNER=sift-stack
set REPO_NAME=sift
set BINARY_NAME=sift_proxy.exe
set INSTALL_DIR=C:\Sift-test
set DOWNLOAD_BINARY_NAME=sift_proxy.exe
set LATEST_RELEASE_API=https://api.github.com/repos/%REPO_OWNER%/%REPO_NAME%/releases

REM Check if sift_proxy is running
tasklist /FI "IMAGENAME eq %BINARY_NAME%" | find /I "%BINARY_NAME%" >nul
if %ERRORLEVEL%==0 (
    echo Error: sift_proxy is currently running. Please stop it before proceeding.
    echo You can stop it by running: taskkill /IM sift_proxy.exe /F
    exit /b 1
)

REM Use PowerShell to fetch release data and extract the download URL
for /f "delims=" %%A in ('powershell -NoProfile -Command ^
    "$tag = '%TAG%';" ^
    "$name = '%DOWNLOAD_BINARY_NAME%';" ^
    "$url = '%LATEST_RELEASE_API%';" ^
    "(Invoke-WebRequest -Uri $url).Content | ConvertFrom-Json |" ^
    "Where-Object { $_.tag_name -eq $tag } |" ^
    "ForEach-Object { $_.assets } |" ^
    "Where-Object { $_.name -eq $name } |" ^
    "Select-Object -ExpandProperty browser_download_url"') do (
    set DOWNLOAD_URL=%%A
)

REM Validate URL
if not defined DOWNLOAD_URL (
    echo Error: Could not find download URL for %DOWNLOAD_BINARY_NAME%.
    exit /b 1
)

echo Download URL: %DOWNLOAD_URL%
set TEMP_DOWNLOAD=%~dp0%BINARY_NAME%

REM Use PowerShell to download the binary
powershell -NoProfile -Command ^
    "Invoke-WebRequest -Uri '%DOWNLOAD_URL%' -OutFile '%TEMP_DOWNLOAD%'"

REM Verify download
if not exist "%TEMP_DOWNLOAD%" (
    echo Error: Download failed.
    exit /b 1
)

REM Create install directory
if not exist "%INSTALL_DIR%" (
    mkdir "%INSTALL_DIR%"
)

REM Move binary
move /Y "%TEMP_DOWNLOAD%" "%INSTALL_DIR%\%BINARY_NAME%" >nul

REM Confirm install
if exist "%INSTALL_DIR%\%BINARY_NAME%" (
    echo %BINARY_NAME% installed successfully to %INSTALL_DIR%.
) else (
    echo Error: Installation failed.
    exit /b 1
)

endlocal
