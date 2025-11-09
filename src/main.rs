// rust-ed - Memory-safe replacement for GNU ed
// Copyright (C) 2025 Brian Boynton, MD
//
// This file is part of rust-ed.
//
// rust-ed is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// rust-ed is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with rust-ed.  If not, see <https://www.gnu.org/licenses/>.

/// GNU ed main entry point - Rust translation
/// This file matches main.c structure exactly for human review
/// C source: main.c (12,515 bytes) - IMMUTABLE REFERENCE
///
/// ═══════════════════════════════════════════════════════════════════════════
/// 🚨 STRUCTURAL BOUNDARIES - DO NOT VIOLATE 🚨
/// ═══════════════════════════════════════════════════════════════════════════
///
/// THIS FILE (main.rs) SHOULD CONTAIN (matching main.c):
/// ✓ main() - Program entry point and argument parsing
/// ✓ Global flag accessors (extended_regexp, restricted, scripted, etc.)
/// ✓ Help and version display functions
/// ✓ Error display utilities (show_error, show_warning, show_strerror)
/// ✓ Utility functions (parse_addr, interactive, may_access_filename)
/// ✓ Command dispatcher (execute_ed_command) that CALLS main_loop functions
///
/// THIS FILE MUST NOT CONTAIN:
/// ✗ NO execute_*_command() implementations (those belong in main_loop.rs)
/// ✗ NO command parsing logic (belongs in main_loop.rs)
/// ✗ NO address extraction (belongs in main_loop.rs)
/// ✗ NO buffer manipulation beyond dispatch (belongs in main_loop.rs)
///
/// COMMAND EXECUTION BELONGS IN main_loop.rs:
/// - All execute_*_command() functions match main_loop.c:567-730 switch cases
/// - If implementing a command, add it to main_loop.rs, NOT here
/// - This file only dispatches to main_loop functions via execute_ed_command()
///
/// TARGET: ~12-18 functions (currently tracking C source exactly)
/// ═══════════════════════════════════════════════════════════════════════════

use std::env;
use std::process;

mod buffer;
// mod commands; // REMOVED: Old modular approach superseded by main.rs implementations
// mod address; // REMOVED: Functions integrated into main_loop.rs
mod regex;
mod error;
// mod compat; // REMOVED: Functions integrated into main.rs and io.rs
mod main_loop;
mod global;
mod carg_parser;
mod signal;
mod io;

use error::EdError;
use buffer::EdBuffer;
use main_loop::main_loop;
use ::regex::{Regex, RegexBuilder};

// Global configuration flags - converted to safe atomic variables
use std::sync::atomic::{AtomicBool, Ordering};

static EXTENDED_REGEXP: AtomicBool = AtomicBool::new(false);
static RESTRICTED: AtomicBool = AtomicBool::new(false);
static SCRIPTED: AtomicBool = AtomicBool::new(false);
static STRIP_CR: AtomicBool = AtomicBool::new(false);
static TRADITIONAL: AtomicBool = AtomicBool::new(false);
static QUIET: AtomicBool = AtomicBool::new(false);
static SAFE_NAMES: AtomicBool = AtomicBool::new(true);
static PROMPT_ON: AtomicBool = AtomicBool::new(false);

static PROGRAM_NAME: &str = "ed";
static PROGRAM_YEAR: &str = "2025";

/// extended_regexp - matches main.c:62 (now memory safe)
pub fn extended_regexp() -> bool { 
    EXTENDED_REGEXP.load(Ordering::Relaxed)
}

/// restricted - matches main.c:63 (now memory safe)
pub fn restricted() -> bool { 
    RESTRICTED.load(Ordering::Relaxed)
}

/// safe_names_enabled - accessor for SAFE_NAMES global (now memory safe)
pub fn safe_names_enabled() -> bool {
    SAFE_NAMES.load(Ordering::Relaxed)
}

/// scripted - matches main.c:64 (now memory safe)
pub fn scripted() -> bool { 
    SCRIPTED.load(Ordering::Relaxed)
}

/// strip_cr - matches main.c:65 (now memory safe)
pub fn strip_cr() -> bool { 
    STRIP_CR.load(Ordering::Relaxed)
}

/// traditional - matches main.c:66 (now memory safe)
pub fn traditional() -> bool {
    TRADITIONAL.load(Ordering::Relaxed)
}

/// quiet - check if quiet mode is enabled
pub fn quiet() -> bool {
    QUIET.load(Ordering::Relaxed)
}

/// prompt_on - check if prompt is enabled
pub fn prompt_on() -> bool {
    PROMPT_ON.load(Ordering::Relaxed)
}

