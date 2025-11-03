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
cargo build --release
```

The binary will be at `target/release/rust-ed`

### Install

```bash
cargo install --path .
```

Or copy the binary manually:
```bash
sudo cp target/release/rust-ed /usr/local/bin/ed
```

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
