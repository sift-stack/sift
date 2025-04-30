@echo off
setlocal

REM Define target directory
set "TARGET_DIR=C:\Sift\Beckhoff"

REM Create target directory if it doesn't exist
if not exist "%TARGET_DIR%" (
    mkdir "%TARGET_DIR%"
)

REM Copy sift_proxy and the start script
copy "%~dp0sift_proxy.exe" "%TARGET_DIR%" >nul
copy "%~dp0start_sift_proxy.bat" "%TARGET_DIR%" >nul

REM Confirm success
echo Installation successful. Sift Proxy copied to %TARGET_DIR%
echo Press Enter to quit.
pause >nul

endlocal