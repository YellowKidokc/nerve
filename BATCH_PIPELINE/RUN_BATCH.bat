@echo off
setlocal enabledelayedexpansion
title Consilience Atlas - Large Batch CKG Engine

echo ===============================================================================
echo     CONSILIENCE ATLAS: AUTONOMOUS LARGE-BATCH PIPELINE RUNNER
echo ===============================================================================
echo  Architecture:
echo    [00_INBOX] --^> [01_PROCESS] --^> [AI DEEPSEEK R1/V3.2] --^> [02_OUTBOX]
echo  - Source bytes preserved with SHA-256
echo  - Strict 14-heading CKG quality gate (no filler, no unproven Lean stubs)
echo  - Truth predicates extracted to structured ledger
echo  - Protected master records mirrored to Synology
echo ===============================================================================
echo.

cd /d "%~dp0"

echo [*] Checking Python environment...
python --version >nul 2>&1
if errorlevel 1 (
    echo [ERROR] Python is not found in PATH. Please install Python or add it to PATH.
    pause
    exit /b 1
)

echo [*] Launching batch runner...
echo.
python 03_SCRIPTS\run_batch.py %*

echo.
echo ===============================================================================
echo Batch process complete!
echo Master records: %~dp002_OUTBOX
echo Mirror backup:  C:\Users\David\Documents\faiththruphysics.com\999_API_CALLS_OUTPUT
echo ===============================================================================
echo.
pause
