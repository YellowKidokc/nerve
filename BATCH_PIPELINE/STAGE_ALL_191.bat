@echo off
setlocal enabledelayedexpansion
title Consilience Atlas - Stage All 191 Axioms

echo ===============================================================================
echo     CONSILIENCE ATLAS: STAGE ALL 191 AXIOMS INTO 00_INBOX
echo ===============================================================================
echo.

cd /d "%~dp0"

set "VAULT_DIR=Z:\Theophysics_Vault\00_REFINED_PAPERS_MASTER\05_Axiom_Nodes_191"

if not exist "%VAULT_DIR%" (
    echo [ERROR] Vault directory not found: %VAULT_DIR%
    echo Please verify Z: drive is connected.
    pause
    exit /b 1
)

echo [*] Copying all 191 axiom papers from Vault to 00_INBOX...
copy "%VAULT_DIR%\*.md" "00_INBOX\" >nul

for /f %%A in ('dir /b /a-d "00_INBOX\*.md" 2^>nul ^| find /c /v ""') do set COUNT=%%A

echo.
echo [✓] Successfully staged %COUNT% papers in 00_INBOX!
echo.
echo To run the full batch, double-click RUN_BATCH.bat.
echo.
pause
