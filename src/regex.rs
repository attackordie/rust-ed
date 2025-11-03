/// Regular expression interface routines - Rust translation
/// This file matches regex.c structure exactly for human review
/// C source: regex.c (449 lines, 14,796 bytes) - IMMUTABLE REFERENCE

use regex::Regex;
use crate::error::EdError;
use std::sync::Mutex;
// Note: Many functions are placeholders until full buffer integration is complete
// These imports will be updated as functions are properly implemented in their respective modules

// Error message constants - matches regex.c:26-30
static INV_I_SUF: &str = "Suffix 'I' not allowed on empty regexp";
static INV_PAT_DEL: &str = "Invalid pattern delimiter";
static MIS_PAT_DEL: &str = "Missing pattern delimiter";
static NO_MATCH: &str = "No match";
static NO_PREV_PAT: &str = "No previous pattern";
static NO_PREV_SUBST: &str = "No previous substitution";

// Static state converted to safe Rust - matches regex.c:31-36 functionality
static LAST_REGEXP: Mutex<Option<Regex>> = Mutex::new(None);     // last regex found
static SUBST_REGEXP: Mutex<Option<Regex>> = Mutex::new(None);    // regex of last substitution

// Replacement buffer state - safe Rust equivalents
static RBUF: Mutex<Vec<u8>> = Mutex::new(Vec::new());            // replacement buffer
static RLEN: Mutex<usize> = Mutex::new(0);                      // replacement length

/// subst_regex - matches regex.c:39 (now memory safe)
pub fn subst_regex() -> bool {
    SUBST_REGEXP.lock().map_or(false, |guard| guard.is_some())
}

/// translit_text - matches regex.c:43
fn translit_text(p: &mut [u8], from: u8, to: u8) {
    for byte in p.iter_mut() {
        if *byte == from {
            *byte = to;
        }
    }
}

/// newline_to_nul - matches regex.c:54
fn newline_to_nul(s: &mut [u8]) {
    translit_text(s, b'\n', b'\0');
}

/// nul_to_newline - matches regex.c:58
fn nul_to_newline(s: &mut [u8]) {
    translit_text(s, b'\0', b'\n');
}

/// islf_or_nul - matches regex.c:62
pub fn islf_or_nul(ch: u8) -> bool {
    ch == b'\n' || ch == 0
}

/// parse_char_class - matches regex.c:65
fn parse_char_class(p: &str) -> Option<usize> {
    let bytes = p.as_bytes();
    let mut i = 0;
    
    if i < bytes.len() && bytes[i] == b'^' {
        i += 1;
    }
    if i < bytes.len() && bytes[i] == b']' {
        i += 1;
    }
    
    while i < bytes.len() && bytes[i] != b']' && !islf_or_nul(bytes[i]) {
        if i + 1 < bytes.len() && bytes[i] == b'[' {
            let d = bytes[i + 1];
            if d == b'.' || d == b':' || d == b'=' {
                i += 1;
                let mut c = bytes[i + 1];
                i += 1;
                while i < bytes.len() && (bytes[i] != b']' || c != d) {
                    c = bytes[i];
                    if islf_or_nul(c) {
                        return None;
                    }
                    i += 1;
                }
            }
        }
        i += 1;
    }
    
    if i < bytes.len() && bytes[i] == b']' {
        Some(i)
    } else {
        None
    }
}

