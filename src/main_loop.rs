/// GNU ed main command loop - Rust translation
/// This file matches main_loop.c structure exactly for human review
/// C source: main_loop.c (32,051 bytes) - IMMUTABLE REFERENCE

use std::io::{self, Write};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Mutex;
use crate::buffer::EdBuffer;
use crate::error::EdError;
use regex::{Regex, RegexBuilder};

/// Address types moved from address.rs - these belong in main_loop.c according to GNU ed structure

/// Address types supported by ed
#[derive(Debug, Clone, PartialEq)]
pub enum Address {
    Line(usize),        // Numeric line (1-based)
    Current,            // Current line (.)
    Last,               // Last line ($)
    Relative(i32),      // Relative (+1, -2, etc.)
    Pattern(String),    // Regex pattern (/pattern/)
}

/// Address range for commands that operate on ranges
#[derive(Debug, Clone, PartialEq)]
pub struct AddressRange {
    pub start: Address,
    pub end: Option<Address>,
}

/// Address extraction result (matching GNU ed extract_addresses)
#[derive(Debug)]
#[derive(Default)]
pub struct AddressExtraction {
    pub first_addr: i32,    // -1 if undefined (GNU ed format)
    pub second_addr: i32,   // -1 if undefined
    pub addr_count: i32,    // Number of addresses found
    pub remaining_command: String, // Command after addresses are removed
}

impl AddressExtraction {
    pub fn has_no_addresses(&self) -> bool {
        self.addr_count == 0
    }
}

// Static state matching main_loop.c
// Global state converted to safe Rust - matches main_loop.c functionality
static VERBOSE: AtomicBool = AtomicBool::new(false);
static ERROR_MSG: Mutex<&'static str> = Mutex::new("");

/// first_e_command - matches main_loop.c:46
pub fn first_e_command(filename: &str) -> i32 {
    // TODO: Implement initial e command handling
    0
}

/// invalid_address - matches main_loop.c:49  
pub fn invalid_address() {
    set_error_msg("Invalid address");
}

/// error_msg - matches main_loop.c:64 (now memory safe)
pub fn error_msg() -> &'static str {
    ERROR_MSG.lock().map_or("", |guard| *guard)
}

/// set_error_msg - matches main_loop.c:66 (now memory safe)
pub fn set_error_msg(msg: &'static str) {
    if let Ok(mut guard) = ERROR_MSG.lock() {
        *guard = msg;
    }
}

/// set_def_filename - matches main_loop.c:51
pub fn set_def_filename(s: &str) -> bool {
    // TODO: Implement filename setting
    true
}

/// set_prompt - matches main_loop.c:72
pub fn set_prompt(s: &str) -> bool {
    // TODO: Implement prompt setting
    true
}

/// set_verbose - matches main_loop.c:85 (now memory safe)
/// Toggles verbose mode (GNU ed: verbose = !verbose)
pub fn set_verbose() {
    let current = VERBOSE.load(Ordering::Relaxed);
    VERBOSE.store(!current, Ordering::Relaxed);
}

/// verbose - check if verbose mode is enabled
pub fn verbose() -> bool {
    VERBOSE.load(Ordering::Relaxed)
}

/// mark_line_node - matches main_loop.c:91
fn mark_line_node(lp: usize, c: char) -> bool {
    // TODO: Implement line marking
    true
}

/// unmark_line_node - matches main_loop.c:101
pub fn unmark_line_node(lp: usize) {
    // TODO: Implement line unmarking
}

/// get_marked_node_addr - matches main_loop.c:111
fn get_marked_node_addr(c: char) -> i32 {
    // TODO: Implement marked node address retrieval
    0
}


/// skip_blanks - matches main_loop.c:170
fn skip_blanks(input: &str) -> &str {
    input.trim_start()
}

/// get_filename - matches main_loop.c:177
/// Implements filename parsing with tilde expansion
/// C source reference: main_loop.c:177-207
/// Tilde expansion: main_loop.c:194-196
pub fn get_filename(ibufpp: &str) -> Option<String> {
    let input = ibufpp.trim();

    // Check for shell command (main_loop.c:193)
    if input.starts_with('!') {
        return None; // Shell commands handled separately
    }

    // Tilde expansion (main_loop.c:194-196)
    // if( **ibufpp == '~' && (*ibufpp)[1] == '/' )
    if input.starts_with("~/") {
        // Get home directory (main_loop.c:195)
        if let Some(home) = crate::signal::home_directory() {
            if !home.is_empty() {
                // Replace ~ with home directory (main_loop.c:196,200)
                let rest = &input[1..]; // Remove '~', keep '/'
                return Some(format!("{}{}", home, rest));
            }
        }
    }

    // No tilde expansion - return as-is
    if input.is_empty() {
        None
    } else {
        Some(input.to_string())
    }
}

/// parse_int - matches main_loop.c:211
fn parse_int(input: &str) -> Option<(i32, &str)> {
    // TODO: Implement integer parsing
    None
}

/// extract_addresses - matches main_loop.c:232 (CRITICAL FUNCTION)
/// This is moved from address.rs to match C structure
pub fn extract_addresses_main_loop(command_line: &str, buffer: &crate::buffer::EdBuffer) -> Result<AddressExtraction, EdError> {
    // Use existing implementation from address.rs
    extract_addresses(command_line, buffer)
}

/// get_third_addr - matches main_loop.c:311
fn get_third_addr(ibufpp: &str) -> Option<i32> {
    // TODO: Implement third address parsing
    None
}

/// set_addr_range - matches main_loop.c:329
fn set_addr_range(n: i32, m: i32, addr_cnt: i32) -> bool {
    // TODO: Implement address range setting
    true
}

/// set_addr_range2 - matches main_loop.c:338
fn set_addr_range2(addr_cnt: i32) -> bool {
    // TODO: Implement address range setting variant
    true
}

/// set_second_addr - matches main_loop.c:344
fn set_second_addr(addr: i32, addr_cnt: i32) -> bool {
    // TODO: Implement second address setting
    true
}

/// get_command_suffix - matches main_loop.c:354
fn get_command_suffix(ibufpp: &str) -> Option<String> {
    // TODO: Implement command suffix parsing
    None
}

/// get_command_s_suffix - matches main_loop.c:373
fn get_command_s_suffix(ibufpp: &str) -> Option<String> {
    // TODO: Implement substitute command suffix parsing
    None
}

/// unexpected_address - matches main_loop.c:404
fn unexpected_address(addr_cnt: i32) -> bool {
    // TODO: Implement unexpected address detection
    false
}

/// unexpected_command_suffix - matches main_loop.c:410
fn unexpected_command_suffix(ch: char) -> bool {
    // TODO: Implement unexpected suffix detection
    false
}

/// command_s - matches main_loop.c:418 (COMPLEX SUBSTITUTE)
fn command_s(ibufpp: &str, pflagsp: &mut i32, buffer: &mut EdBuffer, addresses: &AddressExtraction) -> bool {
    // TODO: Move substitute implementation from main.rs
    // This should match our current execute_substitute_command function
    true
}

/// get_tmpname - matches main_loop.c:492
fn get_tmpname(init: bool) -> Option<String> {
    // TODO: Implement temporary name generation
    None
}

/// command_shell - matches main_loop.c:514
fn command_shell(ibufpp: &str, addr_cnt: i32, buffer: &mut EdBuffer, addresses: &AddressExtraction) -> bool {
    // Call the actual shell command implementation
    match execute_shell_command_with_buffer(buffer, ibufpp, addresses) {
        Ok(()) => true,
        Err(_) => {
            set_error_msg("Shell command failed");
            false
        }
    }
}

/// exec_global - DEPRECATED: moved to global.rs for C structure alignment
fn exec_global(_ibufpp: &str, _pflags: i32, _buffer: &mut EdBuffer, _addresses: &AddressExtraction) -> i32 {
    // Global command implementation moved to global.rs module
    // Use global::clear_active_list, global::set_active_node, etc.
    0
}

/// exec_command - matches main_loop.c:555 (MAIN COMMAND EXECUTOR)
fn exec_command(ibufpp: &str, isglobal: bool, buffer: &mut EdBuffer) -> i32 {
    // TODO: Move main command execution logic from main.rs execute_ed_command
    0
}

