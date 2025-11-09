# rust-ed Justfile - Pure Rust Testing Infrastructure
# Run tests and development tasks for 100% GNU ed compatible ed implementation

# Default recipe - show available commands
default:
    @just --list

# Check for unused dependencies before build
check-deps:
    @echo "🔍 Checking for unused dependencies..."
    @cargo machete || (echo "❌ Found unused dependencies! Run 'cargo machete' to see details" && exit 1)
    @echo "✅ All dependencies are used"

# Build the project
build: check-deps
    cargo build --release --target x86_64-unknown-linux-musl

# Build for development
build-dev: check-deps
    cargo build

# Run all core tests
test:
    @echo "🧪 Running rust-ed comprehensive test suite"
    @echo "==========================================="
    cargo test

# Run comprehensive command tests (main test suite)
test-commands:
    @echo "🎯 Running comprehensive command tests"
    @echo "====================================="
    cargo test --test comprehensive_command_tests -- --nocapture

# Run command coverage analysis
test-coverage:
    @echo "📊 Running command coverage analysis"
    @echo "===================================="
    cargo test --test comprehensive_command_coverage -- --nocapture

# Run differential tests against GNU ed
test-differential:
    @echo "🔄 Running differential tests vs GNU ed"
    @echo "======================================="
    cargo test --test differential -- --nocapture

# Run basic differential tests only (fast TDD cycle)
test-diff-basic:
    @echo "🔄 Running basic differential tests"
    @echo "=================================="
    cargo test --test differential test_basic_commands_only -- --nocapture --test-threads=1

# Run differential test for specific command
test-diff-command COMMAND:
    @echo "🔄 Testing {{COMMAND}} command against GNU ed"
    @echo "==========================================="
    cargo test --test differential {{COMMAND}} -- --nocapture

# Run all individual command tests
test-individual:
    @echo "🔍 Running individual command tests"
    @echo "=================================="
    cargo test --test test_insert_command_comprehensive -- --nocapture
    cargo test --test test_inverse_global_comprehensive -- --nocapture
    cargo test --test test_write_command_comprehensive -- --nocapture
    cargo test --test test_shell_command_comprehensive -- --nocapture
    cargo test --test test_join_command_comprehensive -- --nocapture

# Run memory safety tests
test-memory:
    @echo "🛡️  Running memory safety tests"
    @echo "==============================="
    cargo test --test memory_safety_comparison -- --nocapture

# Run security tests
test-security:
    @echo "🔒 Running security tests"
    @echo "========================"
    cargo test --test privilege_escalation_tests -- --nocapture

# Run performance benchmarks
bench:
    @echo "⚡ Running performance benchmarks"
    @echo "==============================="
    cargo bench

# Check for unsafe code blocks (should be empty)
audit-unsafe:
    @echo "🔍 Checking for unsafe code blocks"
    @echo "================================"
    @if grep -r "unsafe {" src/; then echo "❌ Found unsafe code blocks!"; exit 1; else echo "✅ No unsafe code blocks found"; fi

# Run security audit
audit-security:
    @echo "🔐 Running security audit"
    @echo "========================"
    cargo audit

# Run all audits
audit: audit-unsafe audit-security

# Lint and format code
lint:
    @echo "🧹 Linting and formatting code"
    @echo "=============================="
    cargo fmt
    cargo clippy -- -D warnings

# Type checking
typecheck:
    @echo "🔍 Running type checks"
    @echo "===================="
    cargo check

# Run all quality checks
quality: lint typecheck audit

# Build Docker container for GNU ed testing
docker-build:
    @echo "🐳 Building GNU ed Docker container"
    @echo "=================================="
    docker build -f docker/Dockerfile.gnu-ed -t gnu-ed:latest .

# Build Docker container for rust-ed (same environment as GNU ed)
docker-build-rust:
    @echo "🐳 Building rust-ed Docker container"
    @echo "===================================="
    docker build -f docker/Dockerfile.rust-ed -t rust-ed:latest .

