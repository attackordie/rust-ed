/// Test suite for the help command (h, H)
/// GNU ed reference: main_loop.c case 'h' and 'H' (lines 622-623)
///
/// h - prints explanation of last error
/// H - toggles verbose error mode
/// Syntax: h  or  H

use crate::common::{TestCase, TestSuite};

pub fn get_test_suite() -> TestSuite {
    let mut suite = TestSuite::new(
        "cmd_help",
        "Help command (h, H)"
    );

    // Help basic (print last error)
    suite.add_test(TestCase::new(
        "help_basic",
        "help",
        "h\nq\n",
        "content\n"
    ));

    // Help verbose toggle (H)
    suite.add_test(TestCase::new(
        "help_verbose",
        "help",
        "H\nq\n",
        "content\n"
    ));

    // Help after error
    suite.add_test(TestCase::new(
        "help_after_error",
        "help",
        "999p\nh\nq\n",
        "content\n"
    ));

    suite
}
