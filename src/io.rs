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

/// I/O routines - Rust translation
/// This file matches io.c structure exactly for human review
/// C source: io.c (365 lines, 11,091 bytes) - IMMUTABLE REFERENCE

use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write, BufWriter};
use std::process::{Command, Stdio};
use std::sync::atomic::{AtomicI32, Ordering};
use std::sync::Mutex;
use crate::buffer::EdBuffer;
use crate::error::EdError;
// TODO: Import main module functions once module structure is finalized
// use crate::main::{scripted, show_strerror, show_warning};

// Static state converted to safe Rust - matches io.c functionality
static LINENUM_: AtomicI32 = AtomicI32::new(0);              // script line number
static UNTERMINATED_LINE: Mutex<Option<usize>> = Mutex::new(None);  // last line has no '\n'

/// linenum - matches io.c:30 (now memory safe)
pub fn linenum() -> i32 {
    LINENUM_.load(Ordering::Relaxed)
}

/// reset_unterminated_line - matches io.c:32 (now memory safe)
pub fn reset_unterminated_line() {
    if let Ok(mut guard) = UNTERMINATED_LINE.lock() {
        *guard = None;
    }
}

/// unmark_unterminated_line - matches io.c:34 (now memory safe)
pub fn unmark_unterminated_line(lp: usize) {
    if let Ok(mut guard) = UNTERMINATED_LINE.lock() {
        if let Some(line_addr) = *guard {
            if line_addr == lp {
                *guard = None;
            }
        }
    }
}

/// unterminated_last_line - matches io.c:37 (now memory safe)
fn unterminated_last_line(buffer: &EdBuffer) -> bool {
    UNTERMINATED_LINE.lock().map_or(false, |guard| {
        if let Some(line_addr) = *guard {
            line_addr == buffer.last_addr()
        } else {
            false
        }
    })
}

/// escchar - matches io.c:42
pub fn escchar(ch: u8) -> char {
    match ch {
        b'\x07' => 'a',  // \a (bell)
        b'\x08' => 'b',  // \b (backspace)  
        b'\x0c' => 'f',  // \f (form feed)
        b'\n' => 'n',    // \n (newline)
        b'\r' => 'r',    // \r (carriage return)
        b'\t' => 't',    // \t (tab)
        b'\x0b' => 'v',  // \v (vertical tab)
        _ => '\0',       // No escape character
    }
}

/// print_line - matches io.c:51
fn print_line(p: &str, len: usize, pflags: i32, current_addr: usize) {
    let mut col = 0;

    // Print line number if requested (pf_n flag) - exactly like GNU ed io.c:55
    if (pflags & 0x1) != 0 { // pf_n = 1
        print!("{}\t", current_addr);  // GNU ed: printf( "%d\t", current_addr() );
        col = 8;
    }
    
    for ch in p.chars() {
        if (pflags & 0x2) == 0 { // not pf_l
            print!("{}", ch);
        } else {
            // List mode - escape special characters
            match ch {
                '\t' => print!("\\t"),
                '\n' => print!("\\n"),
                '\r' => print!("\\r"),
                '\\' => print!("\\\\"),
                c if c.is_control() => print!("\\{:03o}", c as u8),
                c => print!("{}", c),
            }
            col += 1;
        }
    }
    
    // Add $ for list mode
    if (pflags & 0x2) != 0 { // pf_l
        println!("$");
    } else {
        println!();
    }
}

/// print_lines - matches io.c:87
pub fn print_lines(buffer: &EdBuffer, from: usize, to: usize, pflags: i32) -> Result<bool, EdError> {
    if from > buffer.last_addr() || to > buffer.last_addr() || from > to {
        return Err(EdError::InvalidAddress);
    }
    
    for line_num in from..=to {
        if let Some(line) = buffer.get_sbuf_line(line_num) {
            print_line(line, line.len(), pflags, line_num);
        }
    }
    
    Ok(true)
}

/// trailing_escape - matches io.c:106
fn trailing_escape(s: &str, len: usize) -> bool {
    let mut escapes = 0;
    let chars: Vec<char> = s.chars().collect();
    
    for i in (0..len.min(chars.len())).rev() {
        if chars[i] == '\\' {
            escapes += 1;
        } else {
            break;
        }
    }
    
    (escapes % 2) == 1
}

/// get_extended_line - matches io.c:119
pub fn get_extended_line(ibufpp: &str, buffer: &mut EdBuffer) -> Result<(String, usize), EdError> {
    // TODO: Implement extended line reading (for multi-line commands)
    Ok((ibufpp.to_string(), ibufpp.len()))
}

/// get_stdin_line - matches io.c:158
pub fn get_stdin_line() -> Result<(String, usize), EdError> {
    let mut input = String::new();
    match std::io::stdin().read_line(&mut input) {
        Ok(bytes_read) => {
            let line = input.trim_end_matches('\n');
            LINENUM_.fetch_add(1, Ordering::Relaxed);
            Ok((line.to_string(), bytes_read))
        },
        Err(_) => Err(EdError::InvalidCommand),
    }
}