/// main_loop - matches main_loop.c:809 (MAIN COMMAND LOOP)
pub fn main_loop(initial_error: bool, loose: bool, buffer: &mut EdBuffer) -> i32 {
    // TODO: Move main loop logic from main.rs run function
    
    let mut had_error = initial_error;
    
    loop {
        // Print prompt if enabled (GNU ed main loop prints "*" when prompt_on)
        if crate::prompt_on() {
            print!("*");
            use std::io::Write;
            std::io::stdout().flush().unwrap_or(());
        }

        // Read command line
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(0) => break, // EOF
            Ok(_) => {},
            Err(_) => {
                had_error = true;
                break;
            }
        }
        
        let command_line = input.trim();
        
        // Process command using GNU ed architecture
        match execute_command_wrapper(buffer, command_line) {
            Ok(()) => {},
            Err(EdError::Quit) => break,
            Err(err) => {
                // Set error message based on error type (GNU ed set_error_msg)
                match &err {
                    EdError::NothingToPut => set_error_msg("Nothing to put"),
                    EdError::NothingToUndo => set_error_msg("Nothing to undo"),
                    EdError::InvalidAddress => set_error_msg("Invalid address"),
                    EdError::InvalidCommand => set_error_msg("Invalid command"),
                    EdError::InvalidFilename => set_error_msg("Invalid filename"),
                    EdError::PatternNotFound => set_error_msg("Pattern not found"),
                    _ => set_error_msg("Error"),
                }

                // Print "?" (GNU ed always prints this)
                println!("?");

                // If verbose mode, also print the error message
                if verbose() {
                    let msg = error_msg();
                    if !msg.is_empty() {
                        println!("{}", msg);
                    }
                }

                had_error = true;
            }
        }
    }
    
    if had_error { 1 } else { 0 }
}

/// get_filename - moved from buffer.rs to match C source structure in main_loop.c:177
pub fn get_filename_from_buffer(buffer: &EdBuffer) -> Option<&str> {
    // Moved from buffer.rs to match GNU ed C source structure (main_loop.c)
    buffer.filename.as_deref()
}

/// may_access_filename - moved from main.rs to match C source structure (main_loop.c)
pub fn may_access_filename(name: &str) -> bool {
    // Moved from main.rs to match GNU ed C source structure (main_loop.c)
    // Implement restricted mode filename checking
    if super::restricted() {
        // In restricted mode, only allow files in current directory
        if name.contains('/') || name.starts_with("..") {
            return false;
        }
    }
    
    // Check for control characters if safe_names is enabled
    if super::safe_names_enabled() {
        for ch in name.chars() {
            if (ch as u32 >= 1 && ch as u32 <= 31) || ch as u32 == 127 {
                // set_error_msg("Control characters not allowed in file names");
                return false;
            }
        }
    }
    true
}

/// get_line_node_addr - moved from buffer.rs to match C source structure (main_loop.c)
pub fn get_line_node_addr(buffer: &crate::buffer::EdBuffer, line_num: usize) -> usize {
    // Moved from buffer.rs to match GNU ed C source structure (main_loop.c)
    // In Rust, we use direct indexing
    line_num
}

/// TODO: COMMAND EXECUTION FUNCTIONS BELONG HERE (from main.rs)
/// According to GNU ed C source structure, main_loop.c contains the command switch statement
/// and all command execution logic. The following functions should be moved here from main.rs:
///
/// Core command execution functions (currently in main.rs):
/// - execute_ed_command() - main command dispatcher (maps to main_loop.c switch statement)
/// - execute_print_command() - 'p' command (case 'p' in main_loop.c:648)  
/// - execute_delete_command() - 'd' command (case 'd' in main_loop.c:580)
/// - execute_change_command() - 'c' command (case 'c' in main_loop.c:572)
/// - execute_list_command() - 'l' command (case 'l' in main_loop.c:646)
/// - execute_number_command() - 'n' command (case 'n' in main_loop.c:647)
/// - execute_line_number_command() - '=' command (number display)
/// - execute_substitute_command() - 's' command (case 's' in main_loop.c:681)
/// - execute_join_command() - 'j' command (case 'j' in main_loop.c:634)
/// - execute_move_command() - 'm' command (case 'm' in main_loop.c:655)
/// - execute_copy_command() - 't' command (case 't' in main_loop.c:684)
/// - execute_mark_command() - 'k' command (case 'k' in main_loop.c:640)
/// - execute_goto_mark_command() - "'" command (goto mark)
/// - execute_global_command() - 'g' command (case 'g' in main_loop.c:609)
/// - execute_inverse_global_command() - 'v' command (case 'v' in main_loop.c:610)
/// - execute_backward_search() - '?' command (backward pattern search)
/// - execute_forward_search() - '/' command (forward pattern search)
/// - execute_help_command() - 'h' command (case 'h' in main_loop.c:622)
/// - execute_verbose_help_command() - 'H' command (case 'H' in main_loop.c:623)
/// - execute_prompt_command() - 'P' command (case 'P' in main_loop.c:664)
///
/// These functions implement the command cases from main_loop.c:567-730 switch statement

/// Command execution functions - moved from main.rs to match main_loop.c structure

/// execute_print_command - moved from main.rs (case 'p' in main_loop.c:648)
pub fn execute_print_command(buffer: &EdBuffer, addresses: &AddressExtraction) -> Result<(), EdError> {
    let (start, end) = get_address_range(buffer, addresses)?;
    
    for line_num in start..=end {
        if let Some(line) = buffer.get_line(line_num) {
            println!("{}", line);
        }
    }
    Ok(())
}

/// execute_delete_command - moved from main.rs (case 'd' in main_loop.c:580)
pub fn execute_delete_command(buffer: &mut crate::buffer::EdBuffer, addresses: &AddressExtraction) -> Result<(), EdError> {
    let (start, end) = get_address_range(buffer, addresses)?;
    
    // Delete lines in reverse order to maintain line numbers
    for line_num in (start..=end).rev() {
        buffer.delete_line(line_num)?;
    }
    Ok(())
}

/// escape_special_chars - moved from main.rs (helper for list command)
pub fn escape_special_chars(line: &str) -> String {
    line.chars().map(|c| match c {
        '$' => "\\$".to_string(),
        '\\' => "\\\\".to_string(),
        '\t' => "\\t".to_string(),
        '\n' => "\\n".to_string(),
        '\r' => "\\r".to_string(),
        c if c.is_control() => format!("\\{:03o}", c as u8),
        c => c.to_string(),
    }).collect()
}

/// execute_list_command - moved from main.rs (case 'l' in main_loop.c:646)
pub fn execute_list_command(buffer: &EdBuffer, addresses: &AddressExtraction) -> Result<(), EdError> {
    let (start, end) = get_address_range(buffer, addresses)?;
    
    for line_num in start..=end {
        if let Some(line) = buffer.get_line(line_num) {
            println!("{}$", escape_special_chars(line));
        }
    }
    Ok(())
}

