/// Common test definitions for differential testing framework
///
/// This module provides the core test structures used by the differential
/// testing framework.

pub mod suites;

/// A single test case for differential testing
#[derive(Debug, Clone)]
pub struct TestCase {
    /// Unique test identifier (e.g., "delete_single_line")
    pub name: String,

    /// Category for grouping/reporting (e.g., "delete", "write")
    pub category: String,

    /// Command sequence to execute (e.g., "1d\nw\nq\n")
    pub commands: String,

    /// Initial file content before commands are run
    pub input_text: String,
}

impl TestCase {
    /// Create a new test case
    pub fn new(name: &str, category: &str, commands: &str, input_text: &str) -> Self {
        Self {
            name: name.to_string(),
            category: category.to_string(),
            commands: commands.to_string(),
            input_text: input_text.to_string(),
        }
    }
}

/// A test suite is a collection of related test cases
pub struct TestSuite {
    pub name: String,
    pub description: String,
    pub test_cases: Vec<TestCase>,
}

impl TestSuite {
    pub fn new(name: &str, description: &str) -> Self {
        Self {
            name: name.to_string(),
            description: description.to_string(),
            test_cases: Vec::new(),
        }
    }

    pub fn add_test(&mut self, test_case: TestCase) {
        self.test_cases.push(test_case);
    }

    pub fn add_tests(&mut self, test_cases: Vec<TestCase>) {
        self.test_cases.extend(test_cases);
    }
}