/// extract_pattern - matches regex.c:84
fn extract_pattern(ibufpp: &mut &str, delimiter: char) -> Option<String> {
    let input = *ibufpp;
    let bytes = input.as_bytes();
    let mut i = 0;
    
    while i < bytes.len() && bytes[i] != delimiter as u8 && !islf_or_nul(bytes[i]) {
        if bytes[i] == b'[' {
            if let Some(class_end) = parse_char_class(&input[i + 1..]) {
                i += class_end + 2; // +1 for '[' and +1 for the relative position
            } else {
                // TODO: set_error_msg("Unbalanced brackets ([])");
                return None;
            }
        } else if bytes[i] == b'\\' {
            i += 1;
            if i >= bytes.len() || islf_or_nul(bytes[i]) {
                // TODO: set_error_msg("Trailing backslash (\\)");
                return None;
            }
        }
        i += 1;
    }
    
    let pattern = String::from_utf8_lossy(&bytes[..i]).to_string();
    *ibufpp = &input[i..];
    
    // Handle binary mode newline conversion
    // TODO: Handle binary mode newline conversion  
    // if isbinary() { nul_to_newline conversion }
    Some(pattern)
}

/// compile_regex - matches regex.c:115
fn compile_regex(pat: &str, ignore_case: bool) -> Option<Regex> {
    // Build regex flags
    let mut builder = regex::RegexBuilder::new(pat);
    builder.case_insensitive(ignore_case);
    
    // Add extended regex support if enabled
    // TODO: Check extended_regexp() from main module
    // Extended regexes are default in Rust regex crate
    builder.multi_line(false); // GNU ed regexes are single-line by default
    
    match builder.build() {
        Ok(exp) => {
            // Free last_regexp if different from subst_regexp (now memory safe)
            if let Ok(mut last_guard) = LAST_REGEXP.lock() {
                // In Rust, RAII automatically handles cleanup - no manual regfree needed
                *last_guard = Some(exp.clone());
            }
            Some(exp)
        }
        Err(err) => {
            // TODO: set_error_msg(&format!("Regex error: {}", err));
            None
        }
    }
}

/// get_compiled_regex - matches regex.c:142
fn get_compiled_regex(ibufpp: &mut &str) -> Option<Regex> {
    let input = *ibufpp;
    let bytes = input.as_bytes();
    
    if bytes.is_empty() {
        // TODO: set_error_msg(INV_PAT_DEL);
        return None;
    }
    
    let delimiter = bytes[0] as char;
    
    if delimiter == ' ' || islf_or_nul(delimiter as u8) {
        // TODO: set_error_msg(INV_PAT_DEL);
        return None;
    }
    
    *ibufpp = &input[1..]; // Move past delimiter
    
    // Check for empty RE
    if ibufpp.is_empty() || ibufpp.chars().next().unwrap() == delimiter || islf_or_nul(ibufpp.as_bytes()[0]) {
        // Safe access to last regex (converted from unsafe block)
        match LAST_REGEXP.lock() {
            Ok(guard) => {
                if guard.is_none() {
                    // TODO: set_error_msg(NO_PREV_PAT);
                    return None;
                }
                
                // Handle delimiter removal and 'I' suffix check
                if !ibufpp.is_empty() && ibufpp.chars().next().unwrap() == delimiter {
                    *ibufpp = &ibufpp[1..]; // Remove delimiter
                    if !ibufpp.is_empty() && ibufpp.chars().next().unwrap() == 'I' {
                        // TODO: set_error_msg(INV_I_SUF);
                        return None;
                    }
                }
                guard.clone()
            }
            Err(_) => None
        }
    } else {
        // Extract pattern
        if let Some(pat) = extract_pattern(ibufpp, delimiter) {
            let mut ignore_case = false;
            
            // Check for delimiter and 'I' suffix
            if !ibufpp.is_empty() && ibufpp.chars().next().unwrap() == delimiter {
                *ibufpp = &ibufpp[1..]; // Remove delimiter
                if !ibufpp.is_empty() && ibufpp.chars().next().unwrap() == 'I' {
                    ignore_case = true;
                    *ibufpp = &ibufpp[1..]; // Remove suffix
                }
            }
            
            compile_regex(&pat, ignore_case)
        } else {
            None
        }
    }
}

