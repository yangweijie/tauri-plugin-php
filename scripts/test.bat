@echo off
REM Tauri PHP Plugin Test Runner for Windows
REM This script runs all tests for the Tauri PHP plugin

setlocal enabledelayedexpansion

REM Function to print colored output (simplified for Windows)
echo [INFO] Starting Tauri PHP Plugin test suite...

REM Check prerequisites
echo [INFO] Checking prerequisites...

where cargo >nul 2>nul
if %errorlevel% neq 0 (
    echo [ERROR] Cargo is not installed. Please install Rust and Cargo.
    exit /b 1
)

where node >nul 2>nul
if %errorlevel% neq 0 (
    echo [ERROR] Node.js is not installed. Please install Node.js.
    exit /b 1
)

where npm >nul 2>nul
if %errorlevel% neq 0 (
    echo [ERROR] npm is not installed. Please install npm.
    exit /b 1
)

echo [SUCCESS] All prerequisites are installed.

REM Parse command line arguments
set RUN_RUST=true
set RUN_JS=true
set RUN_BENCHMARKS=false
set RUN_AUDIT=false
set RUN_COVERAGE=false

:parse_args
if "%1"=="--rust-only" (
    set RUN_JS=false
    shift
    goto parse_args
)
if "%1"=="--js-only" (
    set RUN_RUST=false
    shift
    goto parse_args
)
if "%1"=="--with-benchmarks" (
    set RUN_BENCHMARKS=true
    shift
    goto parse_args
)
if "%1"=="--with-audit" (
    set RUN_AUDIT=true
    shift
    goto parse_args
)
if "%1"=="--with-coverage" (
    set RUN_COVERAGE=true
    shift
    goto parse_args
)
if "%1"=="--all" (
    set RUN_BENCHMARKS=true
    set RUN_AUDIT=true
    set RUN_COVERAGE=true
    shift
    goto parse_args
)
if "%1"=="-h" goto show_help
if "%1"=="--help" goto show_help
if "%1"=="" goto start_tests
echo [ERROR] Unknown option: %1
exit /b 1

:show_help
echo Usage: %0 [OPTIONS]
echo Options:
echo   --rust-only       Run only Rust tests
echo   --js-only         Run only JavaScript tests
echo   --with-benchmarks Run benchmarks
echo   --with-audit      Run security audit
echo   --with-coverage   Generate coverage report
echo   --all             Run all tests including benchmarks, audit, and coverage
echo   -h, --help        Show this help message
exit /b 0

:start_tests

REM Run Rust tests
if "%RUN_RUST%"=="true" (
    echo [INFO] Running Rust tests...
    
    echo [INFO] Checking Rust code formatting...
    cargo fmt --all -- --check
    if %errorlevel% neq 0 (
        echo [ERROR] Rust code formatting issues found. Run 'cargo fmt' to fix.
        exit /b 1
    )
    echo [SUCCESS] Rust code formatting is correct.
    
    echo [INFO] Running Clippy...
    cargo clippy --all-targets --all-features -- -D warnings
    if %errorlevel% neq 0 (
        echo [ERROR] Clippy found issues.
        exit /b 1
    )
    echo [SUCCESS] Clippy checks passed.
    
    echo [INFO] Running Rust unit tests...
    cargo test --lib
    if %errorlevel% neq 0 (
        echo [ERROR] Rust unit tests failed.
        exit /b 1
    )
    echo [SUCCESS] Rust unit tests passed.
    
    echo [INFO] Running Rust integration tests...
    cargo test --test integration_tests
    if %errorlevel% neq 0 (
        echo [ERROR] Integration tests failed.
        exit /b 1
    )
    echo [SUCCESS] Integration tests passed.
    
    echo [INFO] Running framework detector tests...
    cargo test --test framework_detector_tests
    if %errorlevel% neq 0 (
        echo [ERROR] Framework detector tests failed.
        exit /b 1
    )
    echo [SUCCESS] Framework detector tests passed.
    
    echo [INFO] Running PHP binary tests...
    cargo test --test php_binary_tests
    if %errorlevel% neq 0 (
        echo [ERROR] PHP binary tests failed.
        exit /b 1
    )
    echo [SUCCESS] PHP binary tests passed.
    
    echo [INFO] Running PHP server tests...
    cargo test --test php_server_tests
    if %errorlevel% neq 0 (
        echo [ERROR] PHP server tests failed.
        exit /b 1
    )
    echo [SUCCESS] PHP server tests passed.
    
    echo [INFO] Running project manager tests...
    cargo test --test project_manager_tests
    if %errorlevel% neq 0 (
        echo [ERROR] Project manager tests failed.
        exit /b 1
    )
    echo [SUCCESS] Project manager tests passed.
    
    echo [INFO] Running documentation tests...
    cargo test --doc
    if %errorlevel% neq 0 (
        echo [ERROR] Documentation tests failed.
        exit /b 1
    )
    echo [SUCCESS] Documentation tests passed.
)

