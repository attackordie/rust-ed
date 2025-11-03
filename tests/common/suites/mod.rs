/// Test suite registry - one test file per GNU ed command
///
/// This module contains test definitions for all GNU ed commands.
/// Each command has its own file: cmd_COMMAND.rs
///
/// To add tests for a new command:
/// 1. Create common/suites/cmd_COMMAND.rs
/// 2. Implement get_test_suite() -> TestSuite
/// 3. Add the module declaration here
/// 4. Add it to get_all_test_suites()
/// 5. Create a test function in differential_containerized.rs

// Command-specific test suites (one file per GNU ed command - alphabetical)
pub mod cmd_append;      // a - append text after line
pub mod cmd_change;      // c - change lines
pub mod cmd_delete;      // d - delete lines
pub mod cmd_edit;        // e,E - edit file
pub mod cmd_equals;      // = - show line number
pub mod cmd_filename;    // f - get/set filename
pub mod cmd_global;      // g,v,G,V - global commands
pub mod cmd_help;        // h,H - help
pub mod cmd_insert;      // i - insert text before line
pub mod cmd_join;        // j - join lines
pub mod cmd_list;        // l - list with special chars visible
pub mod cmd_mark;        // k,' - mark lines
pub mod cmd_move;        // m - move lines
pub mod cmd_number;      // n - numbered print
pub mod cmd_print;       // p - print lines
pub mod cmd_prompt;      // P - toggle prompt
pub mod cmd_quit;        // q,Q - quit
pub mod cmd_read;        // r - read file
pub mod cmd_search;      // /,? - search forward/backward
pub mod cmd_shell;       // ! - shell command
pub mod cmd_substitute;  // s - substitute text
pub mod cmd_transfer;    // t - transfer/copy lines
pub mod cmd_undo;        // u - undo last change
pub mod cmd_write;       // w,W - write to file
pub mod cmd_yank;        // y - yank lines to buffer

// Supporting test suites
pub mod addressing;      // Address parsing (%, $, ., etc.)
pub mod error_conditions; // Error handling tests

use crate::common::{TestCase, TestSuite};

/// Get all available test suites
/// The framework will automatically run all tests from all suites
pub fn get_all_test_suites() -> Vec<TestSuite> {
    vec![
        // Command-specific test suites (alphabetical by command letter)
        cmd_append::get_test_suite(),
        cmd_change::get_test_suite(),
        cmd_delete::get_test_suite(),
        cmd_edit::get_test_suite(),
        cmd_equals::get_test_suite(),
        cmd_filename::get_test_suite(),
        cmd_global::get_test_suite(),
        cmd_help::get_test_suite(),
        cmd_insert::get_test_suite(),
        cmd_join::get_test_suite(),
        cmd_list::get_test_suite(),
        cmd_mark::get_test_suite(),
        cmd_move::get_test_suite(),
        cmd_number::get_test_suite(),
        cmd_print::get_test_suite(),
        cmd_prompt::get_test_suite(),
        cmd_quit::get_test_suite(),
        cmd_read::get_test_suite(),
        cmd_search::get_test_suite(),
        cmd_shell::get_test_suite(),
        cmd_substitute::get_test_suite(),
        cmd_transfer::get_test_suite(),
        cmd_undo::get_test_suite(),
        cmd_write::get_test_suite(),
        cmd_yank::get_test_suite(),

        // Supporting suites
        addressing::get_test_suite(),
        error_conditions::get_test_suite(),
    ]
}

/// Get test suites by name (for selective testing during development)
pub fn get_test_suites_by_name(names: &[&str]) -> Vec<TestSuite> {
    let all_suites = get_all_test_suites();

    if names.is_empty() {
        return all_suites;
    }

    all_suites
        .into_iter()
        .filter(|suite| names.contains(&suite.name.as_str()))
        .collect()
}
