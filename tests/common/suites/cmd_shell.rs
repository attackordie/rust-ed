/// Test suite for the shell command (!)
/// GNU ed reference: main_loop.c case '!' (line 617)
///
/// The shell command executes a shell command.
/// Two modes:
/// - !command - executes shell command and returns to editor
/// - addr,addr!command - filters lines through shell command
/// Syntax: !command  or  [addr[,addr]]!command

use crate::common::{TestCase, TestSuite};

pub fn get_test_suite() -> TestSuite {
    let mut suite = TestSuite::new(
        "cmd_shell",
        "Shell command (!)"
    );

    // Execute shell command
    suite.add_test(TestCase::new(
        "shell_execute",
        "shell",
        "!echo hello\nq\n",
        "content\n"
    ));

    // Read from shell command output
    suite.add_test(TestCase::new(
        "shell_read_output",
        "shell",
        "r !echo new line\nw\nq\n",
        "existing\n"
    ));

    // Filter lines through shell command (CRITICAL - test #70 failing)
    suite.add_test(TestCase::new(
        "shell_filter_lines",
        "shell",
        "1,2!cat\nw\nq\n",
        "line 1\nline 2\nline 3\n"
    ));

    // Filter with sorting
    suite.add_test(TestCase::new(
        "shell_filter_sort",
        "shell",
        "%!sort\nw\nq\n",
        "zebra\napple\nbanana\n"
    ));

    suite
}