/// toggle_prompt - toggle prompt flag (GNU ed main_loop.c:668)
pub fn toggle_prompt() {
    let current = PROMPT_ON.load(Ordering::Relaxed);
    PROMPT_ON.store(!current, Ordering::Relaxed);
}

/// show_help - matches main.c:69
fn show_help() {
    println!("GNU ed is a line-oriented text editor. It is used to create, display,");
    println!("modify and otherwise manipulate text files, both interactively and via");
    println!("shell scripts. A restricted version of ed, red, can only edit files in");
    println!("the current directory and cannot execute shell commands. Ed is the");
    println!("'standard' text editor in the sense that it is the original editor for");
    println!("Unix, and thus widely available. For most purposes, however, it is");
    println!("superseded by full-screen editors.");
    println!();
    println!("Usage: {} [options] [[+line] file]", PROGRAM_NAME);
    println!();
    println!("The file name may be preceded by '+line', '+/RE', or '+?RE' to set the");
    println!("current line to the line number specified or to the first or last line");
    println!("matching the regular expression 'RE'.");
    println!();
    println!("The environment variable LINES can be used to set the initial window size.");
    println!();
    println!("Options:");
    println!("  -h, --help                 display this help and exit");
    println!("  -V, --version              output version information and exit");
    println!("  -E, --extended-regexp      use extended regular expressions");
    println!("  -G, --traditional          run in compatibility mode");
    println!("  -l, --loose-exit-status    exit with 0 status even if a command fails");
    println!("  -p, --prompt=STRING        use STRING as an interactive prompt");
    println!("  -q, --quiet, --silent      suppress diagnostics written to stderr");
    println!("  -r, --restricted           run in restricted mode");
    println!("  -s, --script               suppress byte counts and '!' prompt");
    println!("  -v, --verbose              be verbose; equivalent to the 'H' command");
    println!("      --strip-trailing-cr    strip carriage returns at end of text lines");
    println!("      --unsafe-names         allow control characters in file names");
    println!();
    println!("Start edit by reading in 'file' if given.");
    println!("If 'file' begins with a '!', read output of shell command.");
    println!();
    println!("Exit status: 0 for a normal exit, 1 for environmental problems");
    println!("(invalid command-line options, memory exhausted, command failed, etc),");
    println!("2 for problems with the input file (file not found, buffer modified,");
    println!("I/O errors), 3 for an internal consistency error (e.g., bug) which caused");
    println!("ed to panic.");
    println!();
    println!("Report bugs to bug-ed@gnu.org");
    println!("Ed home page: http://www.gnu.org/software/ed/ed.html");
    println!("General help using GNU software: http://www.gnu.org/gethelp");
}

/// show_version - matches main.c:109  
fn show_version() {
    println!("GNU {} {}", PROGRAM_NAME, "1.22.2-rust");
    println!("Copyright (C) 1994 Andrew L. Moore.");
    println!("Copyright (C) {} Antonio Diaz Diaz.", PROGRAM_YEAR);
    println!("License GPLv2+: GNU GPL version 2 or later <http://gnu.org/licenses/gpl.html>");
    println!("This is free software: you are free to change and redistribute it.");
    println!("There is NO WARRANTY, to the extent permitted by law.");
}

/// print_escaped - matches main.c:120
pub fn print_escaped(p: &str, to_stdout: bool) {
    // TODO: Implement escaped character printing matching GNU ed exactly
    if to_stdout {
        print!("{}", p);
    } else {
        eprint!("{}", p);
    }
}

/// show_warning - matches main.c:137
pub fn show_warning(filename: Option<&str>, msg: &str) {
    if !QUIET.load(Ordering::Relaxed) {
        if let Some(fname) = filename {
            if !fname.is_empty() {
                print_escaped(fname, false);
                eprint!(": ");
            }
        }
        eprintln!("{}", msg);
    }
}

/// show_strerror - matches main.c:148
pub fn show_strerror(filename: Option<&str>, errcode: i32) {
    if !QUIET.load(Ordering::Relaxed) {
        if let Some(fname) = filename {
            if !fname.is_empty() {
                print_escaped(fname, false);
                eprint!(": ");
            }
        }
        // TODO: Map errcode to actual system error message
        eprintln!("Error code: {}", errcode);
    }
}

/// show_error - matches main.c:159
fn show_error(msg: &str, errcode: i32, help: bool) {
    if errcode != 0 {
        show_strerror(None, errcode);
    } else {
        show_warning(None, msg);
    }
    if help {
        eprintln!("Try '{} --help' for more information.", PROGRAM_NAME);
    }
}