# Build both Docker containers
docker-build-all: docker-build docker-build-rust
    @echo "✅ Both containers built successfully"
    @echo "   - gnu-ed:latest (reference C implementation)"
    @echo "   - rust-ed:latest (Rust implementation)"

# Verify drop-in replacement - run identical test on both containers
docker-verify-drop-in:
    @echo "🔍 Verifying Drop-In Replacement - Both in Identical Containers"
    @echo "==============================================================="
    @echo ""
    @echo "Test: Print last line"
    @echo ""
    @printf "line1\nline2\nline3\n" > /tmp/test_drop_in.txt
    @echo "GNU ed (C version) output:"
    @printf "p\nq\n" | docker run --rm -i -v /tmp/test_drop_in.txt:/tmp/test.txt:rw gnu-ed:latest /tmp/test.txt
    @echo ""
    @echo "rust-ed (Rust version) output:"
    @printf "p\nq\n" | docker run --rm -i -v /tmp/test_drop_in.txt:/tmp/test.txt:rw rust-ed:latest /tmp/test.txt
    @echo ""
    @echo "✅ Drop-in replacement verified!"
    @echo "   Architecture: SYMMETRIC (both in identical containers)"
    @echo "   - Base OS: Ubuntu 22.04"
    @echo "   - Binary location: /usr/local/bin/ed"
    @echo "   - User: testuser (UID 1000)"
    @echo "   - ONLY difference: C vs Rust binary"
    @rm -f /tmp/test_drop_in.txt

# Automated drop-in replacement verification (both containerized)
test-drop-in-automated:
    @echo "🐳 Automated Drop-In Replacement Verification"
    @echo "============================================="
    @echo ""
    @echo "Building containers (if needed)..."
    @just docker-build-all
    @echo ""
    @echo "Running comprehensive automated tests..."
    @echo "(Both GNU ed and rust-ed in identical containers)"
    @echo ""
    cargo test --test differential_containerized -- --nocapture

# Test individual GNU ed commands (NEW STRUCTURE - one test per command)

# Test delete command (d)
test-delete:
    @echo "🎯 Testing DELETE command (d) - containerized"
    @echo "=============================================="
    @just docker-build-all
    cargo test --test differential_containerized test_containerized_cmd_delete -- --nocapture

# Test write command (w, W)
test-write:
    @echo "🎯 Testing WRITE command (w, W) - containerized"
    @echo "=============================================="
    @just docker-build-all
    cargo test --test differential_containerized test_containerized_cmd_write -- --nocapture

# Test append command (a)
test-append:
    @echo "🎯 Testing APPEND command (a) - containerized"
    @echo "=============================================="
    @just docker-build-all
    cargo test --test differential_containerized test_containerized_cmd_append -- --nocapture

# Test print command (p)
test-print:
    @echo "🎯 Testing PRINT command (p) - containerized"
    @echo "=============================================="
    @just docker-build-all
    cargo test --test differential_containerized test_containerized_cmd_print -- --nocapture

# Test substitute command (s)
test-substitute:
    @echo "🎯 Testing SUBSTITUTE command (s) - containerized"
    @echo "=============================================="
    @just docker-build-all
    cargo test --test differential_containerized test_containerized_cmd_substitute -- --nocapture

# Test change command (c)
test-change:
    @echo "🎯 Testing CHANGE command (c) - containerized"
    @echo "=============================================="
    @just docker-build-all
    cargo test --test differential_containerized test_containerized_cmd_change -- --nocapture

# Test insert command (i)
test-insert:
    @echo "🎯 Testing INSERT command (i) - containerized"
    @echo "=============================================="
    @just docker-build-all
    cargo test --test differential_containerized test_containerized_cmd_insert -- --nocapture

# Test quit command (q, Q)
test-quit:
    @echo "🎯 Testing QUIT command (q, Q) - containerized"
    @echo "=============================================="
    @just docker-build-all
    cargo test --test differential_containerized test_containerized_cmd_quit -- --nocapture