/// Extract addresses from command line - moved from address.rs to match main_loop.c structure
/// This function corresponds to address parsing in main_loop.c
pub fn extract_addresses(command_line: &str, buffer: &crate::buffer::EdBuffer) -> Result<AddressExtraction, EdError> {
    let mut first_addr = -1i32;
    let mut second_addr = -1i32;  
    let mut addr_count = 0;
    let mut chars = command_line.char_indices().peekable();
    let mut pos = 0;
    
    // Skip leading blanks (GNU ed skip_blanks)
    while let Some(&(idx, ch)) = chars.peek() {
        if ch.is_whitespace() {
            chars.next();
            pos = idx + ch.len_utf8();
        } else {
            break;
        }
    }
    
    // Parse addresses (simplified version of GNU ed logic)
    while let Some((_idx, ch)) = chars.peek().copied() {
        match ch {
            // Numeric address
            '0'..='9' => {
                let mut num_str = String::new();
                while let Some((_, digit_ch)) = chars.peek() {
                    if digit_ch.is_ascii_digit() {
                        num_str.push(*digit_ch);
                        chars.next();
                        pos += 1;
                    } else {
                        break;
                    }
                }
                
                if let Ok(addr) = num_str.parse::<i32>() {
                    if first_addr == -1 {
                        first_addr = addr;
                        second_addr = addr;
                        addr_count = 1;
                    } else {
                        first_addr = second_addr;
                        second_addr = addr;
                        addr_count = 2;
                    }
                }
            },
            // Relative address (+ or -) - GNU ed main_loop.c:252-261
            '+' | '-' => {
                let is_plus = ch == '+';
                chars.next(); // consume '+' or '-'
                pos += 1;

                // If first address, set second_addr to current_addr (GNU ed line 253)
                if first_addr == -1 {
                    let current_line = buffer.current_line() as i32;
                    second_addr = current_line;
                }

                // Check if there's a digit after + or - (GNU ed line 254-257)
                if let Some(&(_, next_ch)) = chars.peek() {
                    if next_ch.is_ascii_digit() {
                        // Parse the number (e.g., +5, -3)
                        let mut num_str = String::new();
                        while let Some((_, digit_ch)) = chars.peek() {
                            if digit_ch.is_ascii_digit() {
                                num_str.push(*digit_ch);
                                chars.next();
                                pos += 1;
                            } else {
                                break;
                            }
                        }

                        if let Ok(offset) = num_str.parse::<i32>() {
                            if is_plus {
                                second_addr += offset;
                            } else {
                                second_addr -= offset;
                            }
                        }
                    } else {
                        // Just + or - without number means ±1 (GNU ed line 259-260)
                        if is_plus {
                            second_addr += 1;
                        } else {
                            second_addr -= 1;
                        }
                    }
                } else {
                    // No character after + or -, means ±1
                    if is_plus {
                        second_addr += 1;
                    } else {
                        second_addr -= 1;
                    }
                }

                // Update address tracking (GNU ed sets first=false after processing address)
                if first_addr == -1 {
                    first_addr = second_addr;
                    addr_count = 1;
                } else {
                    first_addr = second_addr;
                    addr_count = 2;
                }
            },
            // Current line
            '.' => {
                chars.next();
                pos += 1;
                // Get actual current line number from buffer
                let current_line = buffer.current_line() as i32;
                if first_addr == -1 {
                    first_addr = current_line;
                    second_addr = current_line;
                    addr_count = 1;
                } else {
                    first_addr = second_addr;
                    second_addr = current_line;
                    addr_count = 2;
                }
            },
            // Last line
            '$' => {
                chars.next();
                pos += 1;
                // Get actual last line number from buffer
                let last_line = buffer.len() as i32;
                if first_addr == -1 {
                    first_addr = last_line;
                    second_addr = last_line;
                    addr_count = 1;
                } else {
                    first_addr = second_addr;
                    second_addr = last_line;
                    addr_count = 2;
                }
            },
            // All lines address - GNU ed main_loop.c:277-290
            // In GNU ed, % is treated same as , (both mean 1,$)
            '%' | ',' => {
                chars.next();
                pos += 1;
                // If no first address yet, % or , means "1,$" (all lines)
                if first_addr == -1 {
                    first_addr = 1;
                    second_addr = buffer.len() as i32;
                    addr_count = 2;
                } else {
                    // Comma after an address: shift addresses (GNU ed line 284)
                    // first_addr = second_addr
                    first_addr = second_addr;
                    addr_count = 1; // We now have one address, waiting for potential second
                    // Continue parsing - don't automatically extend to last line
                    // Only if standalone "," at start means 1,$
                }
            },
            // Range separator semicolon
            ';' => {
                chars.next();
                pos += 1;
            },
            // Mark address (GNU ed main_loop.c:272-276)
            '\'' => {
                chars.next(); // consume the '\''
                pos += 1;

                // Get the mark character (GNU ed: *(*ibufpp)++)
                if let Some((_, mark_char)) = chars.next() {
                    pos += 1;

                    // Get the marked line address (GNU ed: get_marked_node_addr)
                    match buffer.get_marked_node_addr(mark_char) {
                        Ok(marked_line) => {
                            let marked_addr = marked_line as i32;
                            if first_addr == -1 {
                                first_addr = marked_addr;
                                second_addr = marked_addr;
                                addr_count = 1;
                            } else {
                                first_addr = second_addr;
                                second_addr = marked_addr;
                                addr_count = 2;
                            }
                        },
                        Err(_) => {
                            // Invalid mark character or mark not set
                            return Err(EdError::InvalidAddress);
                        }
                    }
                } else {
                    // Missing mark character after '\''
                    return Err(EdError::InvalidCommand);
                }
            },
            // Forward search / (GNU ed main_loop.c:267-271)
            '/' => {
                // Parse forward search pattern - matches GNU ed logic exactly
                let remaining_input = &command_line[pos..];
                let mut search_input = remaining_input;

                match crate::regex::next_matching_node_addr_with_buffer(&mut search_input, buffer) {
                    Ok(found_addr) => {
                        let addr = found_addr as i32;
                        if first_addr == -1 {
                            first_addr = addr;
                            second_addr = addr;
                            addr_count = 1;
                        } else {
                            first_addr = second_addr;
                            second_addr = addr;
                            addr_count = 2;
                        }

                        // Update position to after the search pattern
                        let consumed = remaining_input.len() - search_input.len();
                        pos += consumed;

                        // Advance chars iterator to match position
                        while chars.peek().is_some() && pos > chars.peek().unwrap().0 {
                            chars.next();
                        }
                    },
                    _ => {
                        // Search failed or pattern not found
                        return Err(EdError::PatternNotFound);
                    }
                }
            },
            // Backward search ? (GNU ed main_loop.c:267-271)
            '?' => {
                // Parse backward search pattern - matches GNU ed logic exactly
                let remaining_input = &command_line[pos..];
                let mut search_input = remaining_input;

                match crate::regex::next_matching_node_addr_with_buffer(&mut search_input, buffer) {
                    Ok(found_addr) => {
                        let addr = found_addr as i32;
                        if first_addr == -1 {
                            first_addr = addr;
                            second_addr = addr;
                            addr_count = 1;
                        } else {
                            first_addr = second_addr;
                            second_addr = addr;
                            addr_count = 2;
                        }

                        // Update position to after the search pattern
                        let consumed = remaining_input.len() - search_input.len();
                        pos += consumed;

                        // Advance chars iterator to match position
                        while chars.peek().is_some() && pos > chars.peek().unwrap().0 {
                            chars.next();
                        }
                    },
                    _ => {
                        // Search failed or pattern not found
                        return Err(EdError::PatternNotFound);
                    }
                }
            },
            _ => break,
        }
    }
    
    // Extract remaining command
    let remaining_command = if pos < command_line.len() {
        command_line[pos..].to_string()
    } else {
        String::new()
    };
    
    Ok(AddressExtraction {
        first_addr,
        second_addr,
        addr_count,
        remaining_command,
    })
}

/// get_address_range - moved from main.rs to main_loop.rs (core command processing)
/// Matches GNU ed set_addr_range logic from main_loop.c:329-335
pub fn get_address_range(buffer: &EdBuffer, addresses: &AddressExtraction) -> Result<(usize, usize), EdError> {
    // GNU ed: if first_addr < 1, it's invalid (addresses are 1-based)
    // In our system: first_addr >= 0 means address was set, first_addr == -1 means not set
    let start = if addresses.first_addr >= 0 {
        addresses.first_addr as usize
    } else {
        buffer.current_line()
    };
    let end = if addresses.second_addr >= 0 {
        addresses.second_addr as usize
    } else {
        start
    };

    // GNU ed main_loop.c:332: if( first_addr < 1 || first_addr > second_addr || second_addr > last_addr() )
    if start < 1 || end < 1 || start > buffer.len() || end > buffer.len() || start > end {
        return Err(EdError::InvalidAddress);
    }

    Ok((start, end))
}

// Shell command state - matches GNU ed static variables in get_shell_command
static PREVIOUS_SHELL_COMMAND: Mutex<Option<String>> = Mutex::new(None);

/// get_shell_command - matches GNU ed main_loop.c:120-167
/// Parse and expand shell command, handling '!' repetition and '%' filename expansion
fn get_shell_command(command_args: &str, buffer: &EdBuffer) -> Result<String, EdError> {
    if crate::restricted() {
        return Err(EdError::InvalidCommand);
    }

    let mut command = String::new();
    let mut chars = command_args.chars().peekable();
    let mut replacement = false;

    // Handle command repetition with '!!'
    if let Some('!') = chars.peek() {
        chars.next(); // consume the '!'
        if let Ok(guard) = PREVIOUS_SHELL_COMMAND.lock() {
            if let Some(ref prev_cmd) = *guard {
                if !prev_cmd.is_empty() && (!crate::traditional() || prev_cmd.len() > 1) {
                    command.push_str(prev_cmd);
                    replacement = true;
                } else {
                    return Err(EdError::InvalidCommand);
                }
            } else {
                return Err(EdError::InvalidCommand);
            }
        }
    } else {
        // Add '!' prefix if not present
        command.push('!');
    }

    // Process remaining command, handling '%' filename expansion
    while let Some(ch) = chars.next() {
        if ch == '%' {
            // Replace '%' with default filename
            if let Some(filename) = get_filename_from_buffer(buffer) {
                if !filename.is_empty() {
                    command.push_str(&filename);
                    replacement = true;
                } else {
                    return Err(EdError::InvalidCommand);
                }
            } else {
                return Err(EdError::InvalidCommand);
            }
        } else if ch == '\\' {
            // Handle escape sequences
            if let Some(next_ch) = chars.next() {
                if next_ch != '%' {
                    command.push('\\');
                }
                command.push(next_ch);
            } else {
                command.push(ch);
            }
        } else {
            command.push(ch);
        }
    }

    // Store command for future repetition
    if let Ok(mut guard) = PREVIOUS_SHELL_COMMAND.lock() {
        *guard = Some(command.clone());
    }

    // Print expanded command if replacement occurred (GNU ed behavior)
    if replacement && !crate::scripted() {
        crate::print_escaped(&command[1..], true); // Skip initial '!'
        println!();
        std::io::stdout().flush().unwrap_or(());
    }

    Ok(command)
}

