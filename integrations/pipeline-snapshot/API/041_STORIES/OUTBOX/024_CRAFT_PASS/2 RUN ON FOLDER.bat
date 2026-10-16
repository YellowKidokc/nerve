@echo off
title 24 Craft Pass - on a folder
cd /d "%~dp0BACKSIDE"
echo.
set "TARGET=%~1"
if "%TARGET%"=="" set /p TARGET=Drag a folder or a paper here (or paste its path), then press Enter: 
if "%TARGET%"=="" exit /b 0
set "TARGET=%TARGET:"=%"
set "PY=python" & where py >nul 2>nul && set "PY=py -3"
set /p W=How many papers at once? (Enter = 3): 
if "%W%"=="" set W=3
%PY% 24_craft_pass.py "%TARGET%" --workers %W%
echo.
pause