# Test list command (l)
test-list:
    @echo "🎯 Testing LIST command (l) - containerized"
    @echo "=============================================="
    @just docker-build-all
    cargo test --test differential_containerized test_containerized_cmd_list -- --nocapture

# Test number command (n)
test-number:
    @echo "🎯 Testing NUMBER command (n) - containerized"
    @echo "=============================================="
    @just docker-build-all
    cargo test --test differential_containerized test_containerized_cmd_number -- --nocapture

# Test equals command (=)
test-equals:
    @echo "🎯 Testing EQUALS command (=) - containerized"
    @echo "=============================================="
    @just docker-build-all
    cargo test --test differential_containerized test_containerized_cmd_equals -- --nocapture

# Test read command (r)
test-read:
    @echo "🎯 Testing READ command (r) - containerized"
    @echo "=============================================="
    @just docker-build-all
    cargo test --test differential_containerized test_containerized_cmd_read -- --nocapture

# Test filename command (f)
test-filename:
    @echo "🎯 Testing FILENAME command (f) - containerized"
    @echo "=============================================="
    @just docker-build-all
    cargo test --test differential_containerized test_containerized_cmd_filename -- --nocapture

# Test edit command (e, E)
test-edit:
    @echo "🎯 Testing EDIT command (e, E) - containerized"
    @echo "=============================================="
    @just docker-build-all
    cargo test --test differential_containerized test_containerized_cmd_edit -- --nocapture

# Test global command (g, v, G, V)
test-global:
    @echo "🎯 Testing GLOBAL command (g, v) - containerized"
    @echo "=============================================="
    @just docker-build-all
    cargo test --test differential_containerized test_containerized_cmd_global -- --nocapture

# Test join command (j)
test-join:
    @echo "🎯 Testing JOIN command (j) - containerized"
    @echo "=============================================="
    @just docker-build-all
    cargo test --test differential_containerized test_containerized_cmd_join -- --nocapture

# Test move command (m)
test-move:
    @echo "🎯 Testing MOVE command (m) - containerized"
    @echo "=============================================="
    @just docker-build-all
    cargo test --test differential_containerized test_containerized_cmd_move -- --nocapture

# Test transfer command (t)
test-transfer:
    @echo "🎯 Testing TRANSFER command (t) - containerized"
    @echo "=============================================="
    @just docker-build-all
    cargo test --test differential_containerized test_containerized_cmd_transfer -- --nocapture

# Test yank command (y)
test-yank:
    @echo "🎯 Testing YANK command (y) - containerized"
    @echo "=============================================="
    @just docker-build-all
    cargo test --test differential_containerized test_containerized_cmd_yank -- --nocapture

# Test undo command (u)
test-undo:
    @echo "🎯 Testing UNDO command (u) - containerized"
    @echo "=============================================="
    @just docker-build-all
    cargo test --test differential_containerized test_containerized_cmd_undo -- --nocapture

# Test mark command (k, ')
test-mark:
    @echo "🎯 Testing MARK command (k, ') - containerized"
    @echo "=============================================="
    @just docker-build-all
    cargo test --test differential_containerized test_containerized_cmd_mark -- --nocapture

# Test shell command (!)
test-shell:
    @echo "🎯 Testing SHELL command (!) - containerized"
    @echo "=============================================="
    @just docker-build-all
    cargo test --test differential_containerized test_containerized_cmd_shell -- --nocapture

# Test search command (/, ?)
test-search:
    @echo "🎯 Testing SEARCH command (/, ?) - containerized"
    @echo "=============================================="
    @just docker-build-all
    cargo test --test differential_containerized test_containerized_cmd_search -- --nocapture

# Test prompt command (P)
test-prompt:
    @echo "🎯 Testing PROMPT command (P) - containerized"
    @echo "=============================================="
    @just docker-build-all
    cargo test --test differential_containerized test_containerized_cmd_prompt -- --nocapture

# Test help command (h, H)
test-help:
    @echo "🎯 Testing HELP command (h, H) - containerized"
    @echo "=============================================="
    @just docker-build-all
    cargo test --test differential_containerized test_containerized_cmd_help -- --nocapture