/// execute_shell_command - matches GNU ed command_shell from main_loop.c:514-548
/// Implements both shell escape (!command) and line filtering (1,5!sort)
pub fn execute_shell_command(command_args: &str, addresses: &AddressExtraction) -> Result<(), EdError> {
    let buffer = &EdBuffer::new(); // TODO: Get actual buffer reference
    let full_command = get_shell_command(command_args, buffer)?;

    if addresses.addr_count == 0 {
        // Shell escape command - execute and return
        execute_shell_escape(&full_command[1..]) // Skip '!' prefix
    } else {
        // Line filtering command - process lines through shell command
        execute_shell_filter(&full_command[1..], addresses, buffer)
    }
}

/// execute_shell_command_with_buffer - shell command with actual buffer reference
pub fn execute_shell_command_with_buffer(buffer: &mut EdBuffer, command_args: &str, addresses: &AddressExtraction) -> Result<(), EdError> {
    let full_command = get_shell_command(command_args, buffer)?;

    if addresses.addr_count == 0 {
        // Shell escape command - execute and return
        execute_shell_escape(&full_command[1..]) // Skip '!' prefix
    } else {
        // Line filtering command - process lines through shell command
        execute_shell_filter_with_buffer(&full_command[1..], addresses, buffer)
    }
}

/// execute_shell_escape - simple shell command execution (GNU ed main_loop.c:519-525)
fn execute_shell_escape(command: &str) -> Result<(), EdError> {
    use std::process::Command;

    let status = Command::new("/bin/sh")
        .arg("-c")
        .arg(command)
        .status()
        .map_err(|_| EdError::InvalidCommand)?;

    if !status.success() {
        return Err(EdError::InvalidCommand);
    }

    // Print "!" to indicate shell command completion (GNU ed behavior)
    if !crate::scripted() {
        println!("!");
    }

    Ok(())
}

/// execute_shell_filter - filter lines through shell command (GNU ed main_loop.c:526-548)
fn execute_shell_filter(command: &str, addresses: &AddressExtraction, buffer: &EdBuffer) -> Result<(), EdError> {
    use std::process::{Command, Stdio};
    use std::io::Write;

    // Check for redirection - not allowed in filter mode
    if command.contains('<') || command.contains('>') {
        return Err(EdError::InvalidCommand);
    }

    // Get address range for filtering
    let (first_addr, second_addr) = get_address_range(buffer, addresses)?;

    // Create temporary file for input/output (GNU ed get_tmpname logic)
    let temp_dir = std::env::temp_dir();
    let temp_file = temp_dir.join(format!("ed-{}", std::process::id()));

    // Write addressed lines to temporary file
    let mut lines_to_filter = Vec::new();
    for line_num in first_addr..=second_addr {
        if let Some(line) = buffer.get_line(line_num) {
            lines_to_filter.push(line.to_string());
        }
    }

    // Execute shell command with lines as input
    let full_command = format!("{} > {} 2>&1", command, temp_file.display());

    let mut child = Command::new("/bin/sh")
        .arg("-c")
        .arg(&full_command)
        .stdin(Stdio::piped())
        .spawn()
        .map_err(|_| EdError::InvalidCommand)?;

    if let Some(stdin) = child.stdin.as_mut() {
        for line in &lines_to_filter {
            writeln!(stdin, "{}", line).map_err(|_| EdError::InvalidCommand)?;
        }
    }

    let status = child.wait().map_err(|_| EdError::InvalidCommand)?;

    if !status.success() {
        // Clean up temp file
        let _ = std::fs::remove_file(&temp_file);
        return Err(EdError::InvalidCommand);
    }

    // TODO: Complete the filtering implementation:
    // 1. Clear undo stack (GNU ed line 540)
    // 2. Delete original lines (GNU ed delete_lines line 541)
    // 3. Read filtered output from temp file (GNU ed read_file line 543-544)
    // 4. Update current address (GNU ed line 545)
    // 5. Clean up temp file (GNU ed line 546)

    // For now, just clean up
    let _ = std::fs::remove_file(&temp_file);

    Ok(())
}

/// execute_shell_filter_with_buffer - filter lines through shell command with buffer modification
fn execute_shell_filter_with_buffer(command: &str, addresses: &AddressExtraction, buffer: &mut EdBuffer) -> Result<(), EdError> {
    use std::process::Command;
    use std::io::Write;

    // Check for redirection - not allowed in filter mode
    if command.contains('<') || command.contains('>') {
        return Err(EdError::InvalidCommand);
    }

    // Get address range for filtering
    let (first_addr, second_addr) = get_address_range(buffer, addresses)?;

    // Create temporary file for input/output (GNU ed get_tmpname logic)
    let temp_dir = std::env::temp_dir();
    let temp_file = temp_dir.join(format!("ed-{}", std::process::id()));

    // Build shell command with redirection (GNU ed main_loop.c:537-538)
    let temp_file_str = temp_file.to_str().ok_or(EdError::InvalidCommand)?;
    let shell_command_with_redirect = format!("!{} > {} 2>&1", command, temp_file_str);

    // Write addressed lines to shell command (GNU ed line 539)
    // This prints the byte count of lines being filtered
    crate::io::write_file(&shell_command_with_redirect, "w", first_addr, second_addr, buffer)
        .map_err(|_| {
            let _ = std::fs::remove_file(&temp_file);
            EdError::InvalidCommand
        })?;

    // Clear undo stack before modifying buffer (GNU ed line 540)
    buffer.clear_undo_stack();

    // Delete original lines (GNU ed delete_lines line 541-542)
    for line_num in (first_addr..=second_addr).rev() {
        buffer.delete_line(line_num)?;
    }

    // Read filtered output from temp file (GNU ed line 543-544)
    // This prints the byte count of filtered content
    // GNU ed: read_file( tmpname, current_addr() - ( current_addr() >= first_addr ), 0 )
    let insert_after = if buffer.current_line() >= first_addr {
        buffer.current_line().saturating_sub(1)
    } else {
        buffer.current_line()
    };

    crate::io::read_file(temp_file_str, insert_after, buffer).map_err(|_| {
        let _ = std::fs::remove_file(&temp_file);
        EdError::InvalidCommand
    })?;

    // Update current address (GNU ed line 544-545)
    if buffer.current_line() <= 0 && buffer.len() > 0 {
        let _ = buffer.set_current_line(1);
    }

    // Clean up temp file (GNU ed line 546)
    let _ = std::fs::remove_file(&temp_file);

    Ok(())
}

/// Temporary wrapper - will be replaced with proper exec_command
fn execute_command_wrapper(buffer: &mut EdBuffer, command_line: &str) -> Result<(), EdError> {
    // Temporary delegation to existing implementation
    crate::execute_command(buffer, command_line)
}pub fn execute_quit_command(buffer: &mut EdBuffer, forced: bool) -> Result<(), EdError> {
    // GNU ed behavior: if buffer is modified and not warned, print ? and set warned flag
    // On second quit attempt (or if forced with Q), actually quit
    if !forced && buffer.is_modified() && !buffer.warned() {
        // First quit attempt with modified buffer - warn and stay in editor
        buffer.set_warned(true);
        return Err(EdError::WarningUnsavedChanges);
    }

    // Either forced, or not modified, or already warned - quit successfully
    Err(EdError::Quit)
}

