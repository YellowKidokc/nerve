@echo off
setlocal
rem Same as 1 RUN HERE, on any folder or note you point at (inside or outside API).
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
%PY% "%SYS%engine\button.py" folder "%~dp0."
pause
