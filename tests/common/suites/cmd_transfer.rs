/// Test suite for the transfer/copy command (t)
/// GNU ed reference: main_loop.c case 't' (line 684)
///
/// The transfer command copies addressed lines to after the destination address.
/// Syntax: [addr[,addr]]t addr
/// Also known as the 'copy' command

use crate::common::{TestCase, TestSuite};

pub fn get_test_suite() -> TestSuite {
    let mut suite = TestSuite::new(
        "cmd_transfer",
        "Transfer/Copy command (t)"
    );

    // Copy single line
    suite.add_test(TestCase::new(
        "copy_line",
        "transfer",
        "1t2\nw\nq\n",
        "line 1\nline 2\nline 3\n"
    ));

    // Copy range
    suite.add_test(TestCase::new(
        "copy_range",
        "transfer",
        "1,2t3\nw\nq\n",
        "line 1\nline 2\nline 3\n"
    ));

    // Copy to beginning
    suite.add_test(TestCase::new(
        "copy_to_beginning",
        "transfer",
        "3t0\nw\nq\n",
        "line 1\nline 2\nline 3\n"
    ));

    // Copy to end
    suite.add_test(TestCase::new(
        "copy_to_end",
        "transfer",
        "1t$\nw\nq\n",
        "line 1\nline 2\nline 3\n"
    ));

    suite
}