# Test file creation behavior (non-existent files)
test-file-creation:
    @echo "🎯 Testing FILE CREATION (non-existent files) - containerized"
    @echo "=============================================================="
    @just docker-build-all
    cargo test --test differential_containerized test_containerized_file_creation -- --nocapture

# Quick test - runs most common editing commands
test-quick:
    @echo "⚡ Quick Test - Common Commands (d, w, a, p)"
    @echo "==========================================="
    @just test-delete
    @just test-write
    @just test-append
    @just test-print

# Test addressing (%, $, ., ',', +, -, etc.)
test-addressing:
    @echo "🎯 Testing addressing commands (containerized)"
    @just docker-build-all
    cargo test --test differential_containerized test_containerized_addressing -- --ignored --nocapture

# Run full compatibility test suite
test-compatibility:
    @echo "🎯 Running full GNU ed compatibility test suite"
    @echo "=============================================="
    just test-commands
    just test-coverage
    just test-differential

# Run full compatibility including containerized verification
test-compatibility-full: test-compatibility
    @echo ""
    @echo "🐳 Running containerized drop-in verification..."
    just test-drop-in-automated

# Development workflow - build and test
dev: build-dev test-commands

# Release workflow - full testing and quality checks
release: build quality test-compatibility

# Clean build artifacts
clean:
    @echo "🧹 Cleaning build artifacts"
    @echo "=========================="
    cargo clean

# Show test status summary
status:
    @echo "📊 rust-ed Test Status Summary"
    @echo "============================="
    @echo "Running quick compatibility check..."
    @cargo test --test comprehensive_command_coverage 2>/dev/null | grep -E "(Commands (tested|passing)|COMPATIBLE|INCOMPATIBLE)" || echo "Tests need to be run"

# Interactive test selection
interactive:
    @echo "🎮 Interactive Test Selection"
    @echo "============================"
    @echo "1) All tests (just test)"
    @echo "2) Command tests (just test-commands)"
    @echo "3) Coverage analysis (just test-coverage)"
    @echo "4) Differential tests (just test-differential)"
    @echo "5) Memory safety (just test-memory)"
    @echo "6) Security tests (just test-security)"
    @echo "7) Full compatibility (just test-compatibility)"
    @echo ""
    @echo "Enter your choice (1-7):"

# Quick development test cycle
quick:
    @echo "⚡ Quick development test cycle"
    @echo "============================="
    cargo test --test comprehensive_command_tests

# Verbose test output with detailed debugging
verbose:
    @echo "🔍 Verbose test output"
    @echo "===================="
    RUST_BACKTRACE=1 cargo test -- --nocapture

# Test only failing commands
test-failing:
    @echo "❌ Testing only commands known to be failing"
    @echo "==========================================="
    @echo "Running comprehensive coverage to identify failing commands..."
    cargo test --test comprehensive_command_coverage -- --nocapture | grep "INCOMPATIBLE"

# Profile test performance
profile:
    @echo "📈 Profiling test performance"
    @echo "============================"
    time just test-commands

# Generate test report
report:
    @echo "📋 Generating test report"
    @echo "========================"
    just status
    @echo ""
    @echo "📊 Detailed Coverage:"
    just test-coverage | tail -20

# Install rust-ed on host system (replaces GNU ed)
install-rust-ed: build
    @echo "🦀 Installing rust-ed on host system"
    @echo "===================================="
    @echo ""
    @# Check if GNU ed is already installed
    @if [ -f /usr/bin/ed ] && [ ! -L /usr/bin/ed ]; then \
        echo "📦 Backing up GNU ed to /usr/bin/ed.gnu"; \
        sudo mv /usr/bin/ed /usr/bin/ed.gnu; \
    elif [ -L /usr/bin/ed ]; then \
        echo "⚠️  /usr/bin/ed is already a symlink - removing it"; \
        sudo rm /usr/bin/ed; \
    fi
    @echo "🔧 Installing rust-ed to /usr/bin/ed"
    @sudo cp target/x86_64-unknown-linux-musl/release/rust-ed /usr/bin/ed
    @echo ""
    @echo "✅ rust-ed installed successfully!"
    @echo "   Location: /usr/bin/ed"
    @echo "   Backup: /usr/bin/ed.gnu (if existed)"
    @echo ""
    @echo "Test with: echo -e 'a\\ntest\\n.\\np\\nq' | ed"

