@echo off
setlocal
rem Deep CKG on everything in INBOX: prepare (Clean MD / Prompts / Channel Summary, clean), X list, title what is not titled, deep CKG, answers onto each note, results flat in OUTBOX, then the layers.
rem Find _system by walking up, so this folder can be moved anywhere under API\.
set "SYS=%~dp0"
:findsys
if exist "%SYS%_system\engine\menu.py" goto :sysok
for %%I in ("%SYS%..") do set "UP=%%~fI"
if not "%UP:~-1%"=="\" set "UP=%UP%\"
if /i "%UP%"=="%SYS%" echo Cannot find the _system folder above %~dp0 & pause & exit /b 1
set "SYS=%UP%"
goto :findsys
:sysok
set "SYS=%SYS%_system\"
set "PY=python" & where py >nul 2>nul && set "PY=py -3"
%PY% "%SYS%engine\button.py" here "%~dp0."
pause
