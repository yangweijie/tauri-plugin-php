#!/bin/bash

# Tauri PHP Plugin Test Runner
# This script runs all tests for the Tauri PHP plugin

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Function to print colored output
print_status() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Function to check if command exists
command_exists() {
    command -v "$1" >/dev/null 2>&1
}

# Check prerequisites
check_prerequisites() {
    print_status "Checking prerequisites..."
    
    if ! command_exists cargo; then
        print_error "Cargo is not installed. Please install Rust and Cargo."
        exit 1
    fi
    
    if ! command_exists node; then
        print_error "Node.js is not installed. Please install Node.js."
        exit 1
    fi
    
    if ! command_exists npm; then
        print_error "npm is not installed. Please install npm."
        exit 1
    fi
    
    print_success "All prerequisites are installed."
}

# Run Rust tests
run_rust_tests() {
    print_status "Running Rust tests..."
    
    # Format check
    print_status "Checking Rust code formatting..."
    if cargo fmt --all -- --check; then
        print_success "Rust code formatting is correct."
    else
        print_error "Rust code formatting issues found. Run 'cargo fmt' to fix."
        return 1
    fi
    
    # Clippy check
    print_status "Running Clippy..."
    if cargo clippy --all-targets --all-features -- -D warnings; then
        print_success "Clippy checks passed."
    else
        print_error "Clippy found issues."
        return 1
    fi
    
    # Unit tests
    print_status "Running Rust unit tests..."
    if cargo test --lib; then
        print_success "Rust unit tests passed."
    else
        print_error "Rust unit tests failed."
        return 1
    fi
    
    # Integration tests
    print_status "Running Rust integration tests..."
    if cargo test --test integration_tests; then
        print_success "Integration tests passed."
    else
        print_error "Integration tests failed."
        return 1
    fi
    
    # Framework detector tests
    print_status "Running framework detector tests..."
    if cargo test --test framework_detector_tests; then
        print_success "Framework detector tests passed."
    else
        print_error "Framework detector tests failed."
        return 1
    fi
    
    # PHP binary tests
    print_status "Running PHP binary tests..."
    if cargo test --test php_binary_tests; then
        print_success "PHP binary tests passed."
    else
        print_error "PHP binary tests failed."
        return 1
    fi
    
    # PHP server tests
    print_status "Running PHP server tests..."
    if cargo test --test php_server_tests; then
        print_success "PHP server tests passed."
    else
        print_error "PHP server tests failed."
        return 1
    fi
    
    # Project manager tests
    print_status "Running project manager tests..."
    if cargo test --test project_manager_tests; then
        print_success "Project manager tests passed."
    else
        print_error "Project manager tests failed."
        return 1
    fi
    
    # Doc tests
    print_status "Running documentation tests..."
    if cargo test --doc; then
        print_success "Documentation tests passed."
    else
        print_error "Documentation tests failed."
        return 1
    fi
}

# Run JavaScript tests
run_javascript_tests() {
    print_status "Running JavaScript tests..."
    
    cd guest-js
    
    # Install dependencies
    print_status "Installing JavaScript dependencies..."
    if npm ci; then
        print_success "JavaScript dependencies installed."
    else
        print_error "Failed to install JavaScript dependencies."
        cd ..
        return 1
    fi
    
    # Run tests
    print_status "Running JavaScript unit tests..."
    if npm test; then
        print_success "JavaScript tests passed."
    else
        print_error "JavaScript tests failed."
        cd ..
        return 1
    fi
    
    # Run tests with coverage
    print_status "Running JavaScript tests with coverage..."
    if npm run test:coverage; then
        print_success "JavaScript coverage tests passed."
    else
        print_warning "JavaScript coverage tests failed, but continuing..."
    fi
    
    # Build check
    print_status "Building JavaScript package..."
    if npm run build; then
        print_success "JavaScript build successful."
    else
        print_error "JavaScript build failed."
        cd ..
        return 1
    fi
    
    cd ..
}

# Run benchmarks
run_benchmarks() {
    print_status "Running benchmarks..."
    
    if cargo bench; then
        print_success "Benchmarks completed."
    else
        print_warning "Benchmarks failed, but continuing..."
    fi
}

# Run security audit
run_security_audit() {
    print_status "Running security audit..."
    
    # Rust security audit
    if command_exists cargo-audit; then
        print_status "Running Rust security audit..."
        if cargo audit; then
            print_success "Rust security audit passed."
        else
            print_warning "Rust security audit found issues."
        fi
    else
        print_warning "cargo-audit not installed. Install with: cargo install cargo-audit"
    fi
    
    # JavaScript security audit
    print_status "Running JavaScript security audit..."
    cd guest-js
    if npm audit --audit-level moderate; then
        print_success "JavaScript security audit passed."
    else
        print_warning "JavaScript security audit found issues."
    fi
    cd ..
}

# Generate coverage report
generate_coverage() {
    print_status "Generating coverage report..."
    
    if command_exists cargo-llvm-cov; then
        print_status "Generating Rust coverage report..."
        if cargo llvm-cov --all-features --workspace --html; then
            print_success "Rust coverage report generated in target/llvm-cov/html/"
        else
            print_warning "Failed to generate Rust coverage report."
        fi
    else
        print_warning "cargo-llvm-cov not installed. Install with: cargo install cargo-llvm-cov"
    fi
}

# Main function
main() {
    print_status "Starting Tauri PHP Plugin test suite..."
    
    # Parse command line arguments
    RUN_RUST=true
    RUN_JS=true
    RUN_BENCHMARKS=false
    RUN_AUDIT=false
    RUN_COVERAGE=false
    
    while [[ $# -gt 0 ]]; do
        case $1 in
            --rust-only)
                RUN_JS=false
                shift
                ;;
            --js-only)
                RUN_RUST=false
                shift
                ;;
            --with-benchmarks)
                RUN_BENCHMARKS=true
                shift
                ;;
            --with-audit)
                RUN_AUDIT=true
                shift
                ;;
            --with-coverage)
                RUN_COVERAGE=true
                shift
                ;;
            --all)
                RUN_BENCHMARKS=true
                RUN_AUDIT=true
                RUN_COVERAGE=true
                shift
                ;;
            -h|--help)
                echo "Usage: $0 [OPTIONS]"
                echo "Options:"
                echo "  --rust-only       Run only Rust tests"
                echo "  --js-only         Run only JavaScript tests"
                echo "  --with-benchmarks Run benchmarks"
                echo "  --with-audit      Run security audit"
                echo "  --with-coverage   Generate coverage report"
                echo "  --all             Run all tests including benchmarks, audit, and coverage"
                echo "  -h, --help        Show this help message"
                exit 0
                ;;
            *)
                print_error "Unknown option: $1"
                exit 1
                ;;
        esac
    done
    
    # Check prerequisites
    check_prerequisites
    
    # Run tests
    if [ "$RUN_RUST" = true ]; then
        if ! run_rust_tests; then
            print_error "Rust tests failed."
            exit 1
        fi
    fi
    
    if [ "$RUN_JS" = true ]; then
        if ! run_javascript_tests; then
            print_error "JavaScript tests failed."
            exit 1
        fi
    fi
    
    if [ "$RUN_BENCHMARKS" = true ]; then
        run_benchmarks
    fi
    
    if [ "$RUN_AUDIT" = true ]; then
        run_security_audit
    fi
    
    if [ "$RUN_COVERAGE" = true ]; then
        generate_coverage
    fi
    
    print_success "All tests completed successfully!"
}

# Run main function
main "$@"
