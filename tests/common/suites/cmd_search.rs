/// Test suite for the search commands (/, ?)
/// GNU ed reference: main_loop.c case '/' and '?'
///
/// / - forward search for pattern
/// ? - backward search for pattern
/// Syntax: /pattern/  or  ?pattern?

use crate::common::{TestCase, TestSuite};

pub fn get_test_suite() -> TestSuite {
    let mut suite = TestSuite::new(
        "cmd_search",
        "Search commands (/, ?)"
    );

    // Forward search basic
    suite.add_test(TestCase::new(
        "search_forward_basic",
        "search",
        "/world/p\nq\n",
        "hello world\nfoo bar\nworld again\n"
    ));

    // Forward search not found
    suite.add_test(TestCase::new(
        "search_forward_not_found",
        "search",
        "/notfound/p\nq\n",
        "line 1\nline 2\n"
    ));

    // Forward search with action
    suite.add_test(TestCase::new(
        "search_forward_delete",
        "search",
        "/delete/d\nw\nq\n",
        "keep\ndelete this\nkeep\n"
    ));

    // Backward search basic
    suite.add_test(TestCase::new(
        "search_backward_basic",
        "search",
        "$\n?world?p\nq\n",
        "world here\nline 2\nworld there\n"
    ));

    // Backward search with range
    suite.add_test(TestCase::new(
        "search_backward_range",
        "search",
        "$\n?line?,p\nq\n",
        "line 1\nline 2\nline 3\n"
    ));

    suite
}
