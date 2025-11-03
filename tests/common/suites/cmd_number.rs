/// Test suite for the number command (n)
/// GNU ed reference: main_loop.c case 'n' (line 647)
///
/// The number command displays lines with line numbers prepended.
/// Syntax: [addr[,addr]]n

use crate::common::{TestCase, TestSuite};

pub fn get_test_suite() -> TestSuite {
    let mut suite = TestSuite::new(
        "cmd_number",
        "Number command (n)"
    );

    // Numbered print single line
    suite.add_test(TestCase::new(
        "numbered_print_single",
        "numbered",
        "n\nq\n",
        "line 1\n"
    ));

    // Numbered print range
    suite.add_test(TestCase::new(
        "numbered_print_range",
        "numbered",
        "1,3n\nq\n",
        "line 1\nline 2\nline 3\n"
    ));

    // Numbered print all lines
    suite.add_test(TestCase::new(
        "numbered_print_all",
        "numbered",
        "%n\nq\n",
        "line 1\nline 2\nline 3\n"
    ));

    suite
}
