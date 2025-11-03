/// Test suite for the move command (m)
/// GNU ed reference: main_loop.c case 'm' (line 655)
///
/// The move command moves addressed lines to after the destination address.
/// Syntax: [addr[,addr]]m addr

use crate::common::{TestCase, TestSuite};

pub fn get_test_suite() -> TestSuite {
    let mut suite = TestSuite::new(
        "cmd_move",
        "Move command (m)"
    );

    // Move line down
    suite.add_test(TestCase::new(
        "move_line_down",
        "move",
        "1m2\nw\nq\n",
        "line 1\nline 2\nline 3\n"
    ));

    // Move line up
    suite.add_test(TestCase::new(
        "move_line_up",
        "move",
        "3m0\nw\nq\n",
        "line 1\nline 2\nline 3\n"
    ));

    // Move range
    suite.add_test(TestCase::new(
        "move_range",
        "move",
        "1,2m3\nw\nq\n",
        "line 1\nline 2\nline 3\nline 4\n"
    ));

    // Move to end
    suite.add_test(TestCase::new(
        "move_to_end",
        "move",
        "1m$\nw\nq\n",
        "line 1\nline 2\nline 3\n"
    ));

    suite
}
