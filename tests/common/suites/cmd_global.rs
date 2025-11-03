/// Test suite for the global command (g, v, G, V)
/// GNU ed reference: main_loop.c case 'g' and 'v' (lines 609-610)
///
/// Global commands apply operations to lines matching/not-matching patterns.
/// g - apply command to lines matching pattern
/// v - apply command to lines NOT matching pattern
/// G, V - interactive versions

use crate::common::{TestCase, TestSuite};

pub fn get_test_suite() -> TestSuite {
    let mut suite = TestSuite::new(
        "cmd_global",
        "Global command (g, v, G, V)"
    );

    // Global delete matching pattern
    suite.add_test(TestCase::new(
        "global_delete_pattern",
        "global",
        "g/delete/d\nw\nq\n",
        "keep this\ndelete this\nkeep that\ndelete that\n"
    ));

    // Global print matching pattern
    suite.add_test(TestCase::new(
        "global_print_pattern",
        "global",
        "g/test/p\nq\n",
        "test line 1\nother\ntest line 2\n"
    ));

    // Global substitute (CRITICAL - currently failing test #33)
    suite.add_test(TestCase::new(
        "global_substitute",
        "global",
        "g/old/s/old/new/\nw\nq\n",
        "old text\nkeep\nold again\n"
    ));

    // Inverse global delete (v - delete lines NOT matching)
    suite.add_test(TestCase::new(
        "inverse_global_delete",
        "inverse_global",
        "v/keep/d\nw\nq\n",
        "keep this\ndelete this\nkeep that\ndelete that\n"
    ));

    // Inverse global print
    suite.add_test(TestCase::new(
        "inverse_global_print",
        "inverse_global",
        "v/skip/p\nq\n",
        "print this\nskip this\nprint that\n"
    ));

    suite
}