/// get_pattern_for_s - matches regex.c:170
pub fn get_pattern_for_s(ibufpp: &mut &str) -> Option<String> {
    let input = *ibufpp;
    let bytes = input.as_bytes();
    
    if bytes.is_empty() {
        // TODO: set_error_msg(INV_PAT_DEL);
        return None;
    }
    
    let delimiter = bytes[0] as char;
    
    if delimiter == ' ' || delimiter == '\n' {
        // TODO: set_error_msg(INV_PAT_DEL);
        return None;
    }
    
    *ibufpp = &input[1..]; // Move past delimiter
    
    // Check for empty RE
    if !ibufpp.is_empty() && ibufpp.chars().next().unwrap() == delimiter {
        // Safe check for last regex (converted from unsafe)
        if LAST_REGEXP.lock().map_or(true, |guard| guard.is_none()) {
            // TODO: set_error_msg(NO_PREV_PAT);
            return None;
        }
        return Some(String::new()); // Empty pattern
    }
    
    let pat = extract_pattern(ibufpp, delimiter)?;
    
    if ibufpp.is_empty() || ibufpp.chars().next().unwrap() != delimiter {
        // TODO: set_error_msg(MIS_PAT_DEL);
        return None;
    }
    
    Some(pat)
}

/// set_subst_regex - matches regex.c:188
pub fn set_subst_regex(pat: Option<&str>, ignore_case: bool) -> bool {
    if let Some(pattern) = pat {
        if pattern.is_empty() && ignore_case {
            // TODO: set_error_msg(INV_I_SUF);
            return false;
        }
        
        crate::signal::disable_interrupts();
        let exp = if pattern.is_empty() {
            LAST_REGEXP.lock().ok().and_then(|guard| guard.clone())
        } else {
            compile_regex(pattern, ignore_case)
        };
        
        if let Some(regex) = exp {
            // Safe assignment (converted from unsafe)
            if let Ok(mut guard) = SUBST_REGEXP.lock() {
                *guard = Some(regex);
            }
            crate::signal::enable_interrupts();
            true
        } else {
            crate::signal::enable_interrupts();
            false
        }
    } else {
        false
    }
}

/// replace_subst_re_by_search_re - matches regex.c:206
pub fn replace_subst_re_by_search_re() -> bool {
    // Safe implementation (converted from unsafe)
    if LAST_REGEXP.lock().map_or(true, |guard| guard.is_none()) {
        // TODO: set_error_msg(NO_PREV_PAT);
        return false;
    }
    
    crate::signal::disable_interrupts();
    if let (Ok(last_guard), Ok(mut subst_guard)) = (LAST_REGEXP.lock(), SUBST_REGEXP.lock()) {
        *subst_guard = last_guard.clone();
    }
    crate::signal::enable_interrupts();
    true
}

/// build_active_list - matches regex.c:221 (GNU ed main_loop.c:616)
pub fn build_active_list(ibufpp: &mut &str, first_addr: i32, second_addr: i32, match_flag: bool, buffer: &crate::buffer::EdBuffer) -> bool {
    let exp = match get_compiled_regex(ibufpp) {
        Some(regex) => regex,
        None => return false,
    };

    crate::global::clear_active_list();

    // Iterate through lines in the address range and find matches
    // Following GNU ed regex.c:224-238 logic
    for addr in first_addr..=second_addr {
        // addr is already 1-based, get_line expects 1-based input
        if let Some(line_content) = buffer.get_line(addr as usize) {
            // Check if line matches the pattern
            let matches = exp.is_match(&line_content);

            // Add to active list if it matches our criteria
            // match_flag == true for 'g' command (include matches)
            // match_flag == false for 'v' command (include non-matches)
            if matches == match_flag {
                // Store 1-based address (addr) to match GNU ed convention
                if !crate::global::set_active_line(addr as usize) {
                    return false;
                }
            }
        }
    }

    true
}

