/// Test suite for the undo command (u)
/// GNU ed reference: main_loop.c case 'u' (line 688)
///
/// The undo command undoes the last command that modified the buffer.
/// Syntax: u
/// Only one level of undo is supported

use crate::common::{TestCase, TestSuite};

pub fn get_test_suite() -> TestSuite {
    let mut suite = TestSuite::new(
        "cmd_undo",
        "Undo command (u)"
    );

    // Undo delete
    suite.add_test(TestCase::new(
        "undo_delete",
        "undo",
        "1d\nu\nw\nq\n",
        "line 1\nline 2\n"
    ));

    // Undo append
    suite.add_test(TestCase::new(
        "undo_append",
        "undo",
        "a\nnew line\n.\nu\nw\nq\n",
        "original\n"
    ));

    // Undo substitute
    suite.add_test(TestCase::new(
        "undo_substitute",
        "undo",
        "s/old/new/\nu\nw\nq\n",
        "old text\n"
    ));

    // Undo change
    suite.add_test(TestCase::new(
        "undo_change",
        "undo",
        "1c\nchanged\n.\nu\nw\nq\n",
        "original\n"
    ));

    suite
}
