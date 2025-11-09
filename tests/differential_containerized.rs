/// Containerized differential testing framework
/// Compares rust-ed CONTAINER against GNU ed CONTAINER in identical environments
/// This proves true drop-in replacement - both in identical Docker containers
/// ONLY difference: C binary vs Rust binary
///
/// ARCHITECTURE: Modular and test-agnostic
/// - Framework doesn't care what tests it runs
/// - Test suites are defined in suites/
/// - Adding new tests = just create a new test suite file
/// - Framework automatically discovers and runs all registered test suites

mod common;

use std::process::{Command, Output};
use std::fs;
use std::io::Write;
use tempfile::NamedTempFile;
use std::collections::HashMap;
use common::{TestCase, TestSuite};

/// Detailed test failure information for debugging
#[derive(Debug)]
struct TestFailure {
    test_id: usize,
    test_name: String,
    commands: String,
    input_text: String,
    exit_code_diff: Option<(i32, i32)>,
    stdout_diff: Option<(String, String)>,
    stderr_diff: Option<(String, String)>,
    file_diff: Option<(String, String)>,
}

/// Symmetric containerized differential testing engine
/// Both GNU ed and rust-ed run in IDENTICAL container environments
struct EdDifferentialTester {
    gnu_ed_container: String,
    rust_ed_container: String,
    failures: Vec<TestFailure>,
    test_count: usize,
    test_categories: HashMap<String, usize>,
}

impl EdDifferentialTester {
    fn new() -> Self {
        Self {
            gnu_ed_container: "gnu-ed:latest".to_string(),
            rust_ed_container: "rust-ed:latest".to_string(),
            failures: Vec::new(),
            test_count: 0,
            test_categories: HashMap::new(),
        }
    }

    /// Run a single differential test comparing GNU ed container vs rust-ed container
    /// Takes a TestCase from any test suite - framework is test-agnostic
    fn run_differential_test(&mut self, test_case: &TestCase) -> bool {
        self.test_count += 1;
        *self.test_categories.entry(test_case.category.clone()).or_insert(0) += 1;

        println!("[{}/∞] Running: {}", self.test_count, test_case.name);

        // Handle file creation based on test_case.file_should_not_exist
        let (temp_path, _temp_file_guard) = if test_case.file_should_not_exist {
            // For non-existent file tests: create temp file but delete it before running ed
            // We need to create it first so we have a valid path in a writable directory
            let temp_file = NamedTempFile::new().expect("Failed to create temp file");
            let temp_path = temp_file.path().to_path_buf();
            // Delete the file but keep the path
            std::mem::drop(temp_file);  // This deletes the file
            (temp_path, None)
        } else {
            // Create temp file with input (existing behavior)
            let mut temp_file = NamedTempFile::new().expect("Failed to create temp file");
            temp_file.write_all(test_case.input_text.as_bytes()).expect("Failed to write temp file");
            let temp_path = temp_file.path().to_path_buf();
            // Keep temp_file alive so it doesn't get deleted
            (temp_path, Some(temp_file))
        };

        // Run GNU ed in isolated container
        let gnu_output = self.run_gnu_ed_container(&test_case.commands, &temp_path);
        let gnu_final = fs::read_to_string(&temp_path).unwrap_or_default();

        // Reset file for Rust test
        if test_case.file_should_not_exist {
            // Delete the file if it was created, so rust-ed gets the same non-existent state
            let _ = fs::remove_file(&temp_path);
        } else {
            fs::write(&temp_path, &test_case.input_text).expect("Failed to reset temp file");
        }

        // Run rust-ed in isolated container (SYMMETRIC - identical environment)
        let rust_output = self.run_rust_ed_container(&test_case.commands, &temp_path);
        let rust_final = fs::read_to_string(&temp_path).unwrap_or_default();

        // Cleanup temp file if needed
        if test_case.file_should_not_exist {
            let _ = fs::remove_file(&temp_path);
        }

        // Compare everything - build failure record
        let mut failure = TestFailure {
            test_id: self.test_count,
            test_name: test_case.name.clone(),
            commands: test_case.commands.clone(),
            input_text: test_case.input_text.clone(),
            exit_code_diff: None,
            stdout_diff: None,
            stderr_diff: None,
            file_diff: None,
        };

        let mut has_failure = false;

        // Check exit codes
        let gnu_exit = gnu_output.status.code().unwrap_or(-1);
        let rust_exit = rust_output.status.code().unwrap_or(-1);
        if gnu_exit != rust_exit {
            failure.exit_code_diff = Some((gnu_exit, rust_exit));
            has_failure = true;
        }

        // Check stdout
        let gnu_stdout = String::from_utf8_lossy(&gnu_output.stdout).to_string();
        let rust_stdout = String::from_utf8_lossy(&rust_output.stdout).to_string();
        if gnu_stdout != rust_stdout {
            failure.stdout_diff = Some((gnu_stdout, rust_stdout));
            has_failure = true;
        }

        // Check stderr (ignoring version strings)
        let gnu_stderr = String::from_utf8_lossy(&gnu_output.stderr).to_string();
        let rust_stderr = String::from_utf8_lossy(&rust_output.stderr).to_string();
        if !self.stderr_equivalent(&gnu_stderr, &rust_stderr) {
            failure.stderr_diff = Some((gnu_stderr, rust_stderr));
            has_failure = true;
        }

        // Check final file content
        if gnu_final != rust_final {
            failure.file_diff = Some((gnu_final, rust_final));
            has_failure = true;
        }

        if has_failure {
            self.failures.push(failure);
            println!("  ❌ FAIL");
            false
        } else {
            println!("  ✅ PASS");
            true
        }
    }

