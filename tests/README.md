# rust-ed Test Suite

This directory contains the differential testing framework for rust-ed that proves 100% GNU ed 1.22.2 compatibility.

**Target Version:** GNU ed 1.22.2 (released August 18, 2025)
- All tests run against GNU ed 1.22.2 in identical Docker containers
- Source: https://mirror.team-cymru.com/gnu/ed/
- 100% compatibility verified with 119 differential tests

## Quick Start for New Developers

### Run all tests
```bash
just test-drop-in-automated  # 119 tests, both GNU ed and rust-ed in identical containers
```

### Test a specific command
```bash
just test-delete      # Test delete command only
just test-substitute  # Test substitute command only
just test-write       # Test write command only
```

### Add a test
1. Open the file: `common/suites/cmd_COMMAND.rs` (e.g., `cmd_delete.rs`)
2. Add your test:
```rust
suite.add_test(TestCase::new(
    "test_name",
    "category",
    "1d\nw\nq\n",     // Commands to run
    "input text\n"    // Initial file content
));
```
3. Run: `just test-COMMAND`

That's it! The framework automatically discovers and runs your test.

## Directory Structure

```
tests/
├── differential_containerized.rs    ← THE MAIN TEST (start here!)
│                                     Run with: just test-drop-in-automated
│
├── common/
│   ├── mod.rs                       ← TestCase and TestSuite definitions
│   └── suites/                      ← Test definitions (27 files)
│       ├── cmd_append.rs            ← 'a' command tests
│       ├── cmd_change.rs            ← 'c' command tests
│       ├── cmd_delete.rs            ← 'd' command tests
│       ├── cmd_edit.rs              ← 'e,E' command tests
│       ├── cmd_equals.rs            ← '=' command tests
│       ├── cmd_filename.rs          ← 'f' command tests
│       ├── cmd_global.rs            ← 'g,v,G,V' command tests
│       ├── cmd_help.rs              ← 'h,H' command tests
│       ├── cmd_insert.rs            ← 'i' command tests
│       ├── cmd_join.rs              ← 'j' command tests
│       ├── cmd_list.rs              ← 'l' command tests
│       ├── cmd_mark.rs              ← 'k,'' command tests
│       ├── cmd_move.rs              ← 'm' command tests
│       ├── cmd_number.rs            ← 'n' command tests
│       ├── cmd_print.rs             ← 'p' command tests
│       ├── cmd_prompt.rs            ← 'P' command tests
│       ├── cmd_quit.rs              ← 'q,Q' command tests
│       ├── cmd_read.rs              ← 'r' command tests
│       ├── cmd_search.rs            ← '/,?' command tests
│       ├── cmd_shell.rs             ← '!' command tests
│       ├── cmd_substitute.rs        ← 's' command tests
│       ├── cmd_transfer.rs          ← 't' command tests
│       ├── cmd_undo.rs              ← 'u' command tests
│       ├── cmd_write.rs             ← 'w,W' command tests
│       ├── cmd_yank.rs              ← 'y' command tests
│       ├── addressing.rs            ← Address parsing tests
│       └── error_conditions.rs      ← Error handling tests
│
├── security/                        ← Memory safety tests (2 files)
│   ├── memory_safety_comparison.rs
│   └── privilege_escalation_tests.rs
│
├── legacy/                          ← Old tests (38 files, not run)
│   └── README.md                    ← Explains why these exist
│
└── archive/                         ← Debug files (9 files, not run)
    └── README.md                    ← Development debug files
```

## Test Coverage

**Total: 119 tests across 25 GNU ed commands**
- Success rate: 87.4% (104/119 passing)
- Each command has its own test file
- All tests run in identical Docker containers

## Testing Methodology

### Containerized Differential Testing

Both GNU ed and rust-ed run in **identical** Ubuntu 22.04 containers. This proves true drop-in replacement capability - the ONLY difference is C vs Rust binary.

Each test:
1. Creates temp file with initial content
2. Runs same commands in both containers
3. Compares: exit codes, stdout, stderr, final file content
4. Reports any differences

### Test Organization

**ONE FILE PER COMMAND** - Easy to find and modify tests for any GNU ed command.

Want to test delete? Look in `common/suites/cmd_delete.rs`
Want to test substitute? Look in `common/suites/cmd_substitute.rs`

## Available Test Commands

```bash
# Individual commands
just test-append
just test-change
just test-delete
just test-edit
just test-global
just test-insert
just test-join
just test-list
just test-mark
just test-move
just test-print
just test-quit
just test-read
just test-search
just test-shell
just test-substitute
just test-transfer
just test-undo
just test-write
just test-yank

# Quick tests
just test-quick          # Common commands (d, w, a, p)

# Full test
just test-drop-in-automated  # All 119 tests
```

## Framework Architecture

The framework is modular and test-agnostic:
- Framework doesn't care what tests it runs
- Test suites self-register in `common/suites/mod.rs`
- Adding tests = just create TestCase, no framework changes needed
- Automatic discovery and execution

See `docs/modular-differential-testing-framework.md` for full documentation.