REM Run JavaScript tests
if "%RUN_JS%"=="true" (
    echo [INFO] Running JavaScript tests...
    
    cd guest-js
    
    echo [INFO] Installing JavaScript dependencies...
    npm ci
    if %errorlevel% neq 0 (
        echo [ERROR] Failed to install JavaScript dependencies.
        cd ..
        exit /b 1
    )
    echo [SUCCESS] JavaScript dependencies installed.
    
    echo [INFO] Running JavaScript unit tests...
    npm test
    if %errorlevel% neq 0 (
        echo [ERROR] JavaScript tests failed.
        cd ..
        exit /b 1
    )
    echo [SUCCESS] JavaScript tests passed.
    
    echo [INFO] Running JavaScript tests with coverage...
    npm run test:coverage
    if %errorlevel% neq 0 (
        echo [WARNING] JavaScript coverage tests failed, but continuing...
    ) else (
        echo [SUCCESS] JavaScript coverage tests passed.
    )
    
    echo [INFO] Building JavaScript package...
    npm run build
    if %errorlevel% neq 0 (
        echo [ERROR] JavaScript build failed.
        cd ..
        exit /b 1
    )
    echo [SUCCESS] JavaScript build successful.
    
    cd ..
)

REM Run benchmarks
if "%RUN_BENCHMARKS%"=="true" (
    echo [INFO] Running benchmarks...
    cargo bench
    if %errorlevel% neq 0 (
        echo [WARNING] Benchmarks failed, but continuing...
    ) else (
        echo [SUCCESS] Benchmarks completed.
    )
)

REM Run security audit
if "%RUN_AUDIT%"=="true" (
    echo [INFO] Running security audit...
    
    where cargo-audit >nul 2>nul
    if %errorlevel% equ 0 (
        echo [INFO] Running Rust security audit...
        cargo audit
        if %errorlevel% neq 0 (
            echo [WARNING] Rust security audit found issues.
        ) else (
            echo [SUCCESS] Rust security audit passed.
        )
    ) else (
        echo [WARNING] cargo-audit not installed. Install with: cargo install cargo-audit
    )
    
    echo [INFO] Running JavaScript security audit...
    cd guest-js
    npm audit --audit-level moderate
    if %errorlevel% neq 0 (
        echo [WARNING] JavaScript security audit found issues.
    ) else (
        echo [SUCCESS] JavaScript security audit passed.
    )
    cd ..
)

REM Generate coverage report
if "%RUN_COVERAGE%"=="true" (
    echo [INFO] Generating coverage report...
    
    where cargo-llvm-cov >nul 2>nul
    if %errorlevel% equ 0 (
        echo [INFO] Generating Rust coverage report...
        cargo llvm-cov --all-features --workspace --html
        if %errorlevel% neq 0 (
            echo [WARNING] Failed to generate Rust coverage report.
        ) else (
            echo [SUCCESS] Rust coverage report generated in target\llvm-cov\html\
        )
    ) else (
        echo [WARNING] cargo-llvm-cov not installed. Install with: cargo install cargo-llvm-cov
    )
)

echo [SUCCESS] All tests completed successfully!
exit /b 0