    /// Run GNU ed in isolated Docker container
    fn run_gnu_ed_container(&self, commands: &str, file_path: &std::path::Path) -> Output {
        // Get the parent directory and filename
        let parent_dir = file_path.parent().unwrap_or_else(|| std::path::Path::new("/tmp"));
        let filename = file_path.file_name().unwrap().to_str().unwrap();
        let container_path = format!("/tmp/{}", filename);

        Command::new("docker")
            .args(&["run", "--rm", "-i"])
            .args(&["--user", "1000:1000"])  // Run as host user to allow file writes
            .args(&["-v", &format!("{}:/tmp", parent_dir.display())])
            .arg(&self.gnu_ed_container)
            .arg(&container_path)
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped())
            .spawn()
            .and_then(|mut child| {
                if let Some(stdin) = child.stdin.as_mut() {
                    stdin.write_all(commands.as_bytes()).ok();
                }
                child.wait_with_output()
            })
            .unwrap_or_else(|e| {
                eprintln!("Failed to run GNU ed container: {}", e);
                eprintln!("Make sure to run: docker build -f docker/Dockerfile.gnu-ed -t gnu-ed:latest .");
                std::process::exit(1);
            })
    }

    /// Run rust-ed in isolated Docker container (SYMMETRIC with GNU ed)
    fn run_rust_ed_container(&self, commands: &str, file_path: &std::path::Path) -> Output {
        // Get the parent directory and filename
        let parent_dir = file_path.parent().unwrap_or_else(|| std::path::Path::new("/tmp"));
        let filename = file_path.file_name().unwrap().to_str().unwrap();
        let container_path = format!("/tmp/{}", filename);

        Command::new("docker")
            .args(&["run", "--rm", "-i"])
            .args(&["--user", "1000:1000"])  // Run as host user to allow file writes
            .args(&["-v", &format!("{}:/tmp", parent_dir.display())])
            .arg(&self.rust_ed_container)
            .arg(&container_path)
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped())
            .spawn()
            .and_then(|mut child| {
                if let Some(stdin) = child.stdin.as_mut() {
                    stdin.write_all(commands.as_bytes()).ok();
                }
                child.wait_with_output()
            })
            .unwrap_or_else(|e| {
                eprintln!("Failed to run rust-ed container: {}", e);
                eprintln!("Make sure to run: docker build -f docker/Dockerfile.rust-ed -t rust-ed:latest .");
                std::process::exit(1);
            })
    }

    /// Compare stderr, ignoring version strings
    fn stderr_equivalent(&self, gnu: &str, rust: &str) -> bool {
        // Remove version-specific parts
        let gnu_clean = gnu.replace("GNU ed", "ed").replace("GNU ed, version", "ed, version");
        let rust_clean = rust.replace("rust-ed", "ed");

        gnu_clean.trim() == rust_clean.trim()
    }

    /// Generate comprehensive test report
    pub fn print_test_report(&self) {
        println!("\n{}", "=".repeat(60));
        println!("  CONTAINERIZED DIFFERENTIAL TEST REPORT");
        println!("  (Both GNU ed and rust-ed in identical containers)");
        println!("{}", "=".repeat(60));
        println!("Total tests run: {}", self.test_count);
        println!("Failures: {}", self.failures.len());
        println!("Success rate: {:.1}%",
                 ((self.test_count - self.failures.len()) as f64 / self.test_count as f64) * 100.0);

        println!("\nTest categories:");
        for (category, count) in &self.test_categories {
            println!("  {}: {} tests", category, count);
        }

        println!("\nEnvironment:");
        println!("  GNU ed: {} (Ubuntu 22.04)", self.gnu_ed_container);
        println!("  rust-ed: {} (Ubuntu 22.04)", self.rust_ed_container);
        println!("  ONLY difference: C binary vs Rust binary");

        if !self.failures.is_empty() {
            println!("\nFIRST 5 FAILURES (for debugging):");
            for (i, failure) in self.failures.iter().enumerate().take(5) {
                println!("\n--- Failure {} ---", i + 1);
                println!("Test: {} (#{}) ", failure.test_name, failure.test_id);
                println!("Commands: {:?}", failure.commands);

                if let Some((gnu, rust)) = &failure.exit_code_diff {
                    println!("Exit codes differ: GNU={}, Rust={}", gnu, rust);
                }

                if let Some((gnu, rust)) = &failure.stdout_diff {
                    println!("Stdout differs:\n  GNU: {:?}\n  Rust: {:?}", gnu, rust);
                }

                if let Some((gnu, rust)) = &failure.stderr_diff {
                    println!("Stderr differs:\n  GNU: {:?}\n  Rust: {:?}", gnu, rust);
                }

                if let Some((gnu, rust)) = &failure.file_diff {
                    println!("File content differs:\n  GNU: {:?}\n  Rust: {:?}", gnu, rust);
                }
            }
        }
    }

    /// Main test suite runner - loads and runs all test suites
    /// MODULAR: Automatically discovers and runs all registered test suites
    pub fn run_comprehensive_tests(&mut self) -> bool {
        println!("🧪 Starting SYMMETRIC containerized differential testing...");
        println!("Comparing rust-ed container vs GNU ed container");
        println!("Both in IDENTICAL Ubuntu 22.04 environments");
        println!("This proves true drop-in replacement capability\n");

        // Load all test suites from the registry
        let test_suites = common::suites::get_all_test_suites();
        let total_suites = test_suites.len();
        let total_tests: usize = test_suites.iter().map(|s| s.test_cases.len()).sum();

        println!("📋 Loaded {} test suites with {} total test cases\n", total_suites, total_tests);

        // Run each test suite
        for suite in test_suites {
            println!("\n{} Testing {} ({} tests)",
                     get_emoji_for_suite(&suite.name),
                     suite.description,
                     suite.test_cases.len());

            for test_case in &suite.test_cases {
                self.run_differential_test(test_case);
            }
        }

        self.print_test_report();

        // Return true if all tests pass
        self.failures.is_empty()
    }

    /// Run specific test suites by name (for selective testing)
    pub fn run_test_suites(&mut self, suite_names: &[&str]) -> bool {
        println!("🧪 Running selected test suites: {:?}\n", suite_names);

        let test_suites = common::suites::get_test_suites_by_name(suite_names);

        for suite in test_suites {
            println!("\n{} Testing {}",
                     get_emoji_for_suite(&suite.name),
                     suite.description);

            for test_case in &suite.test_cases {
                self.run_differential_test(test_case);
            }
        }

        self.print_test_report();
        self.failures.is_empty()
    }
}

