# RAIN.CHAT v2 Build and Launch Script
# This script builds and launches the RAIN.CHAT v2 AI IDE

Write-Host "=== RAIN.CHAT v2 Build and Launch Script ===" -ForegroundColor Cyan
Write-Host ""

# Check if we're in the right directory
$currentDir = Get-Location
if (-not (Test-Path "src-tauri\Cargo.toml") -or -not (Test-Path "package.json")) {
    Write-Host "Error: Please run this script from the RAIN.CHAT v2 project root directory." -ForegroundColor Red
    Write-Host "Expected files: src-tauri\Cargo.toml, package.json" -ForegroundColor Red
    exit 1
}

Write-Host "Project directory: $currentDir" -ForegroundColor Green
Write-Host ""

# Step 1: Check prerequisites
Write-Host "1. Checking prerequisites..." -ForegroundColor Yellow

# Check Node.js
try {
    $nodeVersion = node --version
    Write-Host "  ✓ Node.js: $nodeVersion" -ForegroundColor Green
} catch {
    Write-Host "  ✗ Node.js not found. Please install Node.js 18+ from https://nodejs.org/" -ForegroundColor Red
    exit 1
}

# Check npm
try {
    $npmVersion = npm --version
    Write-Host "  ✓ npm: v$npmVersion" -ForegroundColor Green
} catch {
    Write-Host "  ✗ npm not found. Please install npm." -ForegroundColor Red
    exit 1
}

# Check Rust
try {
    $rustVersion = rustc --version
    Write-Host "  ✓ Rust: $rustVersion" -ForegroundColor Green
} catch {
    Write-Host "  ✗ Rust not found. Please install Rust from https://rustup.rs/" -ForegroundColor Red
    exit 1
}

# Check Cargo
try {
    $cargoVersion = cargo --version
    Write-Host "  ✓ Cargo: $cargoVersion" -ForegroundColor Green
} catch {
    Write-Host "  ✗ Cargo not found. Please install Rust toolchain." -ForegroundColor Red
    exit 1
}

Write-Host ""

# Step 2: Install dependencies
Write-Host "2. Installing dependencies..." -ForegroundColor Yellow

Write-Host "  Installing frontend dependencies..." -ForegroundColor Cyan
try {
    npm install
    Write-Host "  ✓ Frontend dependencies installed" -ForegroundColor Green
} catch {
    Write-Host "  ✗ Failed to install frontend dependencies" -ForegroundColor Red
    Write-Host "  Error: $_" -ForegroundColor Red
    exit 1
}

Write-Host ""

# Step 3: Check Rust compilation
Write-Host "3. Checking Rust compilation..." -ForegroundColor Yellow
try {
    Set-Location "src-tauri"
    cargo check
    Set-Location ".."
    Write-Host "  ✓ Rust code compiles successfully" -ForegroundColor Green
} catch {
    Set-Location ".."
    Write-Host "  ✗ Rust compilation failed" -ForegroundColor Red
    Write-Host "  Please check the Rust compilation errors above." -ForegroundColor Red
    Write-Host "  You may need to install additional dependencies or fix syntax errors." -ForegroundColor Yellow
    exit 1
}

Write-Host ""

# Step 4: Build and launch
Write-Host "4. Building and launching RAIN.CHAT v2..." -ForegroundColor Yellow
Write-Host "  This may take a few minutes on first run..." -ForegroundColor Cyan
Write-Host ""

try {
    # Try using npm script first
    if (Get-Command "npm" -ErrorAction SilentlyContinue) {
        Write-Host "  Launching with npm run tauri:dev..." -ForegroundColor Cyan
        npm run tauri:dev
    } else {
        Write-Host "  Launching with tauri dev..." -ForegroundColor Cyan
        tauri dev
    }
} catch {
    Write-Host "  ✗ Failed to launch application" -ForegroundColor Red
    Write-Host "  Error: $_" -ForegroundColor Red
    Write-Host ""
    Write-Host "Troubleshooting:" -ForegroundColor Yellow
    Write-Host "1. Make sure all dependencies are installed correctly" -ForegroundColor Yellow
    Write-Host "2. Check if Windows Defender or antivirus is blocking the build" -ForegroundColor Yellow
    Write-Host "3. Try running as administrator" -ForegroundColor Yellow
    Write-Host "4. Check the error messages above for specific issues" -ForegroundColor Yellow
    exit 1
}

Write-Host ""
Write-Host "=== Build Complete ===" -ForegroundColor Green
Write-Host "If the application doesn't open automatically, check for error messages above." -ForegroundColor Yellow