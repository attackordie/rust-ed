/// Test suite for the read command (r)
/// GNU ed reference: main_loop.c case 'r' (line 670)
///
/// The read command reads a file and appends it after the addressed line.
/// Syntax: [addr]r [filename]
/// Can also read from shell command: r !command

use crate::common::{TestCase, TestSuite};

pub fn get_test_suite() -> TestSuite {
    let mut suite = TestSuite::new(
        "cmd_read",
        "Read command (r)"
    );

    // Read non-existent file (should error)
    suite.add_test(TestCase::new(
        "read_file",
        "read",
        "r /tmp/test_read_input.txt\nw\nq\n",
        "existing content\n"
    ));

    // Read from shell command
    suite.add_test(TestCase::new(
        "read_from_shell",
        "read",
        "r !echo hello from shell\nw\nq\n",
        "existing\n"
    ));

    // Read after specific line
    suite.add_test(TestCase::new(
        "read_after_line_1",
        "read",
        "1r !echo inserted\nw\nq\n",
        "line 1\nline 2\n"
    ));

    suite
}