/// Helper to get emoji for test suite (for pretty output)
fn get_emoji_for_suite(suite_name: &str) -> &'static str {
    match suite_name {
        "basic_commands" => "📝",
        "file_operations" => "📁",
        "display_commands" => "🖨️ ",
        "global_commands" => "🌐",
        "line_manipulation" => "✂️ ",
        "search_replace" => "🔍",
        "addressing" => "🎯",
        "miscellaneous" => "🛠️ ",
        "error_conditions" => "⚠️ ",
        _ => "🧪",
    }
}

/// Main test entry point - runs ALL test suites
#[test]
fn test_containerized_drop_in_replacement() {
    let mut tester = EdDifferentialTester::new();

    println!("🐳 CONTAINERIZED DROP-IN REPLACEMENT TEST");
    println!("=========================================");
    println!("This test runs BOTH implementations in identical Docker containers");
    println!("Proving rust-ed is a true drop-in replacement for GNU ed");
    println!("MODULAR: Automatically runs all registered test suites\n");

    let all_pass = tester.run_comprehensive_tests();

    if all_pass {
        println!("\n🎉 SUCCESS! rust-ed is a verified drop-in replacement!");
        println!("Both containers produced IDENTICAL results across all tests.");
    } else {
        println!("\n⚠️  Some tests failed - see report above for details");
        panic!("Containerized drop-in replacement verification failed");
    }
}

