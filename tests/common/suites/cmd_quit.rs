/// Test suite for the quit command (q, Q)
/// GNU ed reference: main_loop.c case 'q' and 'Q'
///
/// The quit command exits the editor.
/// q - quit (warns if buffer modified)
/// Q - quit unconditionally (no warning)

use crate::common::{TestCase, TestSuite};

pub fn get_test_suite() -> TestSuite {
    let mut suite = TestSuite::new(
        "cmd_quit",
        "Quit command (q, Q)"
    );

    // Basic quit
    suite.add_test(TestCase::new(
        "quit_basic",
        "quit",
        "q\n",
        "test content\n"
    ));

    // Unconditional quit (Q)
    suite.add_test(TestCase::new(
        "quit_unconditional",
        "quit",
        "Q\n",
        "test content\n"
    ));

    // Quit after write (clean exit)
    suite.add_test(TestCase::new(
        "quit_after_write",
        "quit",
        "a\nnew line\n.\nw\nq\n",
        ""
    ));

    suite
}