/// next_matching_node_addr_with_buffer - matches regex.c:244
/// Returns the address of the next line matching a regular expression in a given direction.
/// Wraps around begin/end of editor buffer if necessary.
pub fn next_matching_node_addr_with_buffer(ibufpp: &mut &str, buffer: &crate::buffer::EdBuffer) -> Result<usize, EdError> {
    // Determine search direction based on delimiter (GNU ed regex.c:246)
    let forward = !ibufpp.is_empty() && ibufpp.chars().next().unwrap() == '/';

    // Get compiled regex (GNU ed regex.c:247)
    let exp = match get_compiled_regex(ibufpp) {
        Some(regex) => regex,
        None => return Err(EdError::InvalidCommand),
    };

    // Store the compiled regex for future use (GNU ed regex.c:135)
    if let Ok(mut last_guard) = LAST_REGEXP.lock() {
        *last_guard = Some(exp.clone());
    }

    // Get current address (GNU ed regex.c:248)
    let mut addr = buffer.current_line();
    let start_addr = addr;

    // Search with wrap-around (GNU ed regex.c:251-262)
    loop {
        // Move to next/previous address with wrap-around
        addr = if forward {
            if addr >= buffer.len() { 1 } else { addr + 1 }
        } else {
            if addr <= 1 { buffer.len() } else { addr - 1 }
        };

        // Check if we wrapped around to start
        if addr == start_addr {
            break;
        }

        // Skip if addr is 0 (shouldn't happen with proper wrap-around)
        if addr == 0 {
            continue;
        }

        // Get line content and test against regex (GNU ed regex.c:255-259)
        if let Some(line_content) = buffer.get_line(addr) { // addr is 1-based, get_line expects 1-based
            // Handle binary mode newline conversion if needed
            // TODO: Implement binary mode handling like GNU ed

            // Test regex match (GNU ed regex.c:259)
            if exp.is_match(&line_content) {
                return Ok(addr);
            }
        }
    }

    // No match found (GNU ed regex.c:263-264)
    Err(EdError::PatternNotFound)
}

/// next_matching_node_addr - matches regex.c:244 (compatibility wrapper)
/// This is kept for compatibility but delegates to the buffer-aware version
pub fn next_matching_node_addr(ibufpp: &mut &str) -> Result<usize, EdError> {
    // This is a placeholder that should not be called directly
    // The real implementation is next_matching_node_addr_with_buffer
    Err(EdError::PatternNotFound)
}

/// extract_replacement - matches regex.c:270
pub fn extract_replacement(ibufpp: &mut &str, isglobal: bool) -> bool {
    let input = *ibufpp;
    let bytes = input.as_bytes();
    
    if bytes.is_empty() {
        // TODO: set_error_msg(MIS_PAT_DEL);
        return false;
    }
    
    let delimiter = bytes[0];
    *ibufpp = &input[1..]; // Move past delimiter
    
    // Check for single '%' replacement
    if !ibufpp.is_empty() && ibufpp.as_bytes()[0] == b'%' {
        let next_chars = &ibufpp.as_bytes()[1..];
        if !next_chars.is_empty() && 
           (next_chars[0] == delimiter || 
            (next_chars[0] == b'\n' && (!isglobal || next_chars.len() == 1))) {
            *ibufpp = &ibufpp[1..]; // Move past '%'
            // Safe check for replacement buffer (converted from unsafe)
            if RBUF.lock().map_or(true, |guard| guard.is_empty()) {
                // TODO: set_error_msg(NO_PREV_SUBST);
                return false;
            }
            return true;
        }
    }
    
    // Extract replacement text
    let mut replacement = Vec::new();
    let mut i = 0;
    let input_bytes = ibufpp.as_bytes();
    
    while i < input_bytes.len() && input_bytes[i] != delimiter {
        if input_bytes[i] == b'\n' && (!isglobal || i + 1 >= input_bytes.len()) {
            break;
        }
        
        replacement.push(input_bytes[i]);
        i += 1;
        
        // Handle escaped newlines (non-global mode)
        if i >= 2 && replacement[replacement.len()-2] == b'\\' && 
           replacement[replacement.len()-1] == b'\n' && !isglobal {
            // Get next line from stdin
            // This is a placeholder for stdin line reading
            // TODO: Implement stdin line reading for multi-line replacements
            break;
        }
    }
    
    crate::signal::disable_interrupts();
    // Safe assignment (converted from unsafe)
    if let (Ok(mut rbuf_guard), Ok(mut rlen_guard)) = (RBUF.lock(), RLEN.lock()) {
        *rbuf_guard = replacement;
        *rlen_guard = rbuf_guard.len();
    }
    crate::signal::enable_interrupts();
    
    *ibufpp = &ibufpp[i..]; // Update position
    true
}