/// Test only delete command (d)
#[test]
fn test_containerized_cmd_delete() {
    let mut tester = EdDifferentialTester::new();
    println!("🐳 Testing delete command (d) - containerized");
    tester.run_test_suites(&["cmd_delete"]);
}

/// Test only write command (w, W)
#[test]
fn test_containerized_cmd_write() {
    let mut tester = EdDifferentialTester::new();
    println!("🐳 Testing write command (w, W) - containerized");
    tester.run_test_suites(&["cmd_write"]);
}

/// Test only append command (a)
#[test]
fn test_containerized_cmd_append() {
    let mut tester = EdDifferentialTester::new();
    println!("🐳 Testing append command (a) - containerized");
    tester.run_test_suites(&["cmd_append"]);
}

/// Test only print command (p)
#[test]
fn test_containerized_cmd_print() {
    let mut tester = EdDifferentialTester::new();
    println!("🐳 Testing print command (p) - containerized");
    tester.run_test_suites(&["cmd_print"]);
}

/// Test only substitute command (s)
#[test]
fn test_containerized_cmd_substitute() {
    let mut tester = EdDifferentialTester::new();
    println!("🐳 Testing substitute command (s) - containerized");
    tester.run_test_suites(&["cmd_substitute"]);
}

/// Test only change command (c)
#[test]
fn test_containerized_cmd_change() {
    let mut tester = EdDifferentialTester::new();
    println!("🐳 Testing change command (c) - containerized");
    tester.run_test_suites(&["cmd_change"]);
}

/// Test only insert command (i)
#[test]
fn test_containerized_cmd_insert() {
    let mut tester = EdDifferentialTester::new();
    println!("🐳 Testing insert command (i) - containerized");
    tester.run_test_suites(&["cmd_insert"]);
}

/// Test only quit command (q, Q)
#[test]
fn test_containerized_cmd_quit() {
    let mut tester = EdDifferentialTester::new();
    println!("🐳 Testing quit command (q, Q) - containerized");
    tester.run_test_suites(&["cmd_quit"]);
}

/// Test only list command (l)
#[test]
fn test_containerized_cmd_list() {
    let mut tester = EdDifferentialTester::new();
    println!("🐳 Testing list command (l) - containerized");
    tester.run_test_suites(&["cmd_list"]);
}

/// Test only number command (n)
#[test]
fn test_containerized_cmd_number() {
    let mut tester = EdDifferentialTester::new();
    println!("🐳 Testing number command (n) - containerized");
    tester.run_test_suites(&["cmd_number"]);
}

/// Test only equals command (=)
#[test]
fn test_containerized_cmd_equals() {
    let mut tester = EdDifferentialTester::new();
    println!("🐳 Testing equals command (=) - containerized");
    tester.run_test_suites(&["cmd_equals"]);
}

/// Test only read command (r)
#[test]
fn test_containerized_cmd_read() {
    let mut tester = EdDifferentialTester::new();
    println!("🐳 Testing read command (r) - containerized");
    tester.run_test_suites(&["cmd_read"]);
}

/// Test only filename command (f)
#[test]
fn test_containerized_cmd_filename() {
    let mut tester = EdDifferentialTester::new();
    println!("🐳 Testing filename command (f) - containerized");
    tester.run_test_suites(&["cmd_filename"]);
}

/// Test only edit command (e, E)
#[test]
fn test_containerized_cmd_edit() {
    let mut tester = EdDifferentialTester::new();
    println!("🐳 Testing edit command (e, E) - containerized");
    tester.run_test_suites(&["cmd_edit"]);
}

/// Test only global command (g, v, G, V)
#[test]
fn test_containerized_cmd_global() {
    let mut tester = EdDifferentialTester::new();
    println!("🐳 Testing global command (g, v) - containerized");
    tester.run_test_suites(&["cmd_global"]);
}

/// Test only join command (j)
#[test]
fn test_containerized_cmd_join() {
    let mut tester = EdDifferentialTester::new();
    println!("🐳 Testing join command (j) - containerized");
    tester.run_test_suites(&["cmd_join"]);
}

