/// Test suite for the write command (w, W)
/// GNU ed reference: main_loop.c case 'w' and 'W'
///
/// The write command writes the buffer to a file.
/// Syntax: [addr[,addr]]w [filename]
/// W appends to file instead of overwriting

use crate::common::{TestCase, TestSuite};

pub fn get_test_suite() -> TestSuite {
    let mut suite = TestSuite::new(
        "cmd_write",
        "Write command (w, W)"
    );

    // Basic write operation
    suite.add_test(TestCase::new(
        "write_basic",
        "write",
        "w\nq\n",
        "test content\n"
    ));

    // Write after modification
    suite.add_test(TestCase::new(
        "write_after_modification",
        "write",
        "a\nnew line\n.\nw\nq\n",
        "original\n"
    ));

    // Write empty buffer (critical test case!)
    // After deleting all lines, write should succeed with 0 bytes
    suite.add_test(TestCase::new(
        "write_empty_buffer",
        "write",
        "1,$d\nw\nq\n",
        "line 1\nline 2\n"
    ));

    // Write single line buffer
    suite.add_test(TestCase::new(
        "write_single_line",
        "write",
        "w\nq\n",
        "single line\n"
    ));

    // Write after deleting some lines
    suite.add_test(TestCase::new(
        "write_after_delete",
        "write",
        "1d\nw\nq\n",
        "delete this\nkeep this\n"
    ));

    suite
}
