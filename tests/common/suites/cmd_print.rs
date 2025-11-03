/// Test suite for the print command (p)
/// GNU ed reference: main_loop.c case 'p' (line 648)
///
/// The print command displays the addressed lines.
/// Syntax: [addr[,addr]]p
/// Default address: current line

use crate::common::{TestCase, TestSuite};

pub fn get_test_suite() -> TestSuite {
    let mut suite = TestSuite::new(
        "cmd_print",
        "Print command (p)"
    );

    // Print from empty buffer (should error)
    suite.add_test(TestCase::new(
        "print_empty_buffer",
        "print",
        "p\nq\n",
        ""
    ));

    // Print single line
    suite.add_test(TestCase::new(
        "print_single_line",
        "print",
        "p\nq\n",
        "Hello world\n"
    ));

    // Print multiple lines with range
    suite.add_test(TestCase::new(
        "print_multiple_lines",
        "print",
        "1,3p\nq\n",
        "Line 1\nLine 2\nLine 3\n"
    ));

    // Print range
    suite.add_test(TestCase::new(
        "print_range",
        "print",
        "1,2p\nq\n",
        "Line 1\nLine 2\nLine 3\n"
    ));

    // Print all lines using %
    suite.add_test(TestCase::new(
        "print_all_percent",
        "print",
        "%p\nq\n",
        "Line 1\nLine 2\nLine 3\n"
    ));

    // Print all lines using ,
    suite.add_test(TestCase::new(
        "print_all_comma",
        "print",
        ",p\nq\n",
        "Line 1\nLine 2\nLine 3\n"
    ));

    suite
}
