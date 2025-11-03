/// Test suite for the insert command (i)
/// GNU ed reference: main_loop.c case 'i' (line 632)
///
/// The insert command adds text before the addressed line.
/// Syntax: [addr]i
/// Text input ends with a line containing only '.'

use crate::common::{TestCase, TestSuite};

pub fn get_test_suite() -> TestSuite {
    let mut suite = TestSuite::new(
        "cmd_insert",
        "Insert command (i)"
    );

    // Insert before line
    suite.add_test(TestCase::new(
        "insert_before_line",
        "insert",
        "1i\ninserted line\n.\nw\nq\n",
        "original line\n"
    ));

    // Insert multiple lines
    suite.add_test(TestCase::new(
        "insert_multiple_lines",
        "insert",
        "1i\nline 1\nline 2\n.\nw\nq\n",
        "original\n"
    ));

    // Insert at beginning
    suite.add_test(TestCase::new(
        "insert_at_beginning",
        "insert",
        "1i\nfirst\n.\nw\nq\n",
        "second\nthird\n"
    ));

    // Insert at specific position
    suite.add_test(TestCase::new(
        "insert_at_position_2",
        "insert",
        "2i\ninserted\n.\nw\nq\n",
        "line 1\nline 2\nline 3\n"
    ));

    suite
}