/// parse_addr - DEPRECATED: moved to carg_parser.rs for C structure alignment  
fn parse_addr(arg: &str) -> i32 {
    // Address parsing functionality moved to carg_parser module
    // Use carg_parser::ap_argument and related functions
    arg.parse().unwrap_or(0)
}

/// interactive - matches main.c:186
pub fn interactive() -> bool {
    // TODO: Implement interactive mode detection 
    // For now, assume interactive if not scripted
    !scripted()
}

/// may_access_filename - matches main.c:193

/// main - matches main.c:218 (PROGRAM ENTRY POINT)
fn main() {
    let args: Vec<String> = env::args().collect();
    
    let mut initial_error = false;
    let loose = false;
    
    // Argument parsing moved to carg_parser.rs module to match C structure
    // Use carg_parser::ap_init, carg_parser::ap_code, etc. for GNU ed compatible parsing
    // TODO: Integrate with carg_parser module functions
    let mut filename: Option<String> = None;
    
    // Temporary simplified parsing until full carg_parser integration
    for arg in args.iter().skip(1) {
        if arg.starts_with('-') {
            match arg.as_str() {
                "-h" | "--help" => {
                    show_help();
                    process::exit(0);
                },
                "-V" | "--version" => {
                    show_version();
                    process::exit(0);
                },
                _ => {
                    show_error(&format!("Unknown option: {}", arg), 0, true);
                    process::exit(1);
                }
            }
        } else if main_loop::may_access_filename(arg) {
            filename = Some(arg.clone());
            break;
        }
    }
    
    // Initialize buffers (matches C init_buffers())
    let mut buffer = EdBuffer::new();
    
    // Load initial file if provided
    if let Some(fname) = filename {
        if fname.starts_with('!') {
            // TODO: Handle shell command input
            initial_error = true;
        } else {
            // Set default filename and load file
            buffer.set_filename(fname.clone());
            
            // Call first_e_command equivalent
            // GNU ed behavior: missing files print error to stderr but don't exit
            match buffer.load_file(&fname) {
                Ok(bytes_read) => {
                    // File exists (even if empty) - print byte count
                    if !scripted() {
                        println!("{}", bytes_read);
                    }
                },
                Err(EdError::FileNotFound) => {
                    // File doesn't exist - already printed to stderr in load_file
                    // GNU ed: don't print byte count, don't exit, continue with empty buffer
                    // Do nothing - just continue to main_loop
                },
                Err(_) => {
                    // Real I/O errors (not just missing file)
                    initial_error = true;
                    if !interactive() {
                        process::exit(2);
                    }
                }
            }
        }
    }
    
    // Call main_loop (matches C main_loop call)
    let exit_code = main_loop(initial_error, loose, &mut buffer);
    process::exit(exit_code);
}

// Temporary delegation functions for compatibility during transition
pub fn execute_command(buffer: &mut EdBuffer, command_line: &str) -> Result<(), EdError> {
    // This is the old implementation - will be moved to main_loop.rs
    // PHASE 1: Extract addresses (GNU ed extract_addresses)
    let extraction = main_loop::extract_addresses(command_line, buffer)?;
    
    // PHASE 2: Get clean command character (GNU ed c = *(*ibufpp)++)
    let clean_command = extraction.remaining_command.trim();
    if clean_command.is_empty() {
        // Empty command - handle address navigation
        return handle_empty_command(buffer, &extraction);
    }
    
    let command_char = clean_command.chars().next().unwrap_or('\0');
    let command_args = if clean_command.len() > 1 { &clean_command[1..] } else { "" };
    
    // PHASE 3: Execute command (GNU ed exec_command)
    execute_ed_command(buffer, command_char, command_args, &extraction)
}

fn handle_empty_command(buffer: &mut EdBuffer, extraction: &main_loop::AddressExtraction) -> Result<(), EdError> {
    // Handle address-only navigation (like "5" to go to line 5, "+1" for relative)
    // GNU ed uses second_addr as the final computed address (main_loop.c:739-742)
    // NOTE: Only print when it's a bare address, not when address is followed by command

    if extraction.second_addr >= 0 {
        // Address was provided - navigate to it
        let addr = extraction.second_addr as usize;
        if addr > 0 && addr <= buffer.len() {
            buffer.set_current_line(addr)?;
            // Print the line ONLY if remaining command is truly empty (GNU ed behavior)
            // If there's a comma or other trailing chars, it's part of a larger command
            if extraction.remaining_command.trim().is_empty() {
                if let Some(line) = buffer.get_line(addr) {
                    println!("{}", line);
                }
            }
        } else {
            return Err(EdError::InvalidAddress);
        }
    } else {
        // No address - this is a bare newline command (GNU ed main_loop.c:739-742)
        // Navigate to next line and print it: current_addr() + 1
        let current = buffer.current_line();
        let next_line = current + 1;

        if next_line > buffer.len() {
            // Trying to navigate past EOF - return error (GNU ed behavior)
            return Err(EdError::InvalidAddress);
        }

        buffer.set_current_line(next_line)?;
        if let Some(line) = buffer.get_line(next_line) {
            println!("{}", line);
        }
    }
    Ok(())
}