/// replace_matched_text - matches regex.c:314
fn replace_matched_text(txtbuf: &mut Vec<u8>, txt: &[u8], captures: &regex::Captures, re_nsub: usize) -> Result<(), EdError> {
    // Safe iteration over replacement buffer (converted from unsafe)
    if let Ok(rbuf_guard) = RBUF.lock() {
        for &byte in rbuf_guard.iter() {
            if byte == b'&' {
                // Replace with full match
                if let Some(full_match) = captures.get(0) {
                    txtbuf.extend_from_slice(&txt[full_match.start()..full_match.end()]);
                }
            } else if byte == b'\\' && !rbuf_guard.is_empty() {
                // Handle backreferences \1-\9
                // This is a simplified version - full implementation would handle all escapes
                txtbuf.push(byte);
            } else {
                txtbuf.push(byte);
            }
        }
    }
    Ok(())
}

/// line_replace - matches regex.c:351
fn line_replace(txtbuf: &mut Vec<u8>, line_text: &str, snum: i32) -> Result<i32, EdError> {
    // Safe access to substitution regex (converted from unsafe)
    if let Ok(subst_guard) = SUBST_REGEXP.lock() {
        if let Some(ref subst_regex) = *subst_guard {
            let text = line_text.to_string();
            let global = snum <= 0;
            let mut changed = false;
            let mut match_count = 0;
            
            // Handle binary mode
            // TODO: Check isbinary() and implement nul_to_newline conversion
            
            // Find matches and replace
            let result = if global {
                subst_regex.replace_all(&text, |caps: &regex::Captures| {
                    changed = true;
                    // Placeholder for replacement logic
                    "REPLACED".to_string()
                })
            } else {
                subst_regex.replace(&text, |caps: &regex::Captures| {
                    match_count += 1;
                    if snum <= 0 || match_count == snum {
                        changed = true;
                        "REPLACED".to_string()
                    } else {
                        caps.get(0).unwrap().as_str().to_string()
                    }
                })
            };
            
            if changed {
                txtbuf.clear();
                txtbuf.extend_from_slice(result.as_bytes());
                txtbuf.push(b'\n');
                Ok(txtbuf.len() as i32)
            } else {
                Ok(0) // No change
            }
        } else {
            Ok(0) // No substitution regex available
        }
    } else {
        Ok(0) // Failed to lock mutex
    }
}

/// search_and_replace - matches regex.c:406
pub fn search_and_replace(first_addr: i32, second_addr: i32, snum: i32, isglobal: bool) -> bool {
    let mut txtbuf = Vec::new();
    let mut match_found = false;
    
    for addr in first_addr..=second_addr {
        // Get line content from buffer
        // This is a placeholder for the line replacement logic
        // TODO: Implement full line replacement with undo support
        
        match line_replace(&mut txtbuf, "placeholder_line", snum) {
            Ok(size) => {
                if size > 0 {
                    match_found = true;
                    // Replace line in buffer
                    // TODO: Implement line replacement in buffer with undo
                }
            }
            Err(_) => return false,
        }
    }
    
    if !match_found && !isglobal {
        // TODO: set_error_msg(NO_MATCH);
        return false;
    }
    
    true
}