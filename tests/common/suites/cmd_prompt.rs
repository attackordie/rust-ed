/// Test suite for the prompt command (P)
/// GNU ed reference: main_loop.c case 'P' (line 664)
///
/// The prompt command toggles the command prompt on/off.
/// Syntax: P

use crate::common::{TestCase, TestSuite};

pub fn get_test_suite() -> TestSuite {
    let mut suite = TestSuite::new(
        "cmd_prompt",
        "Prompt command (P)"
    );

    // Toggle prompt
    suite.add_test(TestCase::new(
        "prompt_toggle",
        "prompt",
        "P\nq\n",
        "content\n"
    ));

    // Toggle prompt twice (should return to original state)
    suite.add_test(TestCase::new(
        "prompt_toggle_twice",
        "prompt",
        "P\nP\nq\n",
        "content\n"
    ));

    suite
}
