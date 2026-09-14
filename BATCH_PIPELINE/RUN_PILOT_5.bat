@echo off
setlocal enabledelayedexpansion
title Consilience Atlas - Pilot 5 Batch Runner

echo ===============================================================================
echo     CONSILIENCE ATLAS: 5-PAPER PILOT RUNNER (DEEPSEEK R1 / V3.2)
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

echo [*] Staging 5 pilot axioms from Vault into 00_INBOX...
copy "%VAULT_DIR%\0001_A1_1_Existence.md" "00_INBOX\" >nul
copy "%VAULT_DIR%\0002_A1_2_Distinction.md" "00_INBOX\" >nul
copy "%VAULT_DIR%\0003_A2_1_Substrate_Requirement.md" "00_INBOX\" >nul
copy "%VAULT_DIR%\0004_A2.2_A2_2_Self-Grounding.md" "00_INBOX\" >nul
copy "%VAULT_DIR%\0006_BC4_BC4_Three_Observers_Required.md" "00_INBOX\" >nul

echo [✓] 5 Pilot Axioms staged in 00_INBOX.
echo.
echo [*] Launching batch pipeline for 5 papers...
echo.
python 03_SCRIPTS\run_batch.py --limit 5

echo.
echo ===============================================================================
echo Pilot batch complete!
echo Master records: %~dp002_OUTBOX
echo Mirror backup:  C:\Users\David\Documents\faiththruphysics.com\999_API_CALLS_OUTPUT
echo ===============================================================================
echo.
pause
