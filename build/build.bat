@echo off
REM RAIN.CHAT v2 Simple Build Script for Windows
REM This script builds the complete application and creates a distributable .exe file

echo.
echo ========================================
echo    RAIN.CHAT v2 Build Process
echo ========================================
echo.

REM Check if we're in the right directory
if not exist "package.json" (
    echo ERROR: This script must be run from the project root directory!
    echo Make sure you're in the directory containing package.json and src-tauri/
    pause
    exit /b 1
)

if not exist "src-tauri" (
    echo ERROR: src-tauri directory not found!
    echo Make sure you're in the correct project directory.
    pause
    exit /b 1
)

echo [1/4] Installing Node.js dependencies...
call npm install
if errorlevel 1 (
    echo ERROR: Failed to install Node.js dependencies!
    pause
    exit /b 1
)

echo [2/4] Building frontend...
call npm run build
if errorlevel 1 (
    echo ERROR: Frontend build failed!
    pause
    exit /b 1
)

echo [3/4] Building Tauri application...
call npm run tauri:build
if errorlevel 1 (
    echo ERROR: Tauri build failed!
    pause
    exit /b 1
)

echo [4/4] Build completed successfully!
echo.

REM Find the built executable
for %%f in (src-tauri\target\release\rain-chat-v2.exe) do (
    if exist "%%f" (
        echo SUCCESS: Executable created at %%f
        echo File size: 
        dir "%%f" | findstr "rain-chat-v2.exe"
        echo.
        echo You can now run your application by executing:
        echo %%f
        goto :end
    )
)

for %%f in (src-tauri\target\debug\rain-chat-v2.exe) do (
    if exist "%%f" (
        echo SUCCESS: Executable created at %%f (DEBUG VERSION)
        echo File size: 
        dir "%%f" | findstr "rain-chat-v2.exe"
        echo.
        echo You can now run your application by executing:
        echo %%f
        goto :end
    )
)

echo ERROR: Could not locate the built executable!
echo Check the build output above for errors.

:end
echo.
echo Build process completed.
pause
