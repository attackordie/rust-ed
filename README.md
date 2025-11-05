# rust-ed

Memory-safe, 100% drop-in replacement for GNU ed 1.22.2 written in Rust.

## Features

- ✅ **100% GNU ed 1.22.2 compatible** - Verified with 119 differential tests
- ✅ **Memory safe** - No buffer overflows, use-after-free, or undefined behavior
- ✅ **Zero unsafe code** - All core functionality in safe Rust
- ✅ **Drop-in replacement** - Identical command-line interface and behavior
- ✅ **Containerized testing** - Both implementations tested in identical environments

## Quick Start

### Build

```bash
# Linux (native)
cargo build --release

# Linux (static musl binary)
cargo build --release --target x86_64-unknown-linux-musl

# macOS
cargo build --release
```

The binary will be at `target/release/rust-ed`

### Installation

#### Option 1: Via Cargo (Local Install)

```bash
# Install to ~/.cargo/bin/rust-ed
cargo install --path .

# Add to PATH or create symlink
sudo ln -sf ~/.cargo/bin/rust-ed /usr/local/bin/ed
```

#### Option 2: Manual Binary Installation

**Linux:**
```bash
# Build static binary for maximum portability
cargo build --release --target x86_64-unknown-linux-musl

# Install to /usr/local/bin (preferred for user-managed binaries)
sudo cp target/x86_64-unknown-linux-musl/release/rust-ed /usr/local/bin/ed

# Or replace system ed (backup first!)
sudo mv /usr/bin/ed /usr/bin/ed.gnu.backup
sudo cp target/x86_64-unknown-linux-musl/release/rust-ed /usr/bin/ed
```

**macOS:**
```bash
# Build native macOS binary
cargo build --release

# Install to /usr/local/bin
sudo cp target/release/rust-ed /usr/local/bin/ed

# Note: macOS ships with BSD ed, not GNU ed
# rust-ed provides GNU ed 1.22.2 behavior on macOS
```

**Homebrew (Future):**
```bash
# When published to Homebrew
brew install rust-ed
brew link rust-ed
```

#### Option 3: From Crates.io (When Published)

```bash
# Install directly from crates.io
cargo install rust-ed

# Binary will be at ~/.cargo/bin/rust-ed
# Then symlink or copy to desired location
```

#### Option 4: Using Justfile Commands (Recommended for Development)

**Install rust-ed on your system:**
```bash
# Builds and installs rust-ed, automatically backs up GNU ed
just install-rust-ed

# This will:
# - Build the static musl binary
# - Backup /usr/bin/ed to /usr/bin/ed.gnu
# - Install rust-ed as /usr/bin/ed
```

**Restore GNU ed:**
```bash
# Restores GNU ed from backup
just install-gnu-ed

# This will:
# - Remove rust-ed from /usr/bin/ed
# - Restore /usr/bin/ed.gnu to /usr/bin/ed
```

**Check which ed is installed:**
```bash
# Shows which ed version is currently active
just which-ed

# Output shows:
# - Whether it's a symlink or regular file
# - Location and backup status
# - Version information
# - File type details
```

#### Verification

```bash
# Test the installation
echo "Hello, ed!" | ed

# Check version (shows compatibility with GNU ed 1.22.2)
ed --version

# Run simple editing test
echo -e "a\ntest line\n.\nw /tmp/test.txt\nq" | ed
cat /tmp/test.txt  # Should show "test line"
```

#### Swapping Between Implementations

During development and testing, you can easily swap between rust-ed and GNU ed:

```bash
# Install rust-ed for testing
just install-rust-ed

# Test your workflows with rust-ed
# ... run your scripts, tests, etc ...

# Switch back to GNU ed
just install-gnu-ed

# Compare behavior
# ... verify differences ...

# Check which one is active
just which-ed
```

This is particularly useful for:
- **Differential testing** - Compare behavior side-by-side
- **Regression testing** - Ensure rust-ed matches GNU ed exactly
- **Production validation** - Test real-world scripts with both implementations

### Platform Notes

