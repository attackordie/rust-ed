/// Test suite for the change command (c)
/// GNU ed reference: main_loop.c case 'c' (line 572)
///
/// The change command replaces the addressed lines with new text.
/// Syntax: [addr[,addr]]c
/// Text input ends with a line containing only '.'

use crate::common::{TestCase, TestSuite};

pub fn get_test_suite() -> TestSuite {
    let mut suite = TestSuite::new(
        "cmd_change",
        "Change command (c)"
    );

    // Change single line
    suite.add_test(TestCase::new(
        "change_single_line",
        "change",
        "1c\nnew content\n.\nw\nq\n",
        "old content\n"
    ));

    // Change range of lines
    suite.add_test(TestCase::new(
        "change_range",
        "change",
        "1,2c\nreplacement\n.\nw\nq\n",
        "old 1\nold 2\nkeep 3\n"
    ));

    // Change current line (no address)
    suite.add_test(TestCase::new(
        "change_current_line",
        "change",
        "2\nc\nnew line 2\n.\nw\nq\n",
        "line 1\nline 2\nline 3\n"
    ));

    // Change all lines
    suite.add_test(TestCase::new(
        "change_all_lines",
        "change",
        "%c\ncompletely new\n.\nw\nq\n",
        "old 1\nold 2\nold 3\n"
    ));

    suite
}
