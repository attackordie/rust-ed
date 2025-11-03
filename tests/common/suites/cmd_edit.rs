/// Test suite for the edit command (e, E)
/// GNU ed reference: main_loop.c case 'e' and 'E'
///
/// The edit command clears the buffer and reads a new file.
/// e - edit (warns if buffer modified)
/// E - edit unconditionally (no warning)
/// Syntax: e [filename]

use crate::common::{TestCase, TestSuite};

pub fn get_test_suite() -> TestSuite {
    let mut suite = TestSuite::new(
        "cmd_edit",
        "Edit command (e, E)"
    );

    // Edit non-existent file (should error with stderr message)
    suite.add_test(TestCase::new(
        "edit_file",
        "edit",
        "e /tmp/test_edit.txt\nq\n",
        "initial content\n"
    ));

    // Edit with tilde expansion
    suite.add_test(TestCase::new(
        "edit_tilde_expansion",
        "edit",
        "e ~/test.txt\nq\n",
        "content\n"
    ));

    // Edit with shell command
    suite.add_test(TestCase::new(
        "edit_shell_command",
        "edit",
        "e !echo test content\nq\n",
        "old content\n"
    ));

    suite
}
