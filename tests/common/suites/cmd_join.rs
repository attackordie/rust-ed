/// Test suite for the join command (j)
/// GNU ed reference: main_loop.c case 'j' (line 634)
///
/// The join command concatenates addressed lines into a single line.
/// Syntax: [addr[,addr]]j

use crate::common::{TestCase, TestSuite};

pub fn get_test_suite() -> TestSuite {
    let mut suite = TestSuite::new(
        "cmd_join",
        "Join command (j)"
    );

    // Join two lines
    suite.add_test(TestCase::new(
        "join_two_lines",
        "join",
        "1,2j\nw\nq\n",
        "line 1\nline 2\nline 3\n"
    ));

    // Join multiple lines
    suite.add_test(TestCase::new(
        "join_multiple_lines",
        "join",
        "1,3j\nw\nq\n",
        "line 1\nline 2\nline 3\nline 4\n"
    ));

    // Join all lines
    suite.add_test(TestCase::new(
        "join_all_lines",
        "join",
        "%j\nw\nq\n",
        "line 1\nline 2\nline 3\n"
    ));

    suite
}
