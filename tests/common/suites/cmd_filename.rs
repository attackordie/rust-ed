/// Test suite for the filename command (f)
/// GNU ed reference: main_loop.c case 'f' (line 601)
///
/// The filename command displays or sets the default filename.
/// Syntax: f [filename]

use crate::common::{TestCase, TestSuite};

pub fn get_test_suite() -> TestSuite {
    let mut suite = TestSuite::new(
        "cmd_filename",
        "Filename command (f)"
    );

    // Display current filename
    suite.add_test(TestCase::new(
        "filename_display",
        "filename",
        "f\nq\n",
        "content\n"
    ));

    // Set new filename
    suite.add_test(TestCase::new(
        "filename_set",
        "filename",
        "f /tmp/newfile.txt\nq\n",
        "content\n"
    ));

    // Set filename with tilde expansion
    suite.add_test(TestCase::new(
        "filename_tilde_expansion",
        "filename",
        "f ~/testfile.txt\nf\nq\n",
        "content\n"
    ));

    suite
}
