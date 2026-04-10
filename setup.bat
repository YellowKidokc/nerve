@echo off
title Nerve / ClipSync Setup
color 0E
echo.
echo  ============================================
echo   NERVE - ClipSync Agent Setup
echo  ============================================
echo.

:: Check we're in the right place
if not exist "source\Cargo.toml" (
    echo  ERROR: Run this from the _nerve_build folder.
    pause
    exit /b 1
)

:: ── Step 1: Build Release ──
echo  [1/4] Building release binary...
echo.
cd source
cargo build --release
if errorlevel 1 (
    echo.
    echo  BUILD FAILED. Fix errors above and re-run.
    pause
    exit /b 1
)
echo.
echo  Build OK.
echo.

:: ── Step 2: Install ──
echo  [2/4] Installing to %LOCALAPPDATA%\ClipSync ...
echo.

set INSTALLDIR=%LOCALAPPDATA%\ClipSync
if not exist "%INSTALLDIR%" mkdir "%INSTALLDIR%"
if not exist "%INSTALLDIR%\html" mkdir "%INSTALLDIR%\html"

copy /Y "target\release\nerve.exe" "%INSTALLDIR%\nerve.exe" >nul
xcopy /Y /E /Q "html\*" "%INSTALLDIR%\html\" >nul

echo  Installed to: %INSTALLDIR%
echo.

:: ── Step 3: Startup shortcut ──
echo  [3/4] Creating startup shortcut...
echo.

set SHORTCUT=%APPDATA%\Microsoft\Windows\Start Menu\Programs\Startup\ClipSync Agent.lnk

:: Use PowerShell to create .lnk
powershell -NoProfile -Command ^
  "$s=(New-Object -COM WScript.Shell).CreateShortcut('%SHORTCUT%');" ^
  "$s.TargetPath='%INSTALLDIR%\nerve.exe';" ^
  "$s.WorkingDirectory='%INSTALLDIR%';" ^
  "$s.Description='ClipSync Agent';" ^
  "$s.Save()"

echo  Shortcut: %SHORTCUT%
echo.

:: ── Step 4: Worker deploy (optional) ──
echo  [4/4] Cloudflare Worker deploy (optional)
echo.
cd ..

if not exist "worker\package.json" (
    echo  No worker folder found, skipping.
    goto :launch
)

echo  Deploy the sync API to Cloudflare? (Y/N)
set /p DEPLOY="> "
if /i "%DEPLOY%"=="Y" (
    echo.
    echo  Installing worker dependencies...
    cd worker
    call npm install
    echo.
    echo  IMPORTANT: Before deploying you need to:
    echo    1. Run: npx wrangler kv namespace create NERVE_KV
    echo    2. Copy the ID into wrangler.toml
    echo    3. Run: npx wrangler secret put NERVE_API_TOKEN
    echo    4. Then: npx wrangler deploy
    echo.
    echo  Opening a shell in the worker folder for you...
    start cmd /k "cd /d %CD% && echo Ready to deploy. Run the commands above."
    cd ..
) else (
    echo  Skipped worker deploy.
)

:launch
echo.
echo  ============================================
echo   SETUP COMPLETE
echo  ============================================
echo.
echo  Config:  %APPDATA%\clipsync-agent\config.json
echo  Binary:  %INSTALLDIR%\nerve.exe
echo  HTML:    %INSTALLDIR%\html\
echo  Worker:  worker\  (deploy with wrangler)
echo.
echo  Launch now? (Y/N)
set /p LAUNCH="> "
if /i "%LAUNCH%"=="Y" (
    echo  Starting ClipSync Agent...
    start "" "%INSTALLDIR%\nerve.exe"
)
echo.
echo  Done.
pause
