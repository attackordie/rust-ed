/// Test suite for address parsing and navigation
/// Tests line addressing: numbers, ., $, %, ,, ;, +, -, ', //, ??

use crate::common::{TestCase, TestSuite};

pub fn get_test_suite() -> TestSuite {
    let mut suite = TestSuite::new(
        "addressing",
        "Address parsing and line navigation"
    );

    // Basic line number addressing
    suite.add_test(TestCase::new(
        "address_line_number",
        "addressing",
        "2p\nq\n",
        "Line 1\nLine 2\nLine 3\n"
    ));

    suite.add_test(TestCase::new(
        "address_range",
        "addressing",
        "1,2p\nq\n",
        "Line 1\nLine 2\nLine 3\n"
    ));

    // Current line (.) addressing
    suite.add_test(TestCase::new(
        "address_current_line",
        "addressing",
        "2\n.p\nq\n",
        "Line 1\nLine 2\nLine 3\n"
    ));

    // Last line ($) addressing
    suite.add_test(TestCase::new(
        "address_last_line",
        "addressing",
        "$p\nq\n",
        "Line 1\nLine 2\nLast Line\n"
    ));

    // All lines (%) addressing
    suite.add_test(TestCase::new(
        "address_all_lines",
        "addressing",
        "%p\nq\n",
        "Line 1\nLine 2\nLine 3\n"
    ));

    // Comma (,) addressing
    suite.add_test(TestCase::new(
        "address_comma",
        "addressing",
        ",p\nq\n",
        "Line 1\nLine 2\nLine 3\n"
    ));

    // Relative addressing (+/-)
    suite.add_test(TestCase::new(
        "address_relative_plus",
        "addressing",
        "1\n+p\nq\n",
        "Line 1\nLine 2\nLine 3\n"
    ));

    suite.add_test(TestCase::new(
        "address_relative_minus",
        "addressing",
        "3\n-p\nq\n",
        "Line 1\nLine 2\nLine 3\n"
    ));

    // Mark addressing
    suite.add_test(TestCase::new(
        "address_mark",
        "addressing",
        "1ka\n'ap\nq\n",
        "Line 1\nLine 2\nLine 3\n"
    ));

    suite
}