#### Linux
- **GNU ed present:** Most distributions include GNU ed in base system
- **Replacement strategy:** rust-ed is a drop-in replacement
- **Static builds:** Use musl target for maximum portability across distros

#### macOS
- **BSD ed by default:** macOS ships with BSD ed (different from GNU ed)
- **No GNU ed:** GNU ed is not included in macOS by default
- **rust-ed provides:** Full GNU ed 1.22.2 behavior on macOS
- **Testing:** Install rust-ed to get GNU ed compatibility on Mac
- **Homebrew alternative:** `brew install ed` installs GNU ed (if you want C version)

#### iOS
- **No built-in ed:** iOS does not ship with any ed implementation
- **Not applicable:** iOS apps run in sandboxes, command-line tools not accessible
- **Jailbreak only:** Would require jailbroken device to use command-line ed

### Usage

`rust-ed` is a drop-in replacement for GNU ed:

```bash
rust-ed [file]
rust-ed -p '*' myfile.txt  # Prompt mode
rust-ed -s script.ed input.txt  # Script mode
```

All GNU ed commands and flags are supported.

## Testing

### Differential Testing Framework

rust-ed uses containerized differential testing to verify 100% compatibility with GNU ed 1.22.2. Both implementations run in identical Docker containers - the only difference is the binary (C vs Rust).

**Run all tests** (119 tests):
```bash
just test-drop-in-automated
```

**Test specific commands**:
```bash
just test-delete      # Test delete command (d)
just test-substitute  # Test substitute command (s)
just test-write       # Test write command (w, W)
```

See `justfile` for all available test commands.

### Prerequisites

- Docker (for building test containers)
- Rust 1.70+ with Cargo
- Just (command runner) - `cargo install just`

### Building Test Containers

```bash
# Build GNU ed reference container
docker build -f docker/Dockerfile.gnu-ed -t gnu-ed:latest .

# Build rust-ed container
docker build -f docker/Dockerfile.rust-ed -t rust-ed:latest .
```

The GNU ed container builds from the included source tarball (`gnu-ed-source/ed-1.22.2.tar`) ensuring reproducible test results.

## Architecture

rust-ed mirrors GNU ed's architecture while leveraging Rust's safety features:

- **buffer.rs** - Line buffer management with bounds checking
- **main_loop.rs** - Command execution loop
- **io.rs** - Safe file operations
- **regex.rs** - Pattern matching with `regex` crate
- **global.rs** - Global command implementation
- **error.rs** - GNU ed compatible error handling
- **signal.rs** - POSIX signal handling

## Testing Details

### Test Coverage

- **119 automated tests** covering all 32 GNU ed commands
- **100% command coverage** - Every GNU ed command tested
- **Symmetric containerized testing** - Both binaries in identical environments
- **Byte-for-byte output verification** - Exit codes, stdout, stderr, and file state

### Test Organization

```
tests/
├── differential_containerized.rs  # Main test runner
└── common/suites/                 # Test definitions (27 files)
    ├── cmd_append.rs              # 'a' command tests
    ├── cmd_delete.rs              # 'd' command tests
    ├── cmd_substitute.rs          # 's' command tests
    └── ...                        # One file per command
```

See `tests/README.md` for detailed testing documentation.

## License

MIT OR Apache-2.0

## Contributing

Contributions are welcome! Please ensure:

1. All tests pass: `just test-drop-in-automated`
2. Code follows rustfmt: `cargo fmt`
3. No clippy warnings: `cargo clippy`
4. Behavior matches GNU ed 1.22.2 exactly

## References

- GNU ed 1.22.2: https://www.gnu.org/software/ed/
- GNU ed Manual: https://www.gnu.org/software/ed/manual/ed_manual.html
- Target version info: `gnu-ed-source/VERSION.txt`

## Project Status

rust-ed is feature-complete and achieves 100% compatibility with GNU ed 1.22.2:

- ✅ All 32 commands implemented
- ✅ All address modes supported
- ✅ All command flags implemented
- ✅ Signal handling matches GNU ed
- ✅ Error handling matches GNU ed
- ✅ Exit codes match exactly

The project is ready for production use as a drop-in replacement for GNU ed.
