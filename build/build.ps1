# RAIN.CHAT v2 Build Script for Windows
# This script builds the complete application and creates a distributable .exe file

param(
    [switch]$Debug = $false,
    [switch]$Clean = $false,
    [switch]$Help = $false
)

# Colors for output
$ErrorColor = "Red"
$SuccessColor = "Green"
$InfoColor = "Cyan"
$WarningColor = "Yellow"

function Write-ColorOutput {
    param(
        [string]$Message,
        [string]$Color = "White"
    )
    Write-Host $Message -ForegroundColor $Color
}

function Show-Help {
    Write-ColorOutput "RAIN.CHAT v2 Build Script" $InfoColor
    Write-ColorOutput "=========================" $InfoColor
    Write-ColorOutput ""
    Write-ColorOutput "Usage: .\build.ps1 [OPTIONS]" $InfoColor
    Write-ColorOutput ""
    Write-ColorOutput "Options:" $InfoColor
    Write-ColorOutput "  -Debug    Build in debug mode (faster, larger file)" $InfoColor
    Write-ColorOutput "  -Clean    Clean all build artifacts before building" $InfoColor
    Write-ColorOutput "  -Help     Show this help message" $InfoColor
    Write-ColorOutput ""
    Write-ColorOutput "Examples:" $InfoColor
    Write-ColorOutput "  .\build.ps1              # Build release version" $InfoColor
    Write-ColorOutput "  .\build.ps1 -Debug       # Build debug version" $InfoColor
    Write-ColorOutput "  .\build.ps1 -Clean       # Clean and build release" $InfoColor
}

if ($Help) {
    Show-Help
    exit 0
}

# Check if we're in the right directory
if (-not (Test-Path "package.json") -or -not (Test-Path "src-tauri")) {
    Write-ColorOutput "ERROR: This script must be run from the project root directory!" $ErrorColor
    Write-ColorOutput "Make sure you're in the directory containing package.json and src-tauri/" $ErrorColor
    exit 1
}

Write-ColorOutput "RAIN.CHAT v2 Build Process Starting..." $InfoColor
Write-ColorOutput "===========================================" $InfoColor

# Check prerequisites
Write-ColorOutput "Checking prerequisites..." $InfoColor

# Check Node.js
try {
    $nodeVersion = node --version
    Write-ColorOutput "Node.js: $nodeVersion" $SuccessColor
} catch {
    Write-ColorOutput "ERROR: Node.js not found! Please install Node.js 18+ from https://nodejs.org/" $ErrorColor
    exit 1
}

# Check Rust
try {
    $rustVersion = rustc --version
    Write-ColorOutput "Rust: $rustVersion" $SuccessColor
} catch {
    Write-ColorOutput "ERROR: Rust not found! Please install Rust from https://rustup.rs/" $ErrorColor
    exit 1
}

# Check Cargo
try {
    $cargoVersion = cargo --version
    Write-ColorOutput "Cargo: $cargoVersion" $SuccessColor
} catch {
    Write-ColorOutput "ERROR: Cargo not found! Please install Rust toolchain." $ErrorColor
    exit 1
}

# Clean if requested
if ($Clean) {
    Write-ColorOutput "Cleaning build artifacts..." $WarningColor
    
    # Clean Node.js
    if (Test-Path "node_modules") {
        Remove-Item -Recurse -Force "node_modules"
        Write-ColorOutput "  Removed node_modules/" $SuccessColor
    }
    
    if (Test-Path "dist") {
        Remove-Item -Recurse -Force "dist"
        Write-ColorOutput "  Removed dist/" $SuccessColor
    }
    
    # Clean Rust
    if (Test-Path "src-tauri/target") {
        Remove-Item -Recurse -Force "src-tauri/target"
        Write-ColorOutput "  Removed src-tauri/target/" $SuccessColor
    }
    
    Write-ColorOutput "Clean completed!" $SuccessColor
}

# Install Node.js dependencies
Write-ColorOutput "Installing Node.js dependencies..." $InfoColor
npm install
if ($LASTEXITCODE -ne 0) {
    Write-ColorOutput "ERROR: Failed to install Node.js dependencies!" $ErrorColor
    exit 1
}
Write-ColorOutput "Node.js dependencies installed!" $SuccessColor

# Build frontend
Write-ColorOutput "Building frontend..." $InfoColor
npm run build
if ($LASTEXITCODE -ne 0) {
    Write-ColorOutput "ERROR: Frontend build failed!" $ErrorColor
    exit 1
}
Write-ColorOutput "Frontend built successfully!" $SuccessColor

# Determine build mode
$buildMode = if ($Debug) { "debug" } else { "release" }
Write-ColorOutput "Building Tauri application in $buildMode mode..." $InfoColor

# Build Tauri app
if ($Debug) {
    npm run tauri:build -- --debug
} else {
    npm run tauri:build
}

if ($LASTEXITCODE -ne 0) {
    Write-ColorOutput "ERROR: Tauri build failed!" $ErrorColor
    exit 1
}

Write-ColorOutput "Tauri application built successfully!" $SuccessColor

# Find the built executable
$exePath = if ($Debug) {
    Get-ChildItem -Path "src-tauri/target/debug" -Name "rain-chat-v2.exe" -Recurse | Select-Object -First 1
} else {
    Get-ChildItem -Path "src-tauri/target/release" -Name "rain-chat-v2.exe" -Recurse | Select-Object -First 1
}

if ($exePath) {
    $fullPath = if ($Debug) {
        "src-tauri/target/debug/$exePath"
    } else {
        "src-tauri/target/release/$exePath"
    }
    
    $fileSize = [math]::Round((Get-Item $fullPath).Length / 1MB, 2)
    
    Write-ColorOutput "" $InfoColor
    Write-ColorOutput "BUILD COMPLETED SUCCESSFULLY!" $SuccessColor
    Write-ColorOutput "=================================" $SuccessColor
    Write-ColorOutput "Executable location: $fullPath" $InfoColor
    Write-ColorOutput "File size: $fileSize MB" $InfoColor
    Write-ColorOutput "Build mode: $buildMode" $InfoColor
    Write-ColorOutput "" $InfoColor
    Write-ColorOutput "You can now run your application by executing:" $InfoColor
    Write-ColorOutput "   .\$fullPath" $InfoColor
} else {
    Write-ColorOutput "ERROR: Could not locate the built executable!" $ErrorColor
    Write-ColorOutput "Check the build output above for errors." $ErrorColor
    exit 1
}

Write-ColorOutput "" $InfoColor
Write-ColorOutput "TIP: For distribution, you can also find installer files in:" $InfoColor
Write-ColorOutput "   src-tauri/target/$buildMode/bundle/" $InfoColor
