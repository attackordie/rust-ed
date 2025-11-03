/// Test suite for the list command (l)
/// GNU ed reference: main_loop.c case 'l' (line 646)
///
/// The list command displays lines with special characters visible.
/// Syntax: [addr[,addr]]l
/// Shows tabs as \t, $ at end of line, etc.

use crate::common::{TestCase, TestSuite};

pub fn get_test_suite() -> TestSuite {
    let mut suite = TestSuite::new(
        "cmd_list",
        "List command (l)"
    );

    // List basic line
    suite.add_test(TestCase::new(
        "list_basic",
        "list",
        "l\nq\n",
        "hello world$\n"
    ));

    // List with tabs
    suite.add_test(TestCase::new(
        "list_tabs",
        "list",
        "l\nq\n",
        "line\twith\ttabs\n"
    ));

    // List with special characters
    suite.add_test(TestCase::new(
        "list_special_chars",
        "list",
        "l\nq\n",
        "special $chars$ here\n"
    ));

    // List range of lines
    suite.add_test(TestCase::new(
        "list_range",
        "list",
        "1,2l\nq\n",
        "line 1\nline 2\nline 3\n"
    ));

    suite
}