pub fn append_text_input(buffer: &mut EdBuffer, addresses: &AddressExtraction) -> Result<(), EdError> {
    // Append after the addressed line (GNU ed behavior main_loop.c:569)
    // GNU ed: append_lines( ibufpp, second_addr, false, isglobal )
    // If no address specified, append after current line
    let append_after_addr = if addresses.second_addr > 0 {
        addresses.second_addr as usize
    } else {
        buffer.current_line()
    };

    // Collect input lines until we see '.'
    let mut lines_to_append = Vec::new();
    loop {
        let mut input = String::new();
        match std::io::stdin().read_line(&mut input) {
            Ok(_) => {
                let line = input.trim_end_matches('\n');
                if line == "." {
                    break;
                }
                lines_to_append.push(line.to_string());
            },
            Err(_) => return Err(EdError::InvalidCommand),
        }
    }

    // Append all lines at once using buffer.append_lines (GNU ed buffer.c append_lines)
    if !lines_to_append.is_empty() {
        buffer.append_lines(&lines_to_append, append_after_addr)?;
    }

    Ok(())
}

pub fn insert_text_input(buffer: &mut EdBuffer, addresses: &AddressExtraction) -> Result<(), EdError> {
    // Insert before the addressed line (GNU ed behavior)
    // If no address specified, insert before current line
    let insert_addr = if addresses.first_addr > 0 {
        addresses.first_addr as usize
    } else {
        buffer.current_line()
    };

    // GNU ed: insert before the line, so we calculate the position
    // insert_pos is 0-based, insert_addr is 1-based
    let insert_pos = if insert_addr > 0 { insert_addr - 1 } else { 0 };

    let mut lines_inserted = 0;
    loop {
        let mut input = String::new();
        match std::io::stdin().read_line(&mut input) {
            Ok(_) => {
                let line = input.trim_end_matches('\n');
                if line == "." {
                    break;
                }
                // Insert at position, adjusting for previously inserted lines
                buffer.insert_line(insert_pos + lines_inserted, line.to_string())?;
                lines_inserted += 1;
            },
            Err(_) => return Err(EdError::InvalidCommand),
        }
    }

    // Set current line to the last inserted line (GNU ed behavior)
    if lines_inserted > 0 {
        buffer.set_current_line(insert_pos + lines_inserted)?;
    }

    Ok(())
}

pub fn execute_change_command(buffer: &mut EdBuffer, addresses: &AddressExtraction) -> Result<(), EdError> {
    // Change command: delete addressed lines, then insert new content
    // Following GNU ed main_loop.c:572-578: delete_lines() + append_lines()

    let (start, end) = get_address_range(buffer, addresses)?;

    // Remember the start position for insertion after deletion
    let insert_position = start;

    // Step 1: Delete the addressed lines (like GNU ed delete_lines())
    for line_num in (start..=end).rev() {
        buffer.delete_line(line_num)?;
    }

    // Step 2: Insert new content at the position where deletion occurred
    // This follows GNU ed append_lines() behavior for change command
    change_text_input(buffer, insert_position)?;

    Ok(())
}

pub fn change_text_input(buffer: &mut EdBuffer, insert_position: usize) -> Result<(), EdError> {
    // Read lines until '.' and insert them at the specified position
    let mut lines_inserted = 0;

    loop {
        let mut input = String::new();
        match std::io::stdin().read_line(&mut input) {
            Ok(_) => {
                let line = input.trim_end_matches('\n');
                if line == "." {
                    break;
                }
                // Insert at position, adjusting for previously inserted lines
                // Use saturating_sub to handle edge case where insert_position is 0
                let actual_position = insert_position.saturating_sub(1) + lines_inserted;
                buffer.insert_line(actual_position, line.to_string())?;
                lines_inserted += 1;
            },
            Err(_) => return Err(EdError::InvalidCommand),
        }
    }

    // Set current line to the last inserted line (GNU ed behavior)
    if lines_inserted > 0 {
        let new_current = insert_position.saturating_sub(1) + lines_inserted;
        buffer.set_current_line(new_current)?;
    }

    Ok(())
}

pub fn execute_number_command(buffer: &EdBuffer, addresses: &AddressExtraction) -> Result<(), EdError> {
    let (start, end) = get_address_range(buffer, addresses)?;

    for line_num in start..=end {
        if let Some(line) = buffer.get_line(line_num) {
            println!("{}\t{}", line_num, line);  // GNU ed: printf( "%d\t", current_addr() ); - no width formatting
        }
    }
    Ok(())
}

pub fn execute_line_number_command(buffer: &EdBuffer, addresses: &AddressExtraction) -> Result<(), EdError> {
    // GNU ed main_loop.c:735: printf( "%d\n", addr_cnt ? second_addr : last_addr() );
    // If address provided, use second_addr; otherwise use last_addr (total lines)
    let line_num = if addresses.second_addr >= 0 {
        addresses.second_addr as usize
    } else {
        buffer.last_addr()
    };
    println!("{}", line_num);
    Ok(())
}

pub fn undo_last_operation(buffer: &mut EdBuffer) -> Result<(), EdError> {
    buffer.undo_last_operation()
}

pub fn execute_substitute_command(buffer: &mut EdBuffer, command_args: &str, addresses: &AddressExtraction) -> Result<(), EdError> {
    // Direct implementation of substitute command with proper buffer integration
    // Follows GNU ed main_loop.c:418-491 and regex.c:406-448
    let (start, end) = get_address_range(buffer, addresses)?;

    // Parse substitute command: s/pattern/replacement/flags
    if let Some(parsed_sub) = parse_substitute_command(command_args) {
        // Compile the regex pattern with flags (GNU ed regex.c supports I flag)
        let regex_pattern = match RegexBuilder::new(&parsed_sub.pattern)
            .case_insensitive(parsed_sub.ignore_case)
            .build() {
            Ok(re) => re,
            Err(_) => return Err(EdError::InvalidCommand),
        };

        let mut last_modified_line = None;

        // Perform substitution on each line in the range
        // GNU ed regex.c:415-444 - processes each line, tracking last modified
        for line_addr in start..=end {
            if let Some(line_content) = buffer.get_line(line_addr) {
                let new_content = if parsed_sub.global {
                    // Global substitution (replace all matches)
                    regex_pattern.replace_all(line_content, parsed_sub.replacement.as_str()).to_string()
                } else if let Some(n) = parsed_sub.count {
                    // Nth occurrence substitution (GNU ed supports s/pattern/replacement/N)
                    replace_nth_occurrence(&regex_pattern, line_content, &parsed_sub.replacement, n as usize)
                } else {
                    // Single substitution (replace first match)
                    regex_pattern.replace(line_content, parsed_sub.replacement.as_str()).to_string()
                };

                // Only modify buffer if content actually changed
                if new_content != line_content {
                    // Replace the line in the buffer
                    let _ = buffer.modify_line(line_addr, new_content);
                    buffer.set_current_line(line_addr).ok();
                    last_modified_line = Some(line_addr);
                }
            }
        }

        // GNU ed main_loop.c:747 - if pflags set, print current line
        // The 'p' flag causes the last modified line to be printed
        if parsed_sub.print && last_modified_line.is_some() {
            if let Some(line_addr) = last_modified_line {
                if let Some(line_content) = buffer.get_line(line_addr) {
                    println!("{}", line_content);
                }
            }
        }

        // GNU ed regex.c:445-446 - if no match found (and not in global), return error
        if last_modified_line.is_none() {
            return Err(EdError::NoMatch);
        }

        Ok(())
    } else {
        Err(EdError::InvalidCommand)
    }
}

/// Replace the nth occurrence of a pattern in a string
/// GNU ed supports s/pattern/replacement/N where N is the occurrence number
pub fn replace_nth_occurrence(regex: &Regex, text: &str, replacement: &str, n: usize) -> String {
    let mut result = String::new();
    let mut last_match_end = 0;
    let mut occurrence_count = 0;

    for mat in regex.find_iter(text) {
        occurrence_count += 1;

        if occurrence_count == n {
            // Found the nth occurrence - replace it
            result.push_str(&text[last_match_end..mat.start()]);
            result.push_str(replacement);
            last_match_end = mat.end();
        } else {
            // Not the nth occurrence - keep original
            result.push_str(&text[last_match_end..mat.end()]);
            last_match_end = mat.end();
        }
    }

    // Append any remaining text after the last match
    result.push_str(&text[last_match_end..]);

    result
}

/// Parse substitute command arguments: s/pattern/replacement/flags
struct SubstituteArgs {
    pattern: String,
    replacement: String,
    global: bool,
    print: bool,
    ignore_case: bool,
    count: Option<i32>,
}

