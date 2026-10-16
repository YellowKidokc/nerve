@echo off
title 24 Craft Pass
cd /d "%~dp0BACKSIDE"
echo.
echo  24 CRAFT PASS: every paper in INBOX (00_PRIORITY first). Papers that already have results are skipped.
echo  Output: OUTBOX\<paper> - CRAFT.md / .json and <paper> - BEST.md / .json
echo.
set "PY=python" & where py >nul 2>nul && set "PY=py -3"
set /p W=How many papers at once? (Enter = 3): 
if "%W%"=="" set W=3
%PY% 24_craft_pass.py --workers %W%
echo.
pause