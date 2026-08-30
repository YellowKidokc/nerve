# Nerve Installer
# Copies the release binary + HTML to AppData and creates Start Menu,
# Desktop, and Startup shortcuts.
#
# Safe to re-run: it overwrites in place and reuses the same shortcut names,
# so repeated installs never leave duplicates behind.
#
# Non-interactive by default. Pass -Launch to start Nerve when it finishes.

param(
    [switch]$Launch,
    [switch]$NoStartup,
    [switch]$NoDesktop
)

$ErrorActionPreference = "Stop"

$installDir = "$env:LOCALAPPDATA\ClipSync"
$startMenu  = "$env:APPDATA\Microsoft\Windows\Start Menu\Programs"
$startupDir = "$startMenu\Startup"
$exeName    = "nerve.exe"
$source     = "target\release\$exeName"

Write-Host "Installing Nerve..." -ForegroundColor Cyan

if (-not (Test-Path $source)) {
    throw "Release binary not found at '$source'. Run 'cargo build --release' first."
}

# Nerve holds a single-instance lock and keeps its exe open while running.
# Stop it before copying, or the overwrite fails with a sharing violation.
$running = Get-Process nerve -ErrorAction SilentlyContinue
if ($running) {
    Write-Host "  Stopping running instance..." -ForegroundColor Gray
    $running | Stop-Process -Force
    Start-Sleep -Seconds 1
}

New-Item -ItemType Directory -Force -Path "$installDir\html" | Out-Null
Copy-Item $source "$installDir\$exeName" -Force
Copy-Item "html\*" "$installDir\html\" -Force -Recurse

Write-Host "  Installed to: $installDir" -ForegroundColor Green

# --- Shortcuts ------------------------------------------------------------
# One name, "Nerve", used everywhere. An older build installed a
# "ClipSync Agent" startup shortcut; remove it so the two do not both fire.
$legacy = "$startupDir\ClipSync Agent.lnk"
if (Test-Path $legacy) {
    Remove-Item $legacy -Force
    Write-Host "  Removed legacy startup shortcut" -ForegroundColor Gray
}

$shell = New-Object -ComObject WScript.Shell

function New-NerveShortcut {
    param([string]$Path, [string]$Label)
    $sc = $shell.CreateShortcut($Path)
    $sc.TargetPath = "$installDir\$exeName"
    $sc.WorkingDirectory = $installDir
    $sc.Description = "Nerve - selection capture, clipboard, canon workbench"
    $sc.Save()
    Write-Host "  $Label" -ForegroundColor Green
}

New-NerveShortcut "$startMenu\Nerve.lnk" "Start Menu entry"

if (-not $NoDesktop) {
    New-NerveShortcut "$env:USERPROFILE\Desktop\Nerve.lnk" "Desktop shortcut"
}

if (-not $NoStartup) {
    New-NerveShortcut "$startupDir\Nerve.lnk" "Starts automatically at login"
}

Write-Host ""
Write-Host "Nerve installed." -ForegroundColor Cyan
Write-Host "Config: $env:APPDATA\clipsync-agent\config.json" -ForegroundColor Gray
Write-Host ""
Write-Host "Hotkeys:" -ForegroundColor Gray
Write-Host "  Ctrl+Alt+J  selection toolbar     Ctrl+Alt+Q  claim capsule" -ForegroundColor DarkGray
Write-Host "  Ctrl+Alt+V  Stratum actions       Ctrl+Alt+U  canon workbench" -ForegroundColor DarkGray
Write-Host "  Ctrl+Alt+B  atom builder          Ctrl+Alt+N  reconciliation" -ForegroundColor DarkGray

if ($Launch) {
    Start-Process "$installDir\$exeName" -WorkingDirectory $installDir
    Write-Host ""
    Write-Host "Launched. Look for the tray icon." -ForegroundColor Cyan
}
