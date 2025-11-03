# Test Suites - One File Per GNU ed Command

This directory contains test definitions for all GNU ed commands.
Each command has its own file following the pattern: `cmd_COMMAND.rs`

## Organization

One test file per GNU ed command (25 files total):

| File | Command | Description |
|------|---------|-------------|
| cmd_append.rs | a | Append text after line |
| cmd_change.rs | c | Change lines |
| cmd_delete.rs | d | Delete lines |
| cmd_edit.rs | e, E | Edit file |
| cmd_equals.rs | = | Show line number |
| cmd_filename.rs | f | Get/set filename |
| cmd_global.rs | g, v, G, V | Global commands |
| cmd_help.rs | h, H | Help |
| cmd_insert.rs | i | Insert text before line |
| cmd_join.rs | j | Join lines |
| cmd_list.rs | l | List with special chars visible |
| cmd_mark.rs | k, ' | Mark lines and goto marks |
| cmd_move.rs | m | Move lines |
| cmd_number.rs | n | Numbered print |
| cmd_print.rs | p | Print lines |
| cmd_prompt.rs | P | Toggle prompt |
| cmd_quit.rs | q, Q | Quit |
| cmd_read.rs | r | Read file |
| cmd_search.rs | /, ? | Search forward/backward |
| cmd_shell.rs | ! | Shell command |
| cmd_substitute.rs | s | Substitute text |
| cmd_transfer.rs | t | Transfer/copy lines |
| cmd_undo.rs | u | Undo last change |
| cmd_write.rs | w, W | Write to file |
| cmd_yank.rs | y | Yank lines to buffer |

Supporting test suites:
- addressing.rs - Address parsing (%, $, ., +, -, etc.)
- error_conditions.rs - Error handling tests

## Adding Tests

To add tests for a command:

1. Open the appropriate `cmd_COMMAND.rs` file
2. Add tests using `TestCase::new(name, category, commands, input)`
3. The framework automatically discovers and runs them

Example:
```rust
suite.add_test(TestCase::new(
    "delete_single_line",
    "delete",
    "1d\nw\nq\n",
    "line to delete\nline to keep\n"
));
```

## Running Tests

Test individual commands:
```bash
just test-delete      # Test only delete command
just test-substitute  # Test only substitute command
```

Test all commands:
```bash
just test-drop-in-automated  # Run all 119 tests
```