pub fn parse_substitute_command(args: &str) -> Option<SubstituteArgs> {
    // Basic substitute parsing: s/pattern/replacement/flags
    if args.is_empty() || !args.starts_with('/') {
        return None;
    }
    
    let parts: Vec<&str> = args[1..].splitn(3, '/').collect();
    if parts.len() < 2 {
        return None;
    }
    
    let pattern = parts[0].to_string();
    let replacement = parts[1].to_string();
    let flags = if parts.len() > 2 { parts[2] } else { "" };
    
    let mut global = false;
    let mut print = false;
    let mut ignore_case = false;
    let mut count = None;
    
    for ch in flags.chars() {
        match ch {
            'g' => global = true,
            'p' => print = true, 
            'I' => ignore_case = true,
            '1'..='9' => {
                if let Some(digit) = ch.to_digit(10) {
                    count = Some(digit as i32);
                }
            },
            _ => {} // Ignore unknown flags for now
        }
    }
    
    Some(SubstituteArgs {
        pattern,
        replacement,
        global,
        print,
        ignore_case,
        count,
    })
}


pub fn execute_write_append(_buffer: &EdBuffer) -> Result<(), EdError> {
    // TODO: Implement write-append command
    Ok(())
}

pub fn execute_write_command(buffer: &mut EdBuffer, command_args: &str, addresses: &AddressExtraction, append: bool) -> Result<(), EdError> {
    // Write command implementation following GNU ed main_loop.c:694-712
    // Two modes: 'w' (overwrite) and 'W' (append)

    let filename = command_args.trim();

    // Set default address range if none specified (GNU ed line 700-703)
    let (first_addr, second_addr) = if addresses.addr_count == 0 && buffer.len() == 0 {
        // Empty buffer case (GNU ed line 700-701)
        (0, 0)
    } else if addresses.addr_count == 0 {
        // Default to entire buffer (GNU ed line 700-703)
        (1, buffer.len())
    } else {
        get_address_range(buffer, addresses)?
    };

    // Get filename to write to
    let target_filename = if filename.is_empty() {
        // Use default filename (GNU ed behavior)
        match get_filename_from_buffer(buffer) {
            Some(fname) => fname,
            None => return Err(EdError::InvalidFilename),
        }
    } else {
        // Validate filename (GNU ed may_access_filename logic)
        // Note: Shell commands start with '!' are validated inside io::write_file
        if !filename.starts_with('!') && !may_access_filename(filename) {
            return Err(EdError::InvalidFilename);
        }
        filename
    };

    // Use io::write_file which handles shell commands (io.c:287 detects '!')
    let write_mode = if append { "a" } else { "w" };
    match crate::io::write_file(target_filename, write_mode, first_addr, second_addr, buffer) {
        Ok(_bytes) => {
            // io::write_file already prints byte count

            // If we wrote the entire buffer to the default filename, mark buffer as unmodified
            // GNU ed behavior: writing full buffer to default file clears modified flag
            // GNU ed main_loop.c:708: if( addr == last_addr() && fnp[0] != '!' ) set_modified( false );
            // For empty buffer: writing 0,0 IS writing the entire buffer!
            let wrote_entire_buffer = if buffer.len() == 0 {
                // Empty buffer: 0,0 is the entire buffer
                first_addr == 0 && second_addr == 0
            } else {
                // Non-empty buffer: 1,len is the entire buffer
                first_addr == 1 && second_addr == buffer.len()
            };

            if !append &&
               wrote_entire_buffer &&
               !target_filename.starts_with('!') &&
               (filename.is_empty() || filename == get_filename_from_buffer(buffer).unwrap_or("")) {
                buffer.set_modified(false);
            }

            Ok(())
        }
        Err(err) => Err(err)
    }
}

pub fn execute_read_command(buffer: &mut EdBuffer, command_args: &str, addresses: &AddressExtraction) -> Result<(), EdError> {
    // r command: read file content after specified address
    // From GNU ed main_loop.c:670-680

    // Get filename (GNU ed line 672: fnp = get_filename())
    let filename_arg = command_args.trim();
    let filename_to_read = if filename_arg.is_empty() {
        // Use default filename (GNU ed line 677: fnp[0] ? fnp : def_filename)
        match get_filename_from_buffer(buffer) {
            Some(f) => f.to_string(),
            None => return Err(EdError::InvalidFilename),
        }
    } else {
        filename_arg.to_string()
    };

    // If no address given, read after last line (GNU ed line 671)
    // Read command allows address 0 (insert at beginning)
    let insert_after_line = if addresses.has_no_addresses() {
        buffer.len()  // Default: after last line
    } else {
        // For read, we use second_addr directly without get_address_range validation
        // because read allows address 0 (GNU ed line 677: read_file(..., second_addr, ...))
        if addresses.second_addr < 0 {
            buffer.len()
        } else {
            addresses.second_addr as usize
        }
    };

    // Clear undo stack before read operation (GNU ed line 676)
    buffer.clear_undo_stack();

    // Read file content using io::read_file which handles shell commands
    // GNU ed io.c:677 calls read_file which detects '!' and uses popen()
    match crate::io::read_file(&filename_to_read, insert_after_line, buffer) {
        Ok(lines_read) => {
            // io::read_file already prints byte count and sets current address
            // Set current address to last inserted line (GNU ed io.c:308 logic)
            if lines_read > 0 {
                let _ = buffer.set_current_line(insert_after_line + lines_read as usize);
            }

            Ok(())
        }
        Err(err) => {
            // Don't print to stderr - GNU ed handles error display in main loop
            Err(err)
        }
    }
}

pub fn execute_edit_command(buffer: &mut EdBuffer, command_args: &str) -> Result<(), EdError> {
    // GNU ed main_loop.c:586 - 'e' command with modification check
    // if( modified() && !warned() ) return EMOD;
    if buffer.is_modified() && !buffer.warned() {
        buffer.set_warned(true);
        return Err(EdError::WarningUnsavedChanges);
    }

    // Reset warned flag since we're proceeding with edit
    buffer.set_warned(false);

    // Call the actual edit function
    edit_file(buffer, Some(command_args.trim()))
}

pub fn edit_file(buffer: &mut EdBuffer, filename: Option<&str>) -> Result<(), EdError> {
    // GNU ed main_loop.c:587-598
    // e/E command: clear buffer, reload file
    // Note: 'e' calls execute_edit_command which checks modified/warned first
    // 'E' (execute_edit_force) calls this directly, skipping the check

    // Clear buffer (GNU ed delete_lines(1, last_addr()))
    buffer.clear_buffer();
    buffer.set_modified(false);

    // Determine filename to load (GNU ed line 596)
    let file_to_load: String = if let Some(fname) = filename {
        let fname_trimmed = fname.trim();
        if !fname_trimmed.is_empty() {
            // Expand tilde in filename (GNU ed get_filename with tilde expansion)
            let expanded_filename = get_filename(fname_trimmed)
                .unwrap_or_else(|| fname_trimmed.to_string());

            // Set as new default filename ONLY if not a shell command (GNU ed line 594)
            // if( fnp[0] && fnp[0] != '!' && !set_def_filename( fnp ) )
            if !expanded_filename.starts_with('!') {
                buffer.set_filename(expanded_filename.clone());
            }

            expanded_filename
        } else {
            // Use current default filename
            match get_filename_from_buffer(buffer) {
                Some(f) => f.to_string(),
                None => return Err(EdError::InvalidFilename),
            }
        }
    } else {
        // No filename provided, use default (GNU ed: read_file(def_filename))
        match get_filename_from_buffer(buffer) {
            Some(f) => f.to_string(),
            None => return Err(EdError::InvalidFilename),
        }
    };

    // Load the file using io::read_file which handles shell commands
    // GNU ed calls read_file() which detects '!' and uses popen() (io.c:294)
    // Insert at address 0 (beginning of now-empty buffer)
    match crate::io::read_file(&file_to_load, 0, buffer) {
        Ok(_lines_read) => {
            // io::read_file already prints byte count
            // Clear modified flag after successful edit (GNU ed behavior)
            // Edit command loads a clean state - buffer should not be marked modified
            buffer.set_modified(false);
            Ok(())
        },
        Err(err) => Err(err),
    }
}

pub fn execute_edit_force(buffer: &mut EdBuffer, command_args: &str) -> Result<(), EdError> {
    // E command: same as e but doesn't check if modified (GNU ed line 587)
    edit_file(buffer, Some(command_args))
}

