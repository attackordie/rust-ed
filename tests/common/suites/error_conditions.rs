/// Test suite for error handling and edge cases
/// Tests invalid commands, addresses, and error recovery

use crate::common::{TestCase, TestSuite};

pub fn get_test_suite() -> TestSuite {
    let mut suite = TestSuite::new(
        "error_conditions",
        "Error handling and edge cases"
    );

    // Invalid command tests
    suite.add_test(TestCase::new(
        "error_invalid_command",
        "error",
        "x\nq\n",
        "test\n"
    ));

    suite.add_test(TestCase::new(
        "error_unknown_command",
        "error",
        "@\nq\n",
        "test\n"
    ));

    // Invalid address tests
    suite.add_test(TestCase::new(
        "error_invalid_address",
        "error",
        "999p\nq\n",
        "single line\n"
    ));

    suite.add_test(TestCase::new(
        "error_negative_address",
        "error",
        "-1p\nq\n",
        "line 1\n"
    ));

    suite.add_test(TestCase::new(
        "error_invalid_range",
        "error",
        "3,1p\nq\n",
        "line 1\nline 2\n"
    ));

    // Empty buffer operations
    suite.add_test(TestCase::new(
        "error_empty_buffer_print",
        "error",
        "p\nq\n",
        ""
    ));

    suite.add_test(TestCase::new(
        "error_empty_buffer_delete",
        "error",
        "d\nq\n",
        ""
    ));

    // Operation without address when required
    suite.add_test(TestCase::new(
        "error_no_address_for_print",
        "error",
        "p\nq\n",
        ""
    ));

    // Unexpected address where not allowed
    suite.add_test(TestCase::new(
        "error_address_on_quit",
        "error",
        "1q\nq\n",
        "line 1\n"
    ));

    suite
}
