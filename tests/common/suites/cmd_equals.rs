/// Test suite for the equals command (=)
/// GNU ed reference: main_loop.c case '=' (line 729)
///
/// The equals command prints the line number of the addressed line.
/// Syntax: [addr]=

use crate::common::{TestCase, TestSuite};

pub fn get_test_suite() -> TestSuite {
    let mut suite = TestSuite::new(
        "cmd_equals",
        "Equals command (=)"
    );

    // Show current line number
    suite.add_test(TestCase::new(
        "line_number_current",
        "equals",
        "=\nq\n",
        "line 1\n"
    ));

    // Show specific line number
    suite.add_test(TestCase::new(
        "line_number_specific",
        "equals",
        "2=\nq\n",
        "line 1\nline 2\nline 3\n"
    ));

    // Show last line number
    suite.add_test(TestCase::new(
        "line_number_last",
        "equals",
        "$=\nq\n",
        "line 1\nline 2\nline 3\n"
    ));

    // Show line number after navigation
    suite.add_test(TestCase::new(
        "line_number_after_navigation",
        "equals",
        "3\n=\nq\n",
        "line 1\nline 2\nline 3\n"
    ));

    suite
}
