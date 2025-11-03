/// Test suite for the substitute command (s)
/// GNU ed reference: main_loop.c case 's' (line 681)
///
/// The substitute command replaces text matching a pattern.
/// Syntax: [addr[,addr]]s/pattern/replacement/[flags]
/// Flags: g (global), p (print), n (nth occurrence), I (case insensitive)

use crate::common::{TestCase, TestSuite};

pub fn get_test_suite() -> TestSuite {
    let mut suite = TestSuite::new(
        "cmd_substitute",
        "Substitute command (s)"
    );

    // Basic substitution
    suite.add_test(TestCase::new(
        "substitute_basic",
        "substitute",
        "s/old/new/\nw\nq\n",
        "old text here\n"
    ));

    // Substitute with global flag
    suite.add_test(TestCase::new(
        "substitute_global_flag",
        "substitute",
        "s/o/O/g\nw\nq\n",
        "foo boo zoo\n"
    ));

    // Substitute with print flag
    suite.add_test(TestCase::new(
        "substitute_with_print",
        "substitute",
        "s/old/new/p\nw\nq\n",
        "old text\n"
    ));

    // Substitute with case insensitive flag (I)
    // CRITICAL: This is test #50 that's currently failing
    suite.add_test(TestCase::new(
        "substitute_case_insensitive",
        "substitute",
        "s/OLD/new/I\nw\nq\n",
        "old text here\n"
    ));

    // Substitute nth occurrence
    // CRITICAL: This is test #51 that's currently failing
    suite.add_test(TestCase::new(
        "substitute_nth_occurrence",
        "substitute",
        "s/o/O/2\nw\nq\n",
        "foo boo zoo\n"
    ));

    // Substitute on range
    suite.add_test(TestCase::new(
        "substitute_range",
        "substitute",
        "1,2s/old/new/\nw\nq\n",
        "old 1\nold 2\nold 3\n"
    ));

    // Substitute on all lines
    suite.add_test(TestCase::new(
        "substitute_all_lines",
        "substitute",
        "%s/old/new/\nw\nq\n",
        "old 1\nkeep\nold 2\n"
    ));

    suite
}