fn execute_ed_command(
    buffer: &mut EdBuffer,
    command_char: char,
    command_args: &str,
    addresses: &main_loop::AddressExtraction
) -> Result<(), EdError> {
    match command_char {
        'p' => main_loop::execute_print_command(buffer, addresses),
        'q' => {
            // Quit shouldn't have an address (GNU ed main_loop.c:667 unexpected_address)
            if addresses.addr_count > 0 {
                return Err(EdError::InvalidAddress);
            }
            main_loop::execute_quit_command(buffer, false)
        },
        'Q' => {
            // Unconditional quit also shouldn't have an address
            if addresses.addr_count > 0 {
                return Err(EdError::InvalidAddress);
            }
            main_loop::execute_quit_command(buffer, true)
        },
        'a' => {
            buffer.clear_undo_stack();
            main_loop::append_text_input(buffer, addresses)
        },
        'd' => {
            buffer.clear_undo_stack();
            main_loop::execute_delete_command(buffer, addresses)
        },
        'i' => {
            buffer.clear_undo_stack();
            main_loop::insert_text_input(buffer, addresses)
        },
        'c' => {
            buffer.clear_undo_stack();
            main_loop::execute_change_command(buffer, addresses)
        },
        'l' => main_loop::execute_list_command(buffer, addresses),
        'n' => main_loop::execute_number_command(buffer, addresses),
        '=' => main_loop::execute_line_number_command(buffer, addresses),
        'u' => main_loop::undo_last_operation(buffer),
        's' => {
            buffer.clear_undo_stack();
            main_loop::execute_substitute_command(buffer, command_args, addresses)
        },
        'w' => {
            buffer.clear_undo_stack();
            main_loop::execute_write_command(buffer, command_args, addresses, false)
        },
        'W' => {
            buffer.clear_undo_stack();
            main_loop::execute_write_command(buffer, command_args, addresses, true)
        },
        'r' => main_loop::execute_read_command(buffer, command_args, addresses),
        'e' => main_loop::execute_edit_command(buffer, command_args),
        'E' => main_loop::execute_edit_force(buffer, command_args),
        'f' => main_loop::execute_filename_command(buffer, command_args, addresses),
        '!' => main_loop::execute_shell_command_with_buffer(buffer, command_args, addresses),
        '1'..='9' | '0' => {
            let line_str = format!("{}{}", command_char, command_args);
            if let Ok(line_num) = line_str.parse::<usize>() {
                if line_num > 0 && line_num <= buffer.len() {
                    buffer.set_current_line(line_num)?;
                    if let Some(line) = buffer.get_line(line_num) {
                        println!("{}", line);
                    }
                } else {
                    return Err(EdError::InvalidAddress);
                }
            } else {
                return Err(EdError::InvalidCommand);
            }
            Ok(())
        },
        'j' => main_loop::execute_join_command(buffer, addresses),
        'm' => main_loop::execute_move_command(buffer, command_args, addresses),
        't' => main_loop::execute_copy_command(buffer, command_args, addresses),
        'k' => main_loop::execute_mark_command(buffer, command_args, addresses),
        '\'' => main_loop::execute_goto_mark_command(buffer, command_args),
        'g' => main_loop::execute_global_command(buffer, command_args, addresses, true, false),   // match = true, interactive = false for 'g'
        'v' => main_loop::execute_global_command(buffer, command_args, addresses, false, false),  // match = false, interactive = false for 'v'
        'G' => main_loop::execute_global_command(buffer, command_args, addresses, true, true),    // match = true, interactive = true for 'G'
        'V' => main_loop::execute_global_command(buffer, command_args, addresses, false, true),   // match = false, interactive = true for 'V'
        '?' => main_loop::execute_backward_search(buffer, command_args, addresses),
        '/' => main_loop::execute_forward_search(buffer, command_args, addresses),
        'h' => main_loop::execute_help_command(),
        'H' => main_loop::execute_verbose_help_command(),
        'P' => main_loop::execute_prompt_command(),
        'z' => main_loop::execute_scroll_command(buffer, command_args, addresses),
        'y' => main_loop::execute_yank_command(buffer, addresses),
        'x' => main_loop::execute_put_command(buffer, addresses),
        _ => Err(EdError::InvalidCommand),
    }
}