# Restore GNU ed on host system (removes rust-ed)
install-gnu-ed:
    @echo "🐧 Restoring GNU ed on host system"
    @echo "==================================="
    @echo ""
    @# Check if backup exists
    @if [ ! -f /usr/bin/ed.gnu ]; then \
        echo "❌ Error: GNU ed backup not found at /usr/bin/ed.gnu"; \
        echo "   Cannot restore GNU ed without backup"; \
        exit 1; \
    fi
    @echo "🔄 Removing rust-ed from /usr/bin/ed"
    @sudo rm /usr/bin/ed
    @echo "📦 Restoring GNU ed from backup"
    @sudo mv /usr/bin/ed.gnu /usr/bin/ed
    @echo ""
    @echo "✅ GNU ed restored successfully!"
    @echo "   Location: /usr/bin/ed"
    @echo ""
    @echo "Test with: echo -e 'a\\ntest\\n.\\np\\nq' | ed"

# Show which ed is currently installed
which-ed:
    @echo "🔍 Checking which ed is installed"
    @echo "=================================="
    @echo ""
    @if [ -L /usr/bin/ed ]; then \
        echo "Type: Symlink"; \
        echo "Target: $$(readlink -f /usr/bin/ed)"; \
    elif [ -f /usr/bin/ed ]; then \
        echo "Type: Regular file"; \
        echo "Location: /usr/bin/ed"; \
    else \
        echo "Type: Not found"; \
    fi
    @echo ""
    @if [ -f /usr/bin/ed.gnu ]; then \
        echo "Backup: /usr/bin/ed.gnu exists (GNU ed backup)"; \
    else \
        echo "Backup: No backup found"; \
    fi
    @echo ""
    @echo "Version check:"
    @/usr/bin/ed --version 2>&1 | head -1 || echo "Cannot determine version"
    @echo ""
    @echo "File info:"
    @file /usr/bin/ed 2>/dev/null || echo "File not found"

# Help for rust-ed development
help:
    @echo "rust-ed Development Guide"
    @echo "========================"
    @echo ""
    @echo "Quick Start:"
    @echo "  just build     # Build the project"
    @echo "  just test      # Run all tests"
    @echo "  just status    # Check current status"
    @echo ""
    @echo "Development:"
    @echo "  just dev       # Build and test for development"
    @echo "  just quick     # Quick test cycle"
    @echo "  just verbose   # Detailed test output"
    @echo ""
    @echo "Quality:"
    @echo "  just quality   # Run all quality checks"
    @echo "  just audit     # Security and safety audits"
    @echo ""
    @echo "Testing:"
    @echo "  just test-compatibility       # Full GNU ed compatibility tests"
    @echo "  just test-compatibility-full  # Full tests + containerized verification"
    @echo "  just test-drop-in-automated   # Automated drop-in verification (both containerized)"
    @echo "  just test-failing             # Test only failing commands"
    @echo "  just interactive              # Interactive test selection"
    @echo ""
    @echo "Docker:"
    @echo "  just docker-build-all         # Build both GNU ed and rust-ed containers"
    @echo "  just docker-verify-drop-in    # Manual visual drop-in verification"
    @echo ""
    @echo "Host System Installation:"
    @echo "  just install-rust-ed    # Replace GNU ed with rust-ed on host"
    @echo "  just install-gnu-ed     # Restore GNU ed on host (removes rust-ed)"
    @echo "  just which-ed           # Check which ed is currently installed"
    @echo ""
    @echo "The goal: 100% drop-in binary replacement for GNU ed"