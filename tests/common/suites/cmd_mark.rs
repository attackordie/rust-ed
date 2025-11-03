/// Test suite for the mark command (k, ')
/// GNU ed reference: main_loop.c case 'k' (line 640)
///
/// The mark command marks a line with a lowercase letter.
/// The ' command jumps to a marked line.
/// Syntax: [addr]k[a-z]  or  '[a-z]

use crate::common::{TestCase, TestSuite};

pub fn get_test_suite() -> TestSuite {
    let mut suite = TestSuite::new(
        "cmd_mark",
        "Mark command (k, ')"
    );

    // Mark a line and print it
    suite.add_test(TestCase::new(
        "mark_line",
        "mark",
        "1ka\n2kb\n'ap\nq\n",
        "line 1\nline 2\nline 3\n"
    ));

    // Mark and goto
    suite.add_test(TestCase::new(
        "mark_goto",
        "mark",
        "1ka\n3\n'a\n.p\nq\n",
        "line 1\nline 2\nline 3\n"
    ));

    // Mark multiple lines
    suite.add_test(TestCase::new(
        "mark_multiple",
        "mark",
        "1ka\n2kb\n3kc\n'bp\nq\n",
        "line 1\nline 2\nline 3\n"
    ));

    // Use mark in range
    suite.add_test(TestCase::new(
        "mark_in_range",
        "mark",
        "2ka\n'a,$p\nq\n",
        "line 1\nline 2\nline 3\n"
    ));

    suite
}
