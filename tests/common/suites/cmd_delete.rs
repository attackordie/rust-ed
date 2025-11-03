/// Test suite for the delete command (d)
/// GNU ed reference: main_loop.c case 'd' (line 580)
///
/// The delete command removes one or more lines from the buffer.
/// Syntax: [addr[,addr]]d
/// Default address: current line

use crate::common::{TestCase, TestSuite};

pub fn get_test_suite() -> TestSuite {
    let mut suite = TestSuite::new(
        "cmd_delete",
        "Delete command (d)"
    );

    // Basic single line deletion
    suite.add_test(TestCase::new(
        "delete_single_line",
        "delete",
        "1d\nw\nq\n",
        "line to delete\nline to keep\n"
    ));

    // Delete a range of lines
    suite.add_test(TestCase::new(
        "delete_range",
        "delete",
        "1,2d\nw\nq\n",
        "delete 1\ndelete 2\nkeep this\n"
    ));

    // Delete all lines (critical test - tests current_addr update)
    suite.add_test(TestCase::new(
        "delete_all_lines",
        "delete",
        "1,$d\nw\nq\n",
        "line 1\nline 2\nline 3\n"
    ));

    // Delete current line (no address specified)
    suite.add_test(TestCase::new(
        "delete_current_line",
        "delete",
        "2\nd\nw\nq\n",
        "line 1\nline 2\nline 3\n"
    ));

    // Delete last line using $ address
    suite.add_test(TestCase::new(
        "delete_last_line",
        "delete",
        "$d\nw\nq\n",
        "line 1\nline 2\nline 3\n"
    ));

    // Delete using relative addressing
    suite.add_test(TestCase::new(
        "delete_relative_plus",
        "delete",
        "1\n+d\nw\nq\n",
        "line 1\nline 2\nline 3\n"
    ));

    // Delete with mark addressing
    suite.add_test(TestCase::new(
        "delete_marked_line",
        "delete",
        "2ka\n'ad\nw\nq\n",
        "line 1\nline 2\nline 3\n"
    ));

    suite
}
