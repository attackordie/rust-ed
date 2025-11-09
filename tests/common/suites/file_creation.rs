/// Test suite for file creation behavior
/// Tests launching ed with non-existent file argument
///
/// GNU ed behavior when launched with non-existent file:
/// 1. Prints "filename: No such file or directory" to stderr
/// 2. Opens editor with empty buffer
/// 3. 'w' command writes to the filename specified at launch
/// 4. Does NOT prompt "file does not exist. create it? (y/N)"

use crate::common::{TestCase, TestSuite};

pub fn get_test_suite() -> TestSuite {
    let mut suite = TestSuite::new(
        "file_creation",
        "File creation behavior with non-existent files"
    );

    // Test: Launch with non-existent file, add content, write
    // Verifies stderr warning and successful write operation
    suite.add_test(TestCase::new_nonexistent_file(
        "launch_nonexistent_file_then_write",
        "file_creation",
        "a\nline one\nline two\n.\nw\nq\n"
    ));

    // Test: Write command without filename argument uses launch filename
    suite.add_test(TestCase::new_nonexistent_file(
        "write_uses_launch_filename",
        "file_creation",
        "a\ntest content\n.\nw\nq\n"
    ));

    // Test: Non-existent file opens with empty buffer
    suite.add_test(TestCase::new_nonexistent_file(
        "nonexistent_file_empty_buffer",
        "file_creation",
        ",p\nq\n"
    ));

    // Test: Write to different filename from non-existent file launch
    suite.add_test(TestCase::new_nonexistent_file(
        "write_to_different_filename",
        "file_creation",
        "a\ncontent\n.\nw /tmp/otherfile.txt\nq\n"
    ));

    // Test: Error message format for non-existent file
    suite.add_test(TestCase::new_nonexistent_file(
        "nonexistent_file_warning_message",
        "file_creation",
        "q\n"
    ));

    suite
}
