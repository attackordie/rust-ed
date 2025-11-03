/// Test suite for the yank command (y)
/// GNU ed reference: main_loop.c case 'y' (line 726)
///
/// The yank command copies addressed lines to the yank buffer (cut buffer).
/// Syntax: [addr[,addr]]y

use crate::common::{TestCase, TestSuite};

pub fn get_test_suite() -> TestSuite {
    let mut suite = TestSuite::new(
        "cmd_yank",
        "Yank command (y)"
    );

    // Yank single line
    suite.add_test(TestCase::new(
        "yank_line",
        "yank",
        "1,2y\nw\nq\n",
        "line 1\nline 2\nline 3\n"
    ));

    // Yank range
    suite.add_test(TestCase::new(
        "yank_range",
        "yank",
        "1,3y\nw\nq\n",
        "line 1\nline 2\nline 3\n"
    ));

    // Yank all lines
    suite.add_test(TestCase::new(
        "yank_all",
        "yank",
        "%y\nw\nq\n",
        "line 1\nline 2\nline 3\n"
    ));

    suite
}
