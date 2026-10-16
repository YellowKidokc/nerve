@echo off
title 24 Craft Pass - repair finished papers
cd /d "%~dp0BACKSIDE"
echo.
echo  Re-applies the code-side guarantees to every finished paper in OUTBOX (no API calls):
echo  protected text restored word for word, dropped lines put back as ((KEPT)), repeated analogy worlds marked.
echo.
set "PY=python" & where py >nul 2>nul && set "PY=py -3"
%PY% 24_craft_pass.py --repair
echo.
pause