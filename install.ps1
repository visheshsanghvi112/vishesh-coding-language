# Vish Language Installer for Windows
# Created by Vishesh Sanghvi
# https://github.com/visheshsanghvi112/vishesh-coding-language

$ErrorActionPreference = "Stop"

$REPO = "visheshsanghvi112/vishesh-coding-language"

Write-Host ""
Write-Host "╔═══════════════════════════════════════════════════════════════╗" -ForegroundColor Green
Write-Host "║              🕉️  VISH LANGUAGE INSTALLER  🕉️                   ║" -ForegroundColor Green
Write-Host "║           Vedic on Steroids - by Vishesh Sanghvi              ║" -ForegroundColor Green
Write-Host "╚═══════════════════════════════════════════════════════════════╝" -ForegroundColor Green
Write-Host ""

# Determine install location
$InstallDir = "$env:USERPROFILE\.vish"
$BinPath = "$InstallDir\vish.exe"

# Create install directory
if (!(Test-Path $InstallDir)) {
    New-Item -ItemType Directory -Path $InstallDir -Force | Out-Null
}

Write-Host "Downloading Vish Language..." -ForegroundColor Yellow
$DownloadUrl = "https://github.com/$REPO/releases/latest/download/vish-win-x86_64.zip"

try {
    # Download
    $ZipPath = "$env:TEMP\vish.zip"
    Invoke-WebRequest -Uri $DownloadUrl -OutFile $ZipPath
    
    # Extract
    Write-Host "Extracting..." -ForegroundColor Yellow
    Expand-Archive -Path $ZipPath -DestinationPath $InstallDir -Force
    
    # Cleanup
    Remove-Item $ZipPath -Force
    
} catch {
    Write-Host "Error downloading: $_" -ForegroundColor Red
    Write-Host "Please download manually from: https://github.com/$REPO/releases" -ForegroundColor Yellow
    exit 1
}

# Add to PATH if not already there
$CurrentPath = [Environment]::GetEnvironmentVariable("Path", "User")
if ($CurrentPath -notlike "*$InstallDir*") {
    Write-Host "Adding Vish to PATH..." -ForegroundColor Yellow
    [Environment]::SetEnvironmentVariable("Path", "$CurrentPath;$InstallDir", "User")
    $env:Path = "$env:Path;$InstallDir"
}

Write-Host ""
Write-Host "╔═══════════════════════════════════════════════════════════════╗" -ForegroundColor Green
Write-Host "║              ✅ VISH LANGUAGE INSTALLED!                       ║" -ForegroundColor Green
Write-Host "╚═══════════════════════════════════════════════════════════════╝" -ForegroundColor Green
Write-Host ""
Write-Host "Installed to: $InstallDir" -ForegroundColor Cyan
Write-Host ""
Write-Host "To get started, open a NEW terminal and run:" -ForegroundColor Yellow
Write-Host "  vish --help" -ForegroundColor White
Write-Host ""
Write-Host "Create a file with .vish extension and run:" -ForegroundColor Yellow
Write-Host "  vish your_script.vish" -ForegroundColor White
Write-Host ""
Write-Host "Example:" -ForegroundColor Yellow
Write-Host '  echo "वद(""नमस्ते विश्व!"");" > hello.vish' -ForegroundColor White
Write-Host "  vish hello.vish" -ForegroundColor White
Write-Host ""