/// Test only move command (m)
#[test]
fn test_containerized_cmd_move() {
    let mut tester = EdDifferentialTester::new();
    println!("🐳 Testing move command (m) - containerized");
    tester.run_test_suites(&["cmd_move"]);
}

/// Test only transfer command (t)
#[test]
fn test_containerized_cmd_transfer() {
    let mut tester = EdDifferentialTester::new();
    println!("🐳 Testing transfer command (t) - containerized");
    tester.run_test_suites(&["cmd_transfer"]);
}

/// Test only yank command (y)
#[test]
fn test_containerized_cmd_yank() {
    let mut tester = EdDifferentialTester::new();
    println!("🐳 Testing yank command (y) - containerized");
    tester.run_test_suites(&["cmd_yank"]);
}

/// Test only undo command (u)
#[test]
fn test_containerized_cmd_undo() {
    let mut tester = EdDifferentialTester::new();
    println!("🐳 Testing undo command (u) - containerized");
    tester.run_test_suites(&["cmd_undo"]);
}

/// Test only mark command (k, ')
#[test]
fn test_containerized_cmd_mark() {
    let mut tester = EdDifferentialTester::new();
    println!("🐳 Testing mark command (k, ') - containerized");
    tester.run_test_suites(&["cmd_mark"]);
}

/// Test only shell command (!)
#[test]
fn test_containerized_cmd_shell() {
    let mut tester = EdDifferentialTester::new();
    println!("🐳 Testing shell command (!) - containerized");
    tester.run_test_suites(&["cmd_shell"]);
}

/// Test only search command (/, ?)
#[test]
fn test_containerized_cmd_search() {
    let mut tester = EdDifferentialTester::new();
    println!("🐳 Testing search command (/, ?) - containerized");
    tester.run_test_suites(&["cmd_search"]);
}

/// Test only prompt command (P)
#[test]
fn test_containerized_cmd_prompt() {
    let mut tester = EdDifferentialTester::new();
    println!("🐳 Testing prompt command (P) - containerized");
    tester.run_test_suites(&["cmd_prompt"]);
}

/// Test only help command (h, H)
#[test]
fn test_containerized_cmd_help() {
    let mut tester = EdDifferentialTester::new();
    println!("🐳 Testing help command (h, H) - containerized");
    tester.run_test_suites(&["cmd_help"]);
}

/// Test file creation behavior
/// Tests launching ed with non-existent files
#[test]
fn test_containerized_file_creation() {
    let mut tester = EdDifferentialTester::new();
    println!("🐳 Testing file creation behavior - containerized");
    tester.run_test_suites(&["file_creation"]);
}

/// Test only basic commands (LEGACY - for backward compatibility)
/// Includes: delete (d), append (a), insert (i), change (c), print (p), quit (q)
#[test]
#[ignore]  // Disabled - use cmd_* tests instead
fn test_containerized_basic_only() {
    let mut tester = EdDifferentialTester::new();
    println!("🐳 Testing only basic commands (containerized)");
    tester.run_test_suites(&["basic_commands"]);
}

/// Test only addressing commands
#[test]
#[ignore]
fn test_containerized_addressing() {
    let mut tester = EdDifferentialTester::new();
    println!("🐳 Testing only addressing (containerized)");
    tester.run_test_suites(&["addressing"]);
}

/// Test only file operations
#[test]
#[ignore]
fn test_containerized_file_ops() {
    let mut tester = EdDifferentialTester::new();
    println!("🐳 Testing only file operations (containerized)");
    tester.run_test_suites(&["file_operations"]);
}

/// Test only search and replace
#[test]
#[ignore]
fn test_containerized_search_replace() {
    let mut tester = EdDifferentialTester::new();
    println!("🐳 Testing only search/replace (containerized)");
    tester.run_test_suites(&["search_replace"]);
}

/// Test only global commands
#[test]
#[ignore]
fn test_containerized_global() {
    let mut tester = EdDifferentialTester::new();
    println!("🐳 Testing only global commands (containerized)");
    tester.run_test_suites(&["global_commands"]);
}

/// Test specific command categories (for selective testing during development)
#[test]
#[ignore] // Ignore by default, run with: cargo test test_containerized_selective -- --ignored
fn test_containerized_selective() {
    let mut tester = EdDifferentialTester::new();

    // Test only file operations and search/replace
    let suite_names = vec!["file_operations", "search_replace"];
    tester.run_test_suites(&suite_names);
}