/// read_stream_line - matches io.c:199
fn read_stream_line(filename: &str, fp: &mut BufReader<File>, buffer: &mut EdBuffer) -> Result<String, EdError> {
    let mut line = String::new();
    match fp.read_line(&mut line) {
        Ok(0) => Err(EdError::InvalidCommand), // EOF
        Ok(_) => {
            // Remove trailing newline if present
            if line.ends_with('\n') {
                line.pop();
                // GNU ed io.c:213-214: remove CR only from CR/LF pairs
                if line.ends_with('\r') {
                    line.pop();
                }
            } else {
                // Mark as unterminated line
                if let Ok(mut guard) = UNTERMINATED_LINE.lock() {
                    *guard = Some(buffer.last_addr() + 1);
                }
            }
            Ok(line)
        },
        Err(_) => {
            // TODO: show_strerror(Some(filename), 1);
            Err(EdError::InvalidCommand)
        }
    }
}

/// read_stream - matches io.c:240  
fn read_stream(filename: &str, fp: &mut BufReader<File>, addr: usize, buffer: &mut EdBuffer) -> Result<i64, EdError> {
    let mut total_size = 0i64;
    let mut current_addr = addr;
    
    loop {
        match read_stream_line(filename, fp, buffer) {
            Ok(line) => {
                total_size += line.len() as i64 + 1; // +1 for newline
                
                // Add line to buffer at current position
                buffer.insert_line(current_addr, line)?;
                current_addr += 1;
            },
            Err(_) => break, // EOF or error
        }
    }
    
    // Update buffer state
    if buffer.len() > 0 && !buffer.isbinary() {
        // Safe update of unterminated line state (converted from unsafe)
        if let Ok(mut guard) = UNTERMINATED_LINE.lock() {
            if guard.is_some() {
                *guard = Some(buffer.last_addr());
            }
        }
    }
    
    Ok(total_size)
}

/// read_file - matches io.c:288 (MAIN READ FUNCTION)
pub fn read_file(filename: &str, addr: usize, buffer: &mut EdBuffer) -> Result<i32, EdError> {
    // Handle shell command input
    if filename.starts_with('!') {
        return read_shell_command(&filename[1..], addr, buffer);
    }
    
    // Try to open file
    let file = match File::open(filename) {
        Ok(f) => f,
        Err(e) => {
            // Print error to stderr (GNU ed io.c show_strerror behavior)
            // Format to match GNU ed output (just "filename: error_description")
            use std::io::ErrorKind;
            let error_msg = match e.kind() {
                ErrorKind::NotFound => "No such file or directory",
                ErrorKind::PermissionDenied => "Permission denied",
                _ => "I/O error",
            };
            eprintln!("{}: {}", filename, error_msg);
            return Err(EdError::InvalidAddress);
        }
    };
    
    let mut reader = BufReader::new(file);
    
    // Read file into buffer
    let size = read_stream(filename, &mut reader, addr, buffer)?;
    
    // Print file size if not in script mode
    // TODO: Check scripted mode - if !scripted()
    println!("{}", size);
    
    // Return line count
    Ok((buffer.current_addr() - addr) as i32)
}

/// Helper function for shell command input
fn read_shell_command(command: &str, addr: usize, buffer: &mut EdBuffer) -> Result<i32, EdError> {
    let output = Command::new("/bin/sh")
        .arg("-c")
        .arg(command)
        .output()
        .map_err(|_| EdError::InvalidCommand)?;
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut current_addr = addr;
    
    for line in stdout.lines() {
        buffer.insert_line(current_addr, line.to_string())?;
        current_addr += 1;
    }
    
    // TODO: Check scripted mode - if !scripted()
    println!("{}", stdout.len());
    
    Ok((current_addr - addr) as i32)
}

/// write_stream - matches io.c:315
fn write_stream(filename: &str, fp: &mut BufWriter<File>, from: usize, to: usize, buffer: &EdBuffer) -> Result<i64, EdError> {
    let mut total_size = 0i64;
    
    for line_num in from..=to {
        if let Some(line) = buffer.get_sbuf_line(line_num) {
            // Write line content
            fp.write_all(line.as_bytes()).map_err(|_| EdError::InvalidCommand)?;
            total_size += line.len() as i64;
            
            // Add newline unless it's the last line and binary and unterminated
            if line_num != buffer.last_addr() || !buffer.isbinary() || !unterminated_last_line(buffer) {
                fp.write_all(b"\n").map_err(|_| EdError::InvalidCommand)?;
                total_size += 1;
            }
        }
    }
    
    fp.flush().map_err(|_| EdError::InvalidCommand)?;
    Ok(total_size)
}