pub fn execute_filename_command(buffer: &mut EdBuffer, command_args: &str, addresses: &AddressExtraction) -> Result<(), EdError> {
    // Implementation following GNU ed main_loop.c:600-608

    // Line 600: Check for unexpected address (GNU ed unexpected_address(addr_cnt))
    if addresses.addr_count > 0 {
        return Err(EdError::InvalidAddress);
    }

    // Line 601: Check for unexpected command suffix - already handled by argument parsing

    // Line 602: Get filename using get_filename logic with tilde expansion
    // This matches main_loop.c:194-196 tilde expansion behavior
    if let Some(filename) = get_filename(command_args) {
        // Line 604-605: Check for invalid redirection handled in get_filename

        // Line 606: Validate and set filename
        // Validate filename using GNU ed may_access_filename logic
        if !may_access_filename(&filename) {
            return Err(EdError::InvalidFilename);
        }

        // Set the new default filename (GNU ed set_def_filename)
        buffer.set_filename(filename);
    }

    // Line 607: Print current default filename (GNU ed print_escaped(def_filename, true))
    let current_filename = match get_filename_from_buffer(buffer) {
        Some(fname) => fname,
        None => "", // Empty string if no filename set
    };

    // Print with escaping and newline (GNU ed behavior)
    crate::print_escaped(current_filename, true);
    println!(); // GNU ed: putchar('\n')

    Ok(())
}

pub fn execute_join_command(buffer: &mut EdBuffer, addresses: &AddressExtraction) -> Result<(), EdError> {
    // Join command implementation following GNU ed main_loop.c:634-639
    // From GNU ed source: "case 'j': if( !set_addr_range( current_addr(), current_addr() + 1, addr_cnt ) ||"

    // Step 1: Set address range with GNU ed defaults (GNU ed line 634)
    // Default range is current_addr, current_addr + 1 (current line and next line)
    let (first_addr, second_addr) = if addresses.addr_count == 0 {
        let current = buffer.current_line();
        if current >= buffer.len() {
            return Err(EdError::InvalidAddress);
        }
        (current, current + 1)  // Join current and next line
    } else {
        get_address_range(buffer, addresses)?
    };

    // Step 2: Validate address range
    if second_addr > buffer.len() {
        return Err(EdError::InvalidAddress);
    }

    // Step 3: Clear undo stack before operation (GNU ed line 636)
    buffer.clear_undo_stack();

    // Step 4: Only join if first_addr < second_addr (GNU ed line 637-638)
    if first_addr < second_addr {
        // Call buffer.join_lines following GNU ed join_lines() function
        buffer.join_lines(first_addr, second_addr, false)?;

        // Set current address to the joined line (GNU ed behavior)
        buffer.set_current_line(first_addr)?;
    }

    Ok(())
}

/// Parse destination address for move/copy commands (GNU ed get_third_addr logic)
pub fn parse_destination_address(addr_str: &str, buffer: &EdBuffer) -> Result<usize, EdError> {
    // Implementation following GNU ed get_third_addr() from main_loop.c:311-325
    // Uses extract_addresses to parse the destination address properly

    if addr_str.is_empty() {
        // GNU ed: traditional() && addr_cnt == 0 - "Destination expected"
        if crate::traditional() {
            return Err(EdError::InvalidAddress);
        }
        // Default to current address if no destination specified
        return Ok(buffer.current_line());
    }

    // Use existing address extraction logic to parse the destination
    // This matches GNU ed's get_third_addr which calls extract_addresses
    match extract_addresses(addr_str, buffer) {
        Ok(extraction) => {
            if extraction.second_addr < 0 {
                return Err(EdError::InvalidAddress);
            }
            let dest_addr = extraction.second_addr as usize;

            // GNU ed validation: second_addr < 0 || second_addr > last_addr()
            if dest_addr > buffer.len() {
                return Err(EdError::InvalidAddress);
            }

            Ok(dest_addr)
        },
        Err(_) => Err(EdError::InvalidAddress),
    }
}

pub fn execute_move_command(buffer: &mut EdBuffer, command_args: &str, addresses: &AddressExtraction) -> Result<(), EdError> {
    // Move command implementation following GNU ed main_loop.c:655-663

    // Step 1: Validate source address range (GNU ed set_addr_range2)
    let (first_addr, second_addr) = get_address_range(buffer, addresses)?;

    // Step 2: Parse destination address from command_args (GNU ed get_third_addr)
    let dest_addr = parse_destination_address(command_args.trim(), buffer)?;

    // Step 3: Validate destination not within source range (GNU ed line 657-658)
    if dest_addr >= first_addr && dest_addr < second_addr {
        return Err(EdError::InvalidAddress);
    }

    // Step 4: Clear undo stack before operation (GNU ed line 660)
    buffer.clear_undo_stack();

    // Step 5: Perform the move operation (GNU ed line 661)
    buffer.move_lines(first_addr, second_addr, dest_addr, false)?;

    Ok(())
}

pub fn execute_copy_command(buffer: &mut EdBuffer, command_args: &str, addresses: &AddressExtraction) -> Result<(), EdError> {
    // Transfer command implementation following GNU ed main_loop.c:684-689
    // Syntax: [addr1,addr2]t[addr3] - copy lines from addr1-addr2 to after addr3

    // Step 1: Validate source address range (GNU ed set_addr_range2)
    let (first_addr, second_addr) = get_address_range(buffer, addresses)?;

    // Step 2: Parse destination address from command_args (GNU ed get_third_addr)
    let dest_addr = parse_destination_address(command_args.trim(), buffer)?;

    // Step 3: Clear undo stack before operation (GNU ed line 687)
    buffer.clear_undo_stack();

    // Step 4: Perform the copy operation (GNU ed copy_lines)
    // Convert from 1-based addressing (GNU ed) to 0-based for buffer.copy_lines
    // Note: dest_addr in GNU ed is where to copy AFTER, buffer.copy_lines expects where to insert
    buffer.copy_lines(first_addr, second_addr, dest_addr)?;

    Ok(())
}

pub fn execute_mark_command(buffer: &mut EdBuffer, command_args: &str, addresses: &AddressExtraction) -> Result<(), EdError> {
    // Mark command implementation following GNU ed main_loop.c:640-645
    // Syntax: ka - mark current line with 'a', 2ka - mark line 2 with 'a'

    // Step 1: Get the mark character from command_args (GNU ed: n = *(*ibufpp)++)
    let mark_char = command_args.trim().chars().next().unwrap_or('\0');
    if mark_char == '\0' {
        return Err(EdError::InvalidCommand);
    }

    // Step 2: Determine the line to mark (GNU ed: second_addr logic)
    let line_to_mark = if addresses.addr_count > 0 {
        // Use specified address
        if addresses.second_addr <= 0 {
            return Err(EdError::InvalidAddress);
        }
        addresses.second_addr as usize
    } else {
        // Use current line if no address specified
        buffer.current_line()
    };

    // Step 3: Validate line address (GNU ed: if( second_addr == 0 ))
    if line_to_mark == 0 {
        return Err(EdError::InvalidAddress);
    }

    // Step 4: Mark the line (GNU ed: mark_line_node(search_line_node(second_addr), n))
    buffer.mark_line_node(line_to_mark, mark_char)?;

    Ok(())
}

pub fn execute_goto_mark_command(buffer: &mut EdBuffer, command_args: &str) -> Result<(), EdError> {
    // Goto mark command implementation - corresponds to address parsing in main_loop.c:272-276
    // Syntax: 'a - go to line marked with 'a'

    // Step 1: Get the mark character from command_args
    let mark_char = command_args.trim().chars().next().unwrap_or('\0');
    if mark_char == '\0' {
        return Err(EdError::InvalidCommand);
    }

    // Step 2: Get the marked line address (GNU ed: get_marked_node_addr)
    let marked_line = buffer.get_marked_node_addr(mark_char)?;

    // Step 3: Set current line to the marked line and print it (GNU ed behavior)
    buffer.set_current_line(marked_line)?;

    // Step 4: Print the line (same as GNU ed behavior for address navigation)
    if let Some(line) = buffer.get_line(marked_line) {
        println!("{}", line);
    }

    Ok(())
}

