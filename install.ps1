# ClipSync Agent Installer
# Copies the release binary + HTML to AppData and creates a Startup shortcut

$installDir = "$env:LOCALAPPDATA\ClipSync"
$startupDir = "$env:APPDATA\Microsoft\Windows\Start Menu\Programs\Startup"
$exeName = "clipsync-agent.exe"

Write-Host "Installing ClipSync Agent..." -ForegroundColor Cyan

# Create install directory
New-Item -ItemType Directory -Force -Path $installDir | Out-Null
New-Item -ItemType Directory -Force -Path "$installDir\html" | Out-Null

# Copy files
Copy-Item "target\release\$exeName" "$installDir\$exeName" -Force
Copy-Item "html\*" "$installDir\html\" -Force -Recurse

Write-Host "  Installed to: $installDir" -ForegroundColor Green

# Create startup shortcut
$shortcutPath = "$startupDir\ClipSync Agent.lnk"
$shell = New-Object -ComObject WScript.Shell
$shortcut = $shell.CreateShortcut($shortcutPath)
$shortcut.TargetPath = "$installDir\$exeName"
$shortcut.WorkingDirectory = $installDir
$shortcut.Description = "ClipSync Agent - Clipboard sync, hotkeys, hotstrings"
$shortcut.Save()

Write-Host "  Startup shortcut: $shortcutPath" -ForegroundColor Green
Write-Host ""
Write-Host "ClipSync Agent installed! It will auto-start on login." -ForegroundColor Cyan
Write-Host "Config file: $env:APPDATA\clipsync-agent\config.json" -ForegroundColor Gray
Write-Host ""
Write-Host "Run now? (Press Enter to launch, Ctrl+C to skip)" -ForegroundColor Yellow
Read-Host

Start-Process "$installDir\$exeName"
