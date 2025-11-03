/// Test suite for the append command (a)
/// GNU ed reference: main_loop.c case 'a' (line 568)
///
/// The append command adds text after the addressed line.
/// Syntax: [addr]a
/// Text input ends with a line containing only '.'

use crate::common::{TestCase, TestSuite};

pub fn get_test_suite() -> TestSuite {
    let mut suite = TestSuite::new(
        "cmd_append",
        "Append command (a)"
    );

    // Append to a line
    suite.add_test(TestCase::new(
        "append_to_line",
        "append",
        "a\nappended line\n.\nw\nq\n",
        "original line\n"
    ));

    // Append multiple lines
    suite.add_test(TestCase::new(
        "append_multiple_lines",
        "append",
        "a\nline 1\nline 2\nline 3\n.\nw\nq\n",
        "original\n"
    ));

    // Append to empty buffer (address 0)
    suite.add_test(TestCase::new(
        "append_to_empty_buffer",
        "append",
        "a\nfirst line\n.\nw\nq\n",
        ""
    ));

    // Append after specific line
    suite.add_test(TestCase::new(
        "append_after_line_2",
        "append",
        "2a\ninserted\n.\nw\nq\n",
        "line 1\nline 2\nline 3\n"
    ));

    suite
}
