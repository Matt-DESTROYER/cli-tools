@echo off
setlocal enabledelayedexpansion

:: --- Relaunch as admin if not already ---
net session >nul 2>&1
if %errorlevel% neq 0 (
    echo Requesting administrator privileges...
    powershell -Command "Start-Process '%~f0' -Verb RunAs -ArgumentList '%CD%'"
    exit /b
)

:: --- Script begins ---
:: If first argument passed, use it as the working directory
if not "%~1"=="" (
    cd /d "%~1"
)

set "TARGET_DIR=C:\Program Files\cli-tools"
set "CURRENT_DIR=%CD%"

echo.
echo ==== Creating CLI Tools folder ====
if not exist "%TARGET_DIR%" (
    mkdir "%TARGET_DIR%" 2>nul
    if errorlevel 1 (
        echo [!] Failed to create '%TARGET_DIR%'. Try running as Administrator.
        pause
        exit /b 1
    )
)

echo.
echo ==== Building all cargo projects ====

cd /d "%CURRENT_DIR%"

for /d %%D in (*) do (
    if exist "%%D\Cargo.toml" (
        echo.
        echo --- Building %%D ---
        pushd "%%D" >nul

        cargo build --release >"%TEMP%\cargo_build.log" 2>&1
        if errorlevel 1 (
            echo [!] Build failed in %%D, skipping...
            echo -------------------------------------------------
            type "%TEMP%\cargo_build.log"
            echo -------------------------------------------------
            popd >nul
            echo.
            goto :nextproject
        )

        del "%TEMP%\cargo_build.log" >nul 2>&1

        for %%F in (target\release\*.exe) do (
            echo "Moving %%~nxF to '%TARGET_DIR%'..."
            move /Y "%%F" "%TARGET_DIR%\" >nul
        )

        popd >nul
    ) else (
        echo Skipping %%D (no Cargo.toml found)
    )

    :nextproject
)

:: Add to PATH permanently if not already present
echo.
echo ==== Ensuring cli-tools is on PATH ====
echo %PATH% | find /I "C:\Program Files\cli-tools" >nul
if errorlevel 1 (
    echo Adding "C:\Program Files\cli-tools" to system PATH...
    setx PATH "%PATH%;C:\Program Files\cli-tools"
    set "PATH=%PATH%;C:\Program Files\cli-tools"
) else (
    echo Already on PATH.
)

echo.
echo Fin!
pause