/// write_file - matches io.c:346 (MAIN WRITE FUNCTION)
pub fn write_file(filename: &str, mode: &str, from: usize, to: usize, buffer: &EdBuffer) -> Result<i32, EdError> {
    // Handle shell command output
    if filename.starts_with('!') {
        return write_shell_command(&filename[1..], from, to, buffer);
    }
    
    // Open file with specified mode
    let file = if mode == "w" {
        OpenOptions::new().write(true).truncate(true).create(true).open(filename)
    } else if mode == "a" {
        OpenOptions::new().write(true).append(true).create(true).open(filename)
    } else {
        OpenOptions::new().read(true).write(true).open(filename)
    };
    
    let file = match file {
        Ok(f) => f,
        Err(e) => {
            // TODO: show_strerror(Some(filename), e.raw_os_error().unwrap_or(1));
            return Err(EdError::InvalidCommand);
        }
    };
    
    let mut writer = BufWriter::new(file);
    
    // Write lines to file
    let size = write_stream(filename, &mut writer, from, to, buffer)?;
    
    // Print bytes written if not in script mode
    // TODO: Check scripted mode - if !scripted()
    println!("{}", size);
    
    // Return line count
    Ok(if from > 0 && from <= to { (to - from + 1) as i32 } else { 0 })
}

/// Helper function for shell command output
fn write_shell_command(command: &str, from: usize, to: usize, buffer: &EdBuffer) -> Result<i32, EdError> {
    let mut child = Command::new("/bin/sh")
        .arg("-c")
        .arg(command)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .map_err(|_| EdError::InvalidCommand)?;

    let mut bytes_written = 0;
    if let Some(stdin) = child.stdin.as_mut() {
        for line_num in from..=to {
            if let Some(line) = buffer.get_sbuf_line(line_num) {
                stdin.write_all(line.as_bytes()).map_err(|_| EdError::InvalidCommand)?;
                bytes_written += line.len();
                stdin.write_all(b"\n").map_err(|_| EdError::InvalidCommand)?;
                bytes_written += 1;
            }
        }
    }

    let output = child.wait_with_output().map_err(|_| EdError::InvalidCommand)?;

    // Print the output from the shell command (GNU ed behavior for !cat example)
    print!("{}", String::from_utf8_lossy(&output.stdout));

    // Print byte count (GNU ed io.c:361)
    // TODO: Check scripted mode - if !scripted()
    println!("{}", bytes_written);

    Ok(if from > 0 && from <= to { (to - from + 1) as i32 } else { 0 })
}

// Additional utility functions for I/O operations

/// Check if character needs escaping in list mode
pub fn needs_escape(ch: char) -> bool {
    ch.is_control() || ch == '$' || ch == '\\'
}

/// Format line for list mode output
pub fn format_list_line(line: &str) -> String {
    let mut result = String::new();
    for ch in line.chars() {
        match ch {
            '\t' => result.push_str("\\t"),
            '\n' => result.push_str("\\n"),
            '\r' => result.push_str("\\r"),
            '\\' => result.push_str("\\\\"),
            '$' => result.push_str("\\$"),
            c if c.is_control() => result.push_str(&format!("\\{:03o}", c as u8)),
            c => result.push(c),
        }
    }
    result.push('$');
    result
}

/// current_addr - moved from buffer.rs to match C source usage in io.c
pub fn current_addr(buffer: &EdBuffer) -> usize {
    // TODO: Access buffer's current address - need to expose field or use method
    // buffer.current_addr_
    1 // Placeholder until buffer field access is resolved
}

/// format_output_gnu_style - moved from compat.rs (output formatting belongs in I/O module)
pub fn format_output_gnu_style(content: &str) -> String {
    // TODO: Apply any GNU ed specific output formatting
    content.to_string()
}

/// write_file_simple - Convenience wrapper moved from main.rs (originally io.c:346)
/// This wrapper provides a simpler interface for writing entire buffer to file
pub fn write_file_simple(buffer: &mut EdBuffer, filename: Option<&str>) -> Result<(), EdError> {
    
    let fname = if let Some(f) = filename {
        f
    } else {
        crate::main_loop::get_filename_from_buffer(buffer).ok_or(EdError::InvalidCommand)?
    };
    
    let mut content = String::new();
    for line_num in 1..=buffer.len() {
        if let Some(line) = buffer.get_line(line_num) {
            content.push_str(line);
            content.push('\n');
        }
    }
    
    match std::fs::write(fname, &content) {
        Ok(()) => {
            buffer.clear_modified_flag();
            // TODO: Check scripted mode before printing
            println!("{}", content.len());
            Ok(())
        },
        Err(_) => Err(EdError::InvalidCommand),
    }
}

/// Get file size for reporting
pub fn get_file_size(filename: &str) -> Result<u64, EdError> {
    match std::fs::metadata(filename) {
        Ok(metadata) => Ok(metadata.len()),
        Err(_) => Err(EdError::InvalidCommand),
    }
}

/// Check if file exists and is readable
pub fn file_exists(filename: &str) -> bool {
    std::path::Path::new(filename).exists()
}

/// Check if file is writable
pub fn file_writable(filename: &str) -> bool {
    if let Ok(file) = OpenOptions::new().write(true).append(true).open(filename) {
        drop(file);
        true
    } else {
        false
    }
}