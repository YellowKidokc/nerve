@echo off
title Nerve Sync API - Deploy
color 0B
echo.
echo  ============================================
echo   NERVE SYNC API - Cloudflare Worker Deploy
echo  ============================================
echo.

:: Check npm
where npm >nul 2>&1
if errorlevel 1 (
    echo  ERROR: npm not found. Install Node.js first.
    pause
    exit /b 1
)

:: Install deps
echo  [1/4] Installing dependencies...
call npm install
echo.

:: Check if KV namespace ID is set
findstr /C:"REPLACE_WITH" wrangler.toml >nul 2>&1
if not errorlevel 1 (
    echo  [2/4] Creating KV namespace...
    echo.
    echo  Run this command and copy the ID:
    echo    npx wrangler kv namespace create NERVE_KV
    echo.
    echo  Then edit wrangler.toml and replace REPLACE_WITH_YOUR_KV_NAMESPACE_ID
    echo  with the actual ID.
    echo.
    pause
    notepad wrangler.toml
    echo.
    echo  Did you update the ID? (Y/N)
    set /p UPDATED="> "
    if /i not "%UPDATED%"=="Y" (
        echo  Come back when it's updated.
        pause
        exit /b 1
    )
) else (
    echo  [2/4] KV namespace already configured.
)
echo.

:: Set API token secret
echo  [3/4] Setting API token secret...
echo  Pick a token (password) for the sync API.
echo  This same token goes in the Settings SYNC tab.
echo.
call npx wrangler secret put NERVE_API_TOKEN
echo.

:: Deploy
echo  [4/4] Deploying to Cloudflare...
echo.
call npx wrangler deploy
echo.

echo  ============================================
echo   DEPLOYED
echo  ============================================
echo.
echo  Your API is live. Copy the URL above and paste it
echo  into the Settings ^> SYNC ^> API URL field.
echo.
echo  Set the same token in Settings ^> SYNC ^> API Token.
echo.
pause