pub fn execute_global_command(buffer: &mut EdBuffer, command_args: &str, addresses: &AddressExtraction, match_flag: bool, interactive: bool) -> Result<(), EdError> {
    // Global command implementation following GNU ed main_loop.c:609-620
    // Supports both batch (g/v) and interactive (G/V) modes

    let (start, end) = if addresses.addr_count == 0 {
        (1, buffer.len() as i32)  // Default to all lines for global command
    } else {
        let (s, e) = get_address_range(buffer, addresses)?;
        (s as i32, e as i32)
    };

    // Step 1: Build active list using proper regex matching (GNU ed regex.c:221)
    let mut command_args_mut = command_args;
    if !crate::regex::build_active_list(&mut command_args_mut, start, end, match_flag, buffer) {
        return Err(EdError::InvalidCommand);
    }

    // For interactive mode (G/V), validate print flags and use interactive prompting
    if interactive {
        // GNU ed line 619: get_command_suffix validates print flags (p, l, n)
        let pflags = command_args_mut.trim();
        if !pflags.is_empty() && !matches!(pflags, "p" | "l" | "n" | "pn" | "pl" | "ln" | "pln") {
            return Err(EdError::InvalidCommand);
        }

        // Clear undo stack before global execution (GNU ed main_loop.c:772)
        buffer.clear_undo_stack();

        // Interactive mode: print each line and prompt for command
        // GNU ed exec_global lines 779-797
        while let Some(line_addr) = crate::global::next_active_line() {
            let _ = buffer.set_current_line(line_addr);

            // Print the line (with pflags)
            if let Some(line_content) = buffer.get_line(line_addr) {
                println!("{}", line_content);
            }

            // Read command from stdin (GNU ed line 784)
            let mut input = String::new();
            match std::io::stdin().read_line(&mut input) {
                Ok(0) => return Ok(()), // EOF - stop processing
                Ok(_) => {
                    let cmd = input.trim();
                    // If just newline, continue to next line (GNU ed line 787)
                    if cmd.is_empty() {
                        continue;
                    }
                    // Execute the command on the current line (GNU ed exec_command)
                    // Parse and execute the command
                    match crate::execute_command(buffer, cmd) {
                        Ok(_) => {}, // Command succeeded, continue
                        Err(EdError::Quit) => return Err(EdError::Quit), // Quit command
                        Err(_) => {} // Other errors - print ? and continue (GNU ed behavior)
                    }
                },
                Err(_) => return Err(EdError::InvalidCommand),
            }
        }

        return Ok(());
    }

    // Step 2: Parse remaining command after pattern (GNU ed exec_global logic)
    let command_to_execute = if command_args_mut.trim().is_empty() ||
                                command_args_mut.trim() == "\n" {
        // Default command is print (GNU ed main_loop.c:764-765)
        "p".to_string()
    } else {
        command_args_mut.trim().to_string()
    };

    // Step 3: Clear undo stack before global execution (GNU ed main_loop.c:772)
    buffer.clear_undo_stack();

    // Step 4: Execute commands on active lines (GNU ed exec_global main loop)
    if command_to_execute == "d" {
        // Special handling for delete: collect all indices and delete from highest to lowest
        // to avoid index shifting issues
        let mut indices_to_delete = Vec::new();
        while let Some(line_addr) = crate::global::next_active_line() {
            indices_to_delete.push(line_addr);
        }

        // Sort indices in descending order and delete from highest to lowest
        indices_to_delete.sort_by(|a, b| b.cmp(a));
        for line_addr in indices_to_delete {
            buffer.delete_line(line_addr)?;
        }
    } else {
        // For other commands, process normally (GNU ed exec_global:773-804)
        while let Some(line_addr) = crate::global::next_active_line() {
            let _ = buffer.set_current_line(line_addr);

            // Execute the command following GNU ed exec_global logic
            // Check if command starts with specific letter
            let cmd_char = command_to_execute.chars().next().unwrap_or(' ');

            match cmd_char {
                'p' => {
                    // Print command (GNU ed main_loop.c:765 - default behavior)
                    if let Some(line_content) = buffer.get_line(line_addr) {
                        println!("{}", line_content);
                    }
                },
                's' => {
                    // Substitute command in global context
                    // Create a single-line address extraction for this line
                    let line_addresses = AddressExtraction {
                        first_addr: line_addr as i32,
                        second_addr: line_addr as i32,
                        addr_count: 2,
                        remaining_command: String::new(),
                    };

                    // Extract the substitute command (everything after 's')
                    let sub_command = &command_to_execute[1..];

                    // Execute substitute on this single line
                    // Ignore errors (GNU ed continues on error in global context)
                    let _ = execute_substitute_command(buffer, sub_command, &line_addresses);
                },
                _ => {
                    // For now, other commands in global context return error
                    // GNU ed supports more commands but these are most common
                    return Err(EdError::InvalidCommand);
                }
            }
        }
    }

    Ok(())
}

// The parse_global_command and extract_global_command functions have been replaced
// by proper regex-based implementations in regex.rs that match GNU ed exactly


pub fn execute_backward_search(_buffer: &mut EdBuffer, _command_args: &str, _addresses: &AddressExtraction) -> Result<(), EdError> {
    Ok(())
}

pub fn execute_forward_search(_buffer: &mut EdBuffer, _command_args: &str, _addresses: &AddressExtraction) -> Result<(), EdError> {
    Ok(())
}

pub fn execute_help_command() -> Result<(), EdError> {
    // h command implementation following GNU ed main_loop.c:622-628
    // Print last error message if it exists
    let error_msg = error_msg();
    if !error_msg.is_empty() {
        println!("{}", error_msg);
    }
    Ok(())
}

pub fn execute_verbose_help_command() -> Result<(), EdError> {
    // H command implementation following GNU ed main_loop.c:622-628
    // Toggle verbose mode (GNU ed: if( c == 'H' ) verbose = !verbose;)
    set_verbose();

    // Print last error message if verbose mode is on (GNU ed line 626-627)
    // if( ( c == 'h' || verbose ) && errmsg[0] )
    if verbose() {
        let error_msg = error_msg();
        if !error_msg.is_empty() {
            println!("{}", error_msg);
        }
    }
    Ok(())
}

pub fn execute_prompt_command() -> Result<(), EdError> {
    // P command - toggle prompt flag (GNU ed main_loop.c:668)
    // GNU ed: if( c == 'P' ) { prompt_on = !prompt_on; break; }
    // NOTE: P command doesn't print anything itself
    // The "*" prompt is printed by main loop when reading next command
    crate::toggle_prompt();
    Ok(())
}

pub fn execute_scroll_command(buffer: &EdBuffer, command_args: &str, addresses: &AddressExtraction) -> Result<(), EdError> {
    // z command implementation following GNU ed main_loop.c:723-733
    // Syntax: [addr]z[n] - display n lines starting from addr (default: current+1, n=22)

    // Step 1: Set second address to current_addr() + !isglobal (GNU ed line 723)
    // For simplicity, assuming not in global mode (always add 1)
    let start_addr = if addresses.addr_count > 0 {
        addresses.second_addr as usize
    } else {
        buffer.current_line() + 1  // Default to next line if no address
    };

    // Step 2: Parse window lines if provided (GNU ed lines 725-727)
    let window_lines = if !command_args.trim().is_empty() {
        // Parse the number from command_args
        match command_args.trim().parse::<usize>() {
            Ok(n) if n > 0 => n,
            _ => 22, // Default window size (GNU ed default)
        }
    } else {
        22 // Default window size (GNU ed default)
    };

    // Step 3: Calculate end address (GNU ed line 729-730)
    // min(last_addr(), second_addr + window_lines - 1)
    let last_addr = buffer.len();
    let end_addr = std::cmp::min(last_addr, start_addr + window_lines - 1);

    // Step 4: Print lines from start_addr to end_addr (GNU ed line 729-731)
    // GNU ed uses print_lines() without numbering
    for line_num in start_addr..=end_addr {
        if let Some(line) = buffer.get_line(line_num) {
            println!("{}", line);
        }
    }

    Ok(())
}

pub fn execute_yank_command(buffer: &mut EdBuffer, addresses: &AddressExtraction) -> Result<(), EdError> {
    // y command implementation following GNU ed main_loop.c:719-722
    // Syntax: [addr1,addr2]y - yank (copy) lines to yank buffer

    // Get address range (GNU ed set_addr_range2)
    let (first_addr, second_addr) = get_address_range(buffer, addresses)?;

    // Yank the lines (GNU ed yank_lines)
    buffer.yank_lines(first_addr, second_addr)?;

    Ok(())
}

pub fn execute_put_command(buffer: &mut EdBuffer, addresses: &AddressExtraction) -> Result<(), EdError> {
    // x command implementation following GNU ed main_loop.c:713-718
    // Syntax: [addr]x - put (paste) yanked lines after addr

    // Get the address to put after (GNU ed line 713-714)
    // GNU ed: if no address given, second_addr is set to current_addr by extract_addresses
    let addr = if addresses.second_addr < 0 {
        // No address given - use current address (GNU ed default behavior)
        buffer.current_line()
    } else {
        addresses.second_addr as usize
    };


    // Validate address (GNU ed line 713-714)
    if addr > buffer.len() {
        return Err(EdError::InvalidAddress);
    }

    // Clear undo stack (GNU ed line 716)
    buffer.clear_undo_stack();

    // Put the lines (GNU ed put_lines)
    buffer.put_lines(addr)?;

    Ok(())
}
