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

/// Line buffer management - Rust translation
/// This file matches buffer.c structure exactly for human review
/// C source: buffer.c (18,256 bytes) - IMMUTABLE REFERENCE

use std::collections::VecDeque;
use crate::error::EdError;

/// Safety limits to prevent resource exhaustion (matching GNU ed behavior)
pub struct SafetyLimits {
    pub max_file_size: usize,        // Default: 1GB
    pub max_line_length: usize,      // Default: 1MB  
    pub max_buffer_lines: usize,     // Default: 10M lines
    pub max_recursion_depth: usize,  // Default: 1000
}

impl Default for SafetyLimits {
    fn default() -> Self {
        Self {
            max_file_size: 1024 * 1024 * 1024,      // 1GB
            max_line_length: 1024 * 1024,           // 1MB
            max_buffer_lines: 10_000_000,           // 10M lines
            max_recursion_depth: 1000,              // 1000 levels
        }
    }
}

/// Undo operation record (matching GNU ed undo_atom structure)
#[derive(Debug, Clone)]
enum UndoOperation {
    AddLine { position: usize, line: String },
    DeleteLine { position: usize, line: String },
    ModifyLine { position: usize, old_line: String, new_line: String },
}

/// Line buffer with GNU ed semantics and Rust memory safety
pub struct EdBuffer {
    lines: VecDeque<String>,
    current_addr_: usize,           // matches C current_addr_ exactly
    last_addr_: usize,              // matches C last_addr_ exactly
    modified_: u8,                  // matches C modified_ (bitfield for warned)
    isbinary_: bool,               // matches C isbinary_ exactly
    pub filename: Option<String>,
    limits: SafetyLimits,
    undo_stack: Vec<UndoOperation>,
    yank_buffer: Vec<String>,      // GNU ed yank buffer
    marks: [Option<usize>; 26],    // line markers - matches C mark[26] exactly
    markno: usize,                 // line marker count - matches C markno exactly
    // GNU ed undo state management (matches buffer.c:533-535)
    u_current_addr: i32,           // matches C u_current_addr (-1 if undo disabled)
    u_last_addr: i32,              // matches C u_last_addr (-1 if undo disabled)
    u_modified: bool,              // matches C u_modified
}

impl EdBuffer {
    /// init_buffers - matches buffer.c:292
    pub fn new() -> Self {
        Self {
            lines: VecDeque::new(),
            current_addr_: 0,
            last_addr_: 0,
            modified_: 0,
            isbinary_: false,
            filename: None,
            limits: SafetyLimits::default(),
            undo_stack: Vec::new(),
            yank_buffer: Vec::new(),
            marks: [None; 26],
            markno: 0,
            // Initialize undo state - matches buffer.c:564-565
            u_current_addr: -1,  // disabled initially
            u_last_addr: -1,     // disabled initially
            u_modified: false,
        }
    }
    
    /// current_addr - matches buffer.c:42  
    // NOTE: This should be moved to io.rs to match C source structure
    // Keeping temporary access method until migration is complete
    pub fn current_addr(&self) -> usize {
        self.current_addr_
    }
    
    /// inc_current_addr - matches buffer.c:43
    pub fn inc_current_addr(&mut self) -> usize {
        if self.current_addr_ < self.last_addr_ {
            self.current_addr_ += 1;
        }
        self.current_addr_
    }
    
    /// set_current_addr - matches buffer.c:46  
    pub fn set_current_addr(&mut self, addr: usize) {
        self.current_addr_ = addr;
    }
    
    /// last_addr - matches buffer.c:48
    pub fn last_addr(&self) -> usize {
        self.last_addr_
    }
    
    /// isbinary - matches buffer.c:50
    pub fn isbinary(&self) -> bool {
        self.isbinary_
    }
    
    /// set_binary - matches buffer.c:51
    pub fn set_binary(&mut self) {
        self.isbinary_ = true;
    }
    
    /// modified - matches buffer.c:53 (ignore warned bit)
    pub fn modified(&self) -> bool {
        (self.modified_ & 1) != 0
    }
    
    /// set_modified - matches buffer.c:54 (clear warned bit)
    pub fn set_modified(&mut self, b: bool) {
        self.modified_ = if b { 1 } else { 0 };
    }
    
    /// warned - matches buffer.c:55
    pub fn warned(&self) -> bool {
        self.modified_ == 3
    }
    
    /// set_warned - matches buffer.c:56
    pub fn set_warned(&mut self, b: bool) {
        if b {
            self.modified_ |= 2;
        } else {
            self.modified_ &= 1;
        }
    }
    
    /// inc_addr - matches buffer.c:60
    pub fn inc_addr(&self, addr: i32) -> i32 {
        let new_addr = addr + 1;
        if new_addr > self.last_addr_ as i32 {
            self.last_addr_ as i32
        } else {
            new_addr
        }
    }
    
    /// dec_addr - matches buffer.c:63
    pub fn dec_addr(&self, addr: i32) -> i32 {
        if addr <= 1 {
            1
        } else {
            addr - 1
        }
    }
    
    // Note: link_nodes, insert_node, add_line_node abstracted by VecDeque
    
    /// too_many_lines - matches buffer.c:81
    fn too_many_lines(&self) -> bool {
        self.lines.len() >= self.limits.max_buffer_lines
    }
    
    // Note: dup_line_node abstracted by String::clone()
    
    /// append_lines - matches buffer.c:116
    pub fn append_lines(&mut self, lines_to_add: &[String], addr: usize) -> Result<bool, EdError> {
        if self.too_many_lines() {
            return Err(EdError::InvalidCommand);
        }

        // Insert at specified address and create undo records (GNU ed logic)
        let mut insert_pos = addr;
        for line in lines_to_add {
            self.lines.insert(insert_pos, line.clone());

            // Create undo record for each added line (matches GNU ed push_undo_atom)
            self.undo_stack.push(UndoOperation::AddLine {
                position: insert_pos,
                line: line.clone(),
            });

            insert_pos += 1;
        }

        self.last_addr_ = self.lines.len();
        self.current_addr_ = insert_pos.saturating_sub(1);
        self.modified_ = 1;
        Ok(true)
    }
    
    /// clear_yank_buffer - matches buffer.c:154
    fn clear_yank_buffer(&mut self) {
        self.yank_buffer.clear();
    }
    
    /// close_sbuf - matches buffer.c:171 (string buffer operations)
    pub fn close_sbuf(&self) -> bool {
        // TODO: Implement string buffer closing
        true
    }
    
    /// copy_lines - matches buffer.c:192
    pub fn copy_lines(&mut self, first_addr: usize, second_addr: usize, addr: usize) -> Result<bool, EdError> {
        // Validate addresses match GNU ed behavior
        if first_addr == 0 || second_addr == 0 || first_addr > self.last_addr_ ||
           second_addr > self.last_addr_ || first_addr > second_addr || addr > self.last_addr_ {
            return Err(EdError::InvalidAddress);
        }

        // Set current address to destination (GNU ed line 199)
        self.current_addr_ = addr;

        // Calculate number of lines to copy
        let mut n = second_addr - first_addr + 1;
        let mut m = 0;

        // Special case: if addr is within the range being copied (GNU ed lines 200-204)
        if addr >= first_addr && addr < second_addr {
            n = addr - first_addr + 1;
            m = second_addr - addr;
        }

        // Copy lines following GNU ed algorithm (lines 205-221)
        let mut source_addr = first_addr;
        loop {
            for _ in 0..n {
                if self.too_many_lines() {
                    return Err(EdError::InvalidCommand);
                }

                // Get the line to copy (GNU ed dup_line_node)
                // source_addr is 1-based, get_line expects 1-based
                if let Some(line_content) = self.get_line(source_addr) {
                    let line_to_copy = line_content.to_string();

                    // Insert after current_addr (GNU ed add_line_node behavior)
                    // current_addr is 1-based, Vec::insert expects 0-based
                    let insert_pos = self.current_addr_;  // Insert after this position (0-based for Vec)
                    self.lines.insert(insert_pos, line_to_copy.clone());
                    self.last_addr_ = self.lines.len();
                    self.current_addr_ += 1;  // Increment to point to newly inserted line

                    // Record undo operation (GNU ed push_undo_atom)
                    self.undo_stack.push(UndoOperation::AddLine {
                        position: insert_pos,
                        line: line_to_copy
                    });
                } else {
                    return Err(EdError::InvalidAddress);
                }

                source_addr += 1;
            }

            // Check if we need to continue with second part (GNU ed algorithm)
            if m > 0 {
                n = m;
                m = 0;
                source_addr = self.current_addr_ - n + 1;  // Adjust for newly inserted lines
            } else {
                break;
            }
        }

        // Mark buffer as modified (GNU ed line 219)
        self.modified_ = 1;
        Ok(true)
    }
    
    /// delete_lines - matches buffer.c:227
    pub fn delete_lines(&mut self, from: usize, to: usize, _isglobal: bool) -> Result<bool, EdError> {
        if from > self.last_addr_ || to > self.last_addr_ || from > to {
            return Err(EdError::InvalidAddress);
        }
        
        // Record undo operations before deletion and unmark lines (GNU ed unmark_line_node)
        for line_num in from..=to {
            if let Some(line) = self.get_line(line_num) {
                self.undo_stack.push(UndoOperation::DeleteLine {
                    position: line_num - 1,
                    line: line.to_string()
                });
            }
            // Unmark any marks pointing to this line (GNU ed main_loop.c:101)
            self.unmark_line_node(line_num);
        }
        
        // Delete lines in reverse order to maintain indices
        for line_num in (from..=to).rev() {
            if line_num > 0 && line_num - 1 < self.lines.len() {
                self.lines.remove(line_num - 1);
            }
        }

        self.last_addr_ = self.lines.len();

        // Adjust marks that point to lines after the deleted range (GNU ed behavior)
        let lines_deleted = to - from + 1;
        for i in 0..26 {
            if let Some(marked_line) = self.marks[i] {
                if marked_line > to {
                    // Adjust mark to point to new line number after deletion
                    self.marks[i] = Some(marked_line - lines_deleted);
                }
            }
        }

        // Update current address (GNU ed buffer.c:239)
        // current_addr_ = min( from, last_addr_ );
        self.current_addr_ = from.min(self.last_addr_);

        self.modified_ = 1;
        Ok(true)
    }
    
    /// get_line_node_addr - matches buffer.c:246 (abstracted to index-based)
    
    /// get_sbuf_line - matches buffer.c:258
    pub fn get_sbuf_line(&self, line_num: usize) -> Option<&str> {
        if line_num == 0 || line_num > self.lines.len() {
            None
        } else {
            self.lines.get(line_num - 1).map(|s| s.as_str())
        }
    }
    
    /// join_lines - matches buffer.c:309 exactly
    pub fn join_lines(&mut self, from: usize, to: usize, isglobal: bool) -> Result<bool, EdError> {
        // Following GNU ed join_lines C implementation exactly
        if from >= to || to > self.last_addr_ {
            return Err(EdError::InvalidAddress);
        }

        // Build joined string by concatenating all lines (GNU ed logic)
        let mut joined = String::new();
        for line_num in from..=to {
            if let Some(line) = self.get_line(line_num) {
                joined.push_str(line);  // Concatenate without newlines (GNU ed behavior)
            }
        }

        // Add final newline to joined result (GNU ed adds '\n' to final buffer)
        joined.push('\n');

        // Delete the lines from from+1 to to (GNU ed delete_lines logic)
        if to > from {
            self.delete_lines(from + 1, to, isglobal)?;
        }

        // Replace the first line with joined content (GNU ed put_sbuf_line + push_undo_atom)
        self.modify_line(from, joined.trim_end_matches('\n').to_string())?;

        // Set current address to the joined line (GNU ed: current_addr_ = from - 1; but then += 1)
        self.current_addr_ = from;

        // Mark as modified (GNU ed: modified_ = true)
        self.modified_ = 1;

        Ok(true)
    }
    
    /// move_lines - matches buffer.c:341 exactly
    pub fn move_lines(&mut self, first_addr: usize, second_addr: usize, addr: usize, _isglobal: bool) -> Result<bool, EdError> {
        // Validate addresses
        if first_addr == 0 || second_addr == 0 || first_addr > self.last_addr_ ||
           second_addr > self.last_addr_ || first_addr > second_addr || addr > self.last_addr_ {
            return Err(EdError::InvalidAddress);
        }

        // GNU ed special cases: if destination is adjacent to source, it's essentially a no-op
        // addr == first_addr - 1: moving to just before the range (no change)
        // addr == second_addr: moving to just after the range (no change)
        if addr == first_addr.saturating_sub(1) || addr == second_addr {
            // Set current address and return (GNU ed behavior)
            self.current_addr_ = second_addr;
            return Ok(true);
        }

        // Extract the lines to be moved (0-based indexing for Vec operations)
        let mut moved_lines = Vec::new();
        for line_num in first_addr..=second_addr {
            if let Some(line) = self.get_line(line_num) {
                moved_lines.push(line.to_string());
            }
        }

        // Remove the lines from their original position (in reverse order to maintain indices)
        for line_num in (first_addr..=second_addr).rev() {
            if line_num > 0 && line_num <= self.lines.len() {
                self.lines.remove(line_num - 1); // Convert to 0-based
            }
        }

        // Update last_addr after deletion
        self.last_addr_ = self.lines.len();

        // Calculate insertion point after deletion
        let insert_pos = if addr < first_addr {
            // Moving to earlier position - insert position doesn't change
            addr
        } else {
            // Moving to later position - adjust for the deleted lines
            addr - (second_addr - first_addr + 1)
        };

        // Insert the moved lines at the new position
        let insert_index = if insert_pos == 0 { 0 } else { insert_pos }; // 0-based index
        for (i, line) in moved_lines.iter().enumerate() {
            if insert_index + i <= self.lines.len() {
                self.lines.insert(insert_index + i, line.clone());
            }
        }

        // Update last_addr after insertion
        self.last_addr_ = self.lines.len();

        // Update current address following GNU ed logic:
        // current_addr_ = addr + ( ( addr < first_addr ) ? second_addr - first_addr + 1 : 0 );
        self.current_addr_ = if addr < first_addr {
            addr + (second_addr - first_addr + 1)
        } else {
            addr
        };

        // Ensure current address is valid
        if self.current_addr_ > self.last_addr_ {
            self.current_addr_ = self.last_addr_;
        }

        // Mark buffer as modified (GNU ed: modified_ = true)
        self.modified_ = 1;

        Ok(true)
    }
    
    /// open_sbuf - matches buffer.c:386
    pub fn open_sbuf(&mut self) -> bool {
        // TODO: Implement string buffer opening
        true
    }
    
    /// path_max - matches buffer.c:400
    pub fn path_max(_filename: &str) -> i32 {
        // TODO: Implement path length checking
        4096 // Default path max
    }
    
    /// put_lines - matches buffer.c:412
    pub fn put_lines(&mut self, addr: usize) -> Result<bool, EdError> {
        // Check if yank buffer is empty (GNU ed line 417-418)
        if self.yank_buffer.is_empty() {
            return Err(EdError::NothingToPut);
        }

        // Validate address (GNU ed allows addr up to last_addr)
        if addr > self.last_addr_ {
            return Err(EdError::InvalidAddress);
        }

        // Set current address to insertion point (GNU ed line 419)
        self.current_addr_ = addr;

        // Insert each line from yank buffer after current address (GNU ed lines 420-436)
        for line in self.yank_buffer.iter() {
            if self.too_many_lines() {
                return Err(EdError::InvalidCommand);
            }

            // Insert after current_addr (0-based for Vec::insert)
            let insert_pos = self.current_addr_;
            self.lines.insert(insert_pos, line.clone());
            self.last_addr_ = self.lines.len();
            self.current_addr_ += 1; // Move to newly inserted line

            // Record undo operation (GNU ed push_undo_atom)
            self.undo_stack.push(UndoOperation::AddLine {
                position: insert_pos,
                line: line.clone(),
            });
        }

        // Mark buffer as modified (GNU ed line 433)
        self.modified_ = 1;

        Ok(true)
    }
    
    /// put_sbuf_line - matches buffer.c:446
    pub fn put_sbuf_line(&self, buf: &str, _size: usize) -> Option<String> {
        // TODO: Implement string buffer line output
        Some(buf.to_string())
    }
    
    /// search_line_node - matches buffer.c:483
    pub fn search_line_node(&self, addr: usize) -> Option<usize> {
        if addr > 0 && addr <= self.last_addr_ {
            Some(addr)
        } else {
            None
        }
    }
    
    /// yank_lines - matches buffer.c:510
    pub fn yank_lines(&mut self, from: usize, to: usize) -> Result<bool, EdError> {
        if from > self.last_addr_ || to > self.last_addr_ || from > to {
            return Err(EdError::InvalidAddress);
        }
        
        // Clear existing yank buffer
        self.clear_yank_buffer();
        
        // Copy lines to yank buffer
        for line_num in from..=to {
            if let Some(line) = self.get_line(line_num) {
                self.yank_buffer.push(line.to_string());
            }
        }
        
        Ok(true)
    }
    
    /// clear_undo_stack - matches buffer.c:538
    pub fn clear_undo_stack(&mut self) {
        self.undo_stack.clear();
        // Save current state for undo (matches buffer.c:555-557)
        self.u_current_addr = self.current_addr_ as i32;
        self.u_last_addr = self.last_addr_ as i32;
        self.u_modified = self.modified();
    }
    
    /// reset_undo_state - matches buffer.c:561
    pub fn reset_undo_state(&mut self) {
        self.clear_undo_stack();
        // Disable undo completely (matches buffer.c:564-565)
        self.u_current_addr = -1;
        self.u_last_addr = -1;
        self.u_modified = false;
    }
    
    // Note: free_undo_stack handled by Rust's Drop trait
    
    /// push_undo_atom - matches buffer.c:583 (internal)
    fn push_undo_atom(&mut self, op_type: i32, from: usize, _to: usize, line: String) {
        match op_type {
            1 => self.undo_stack.push(UndoOperation::AddLine { position: from, line }),
            2 => self.undo_stack.push(UndoOperation::DeleteLine { position: from, line }),
            _ => {} // Other types as needed
        }
    }
    
    /// undo - matches buffer.c:613
    pub fn undo(&mut self, _isglobal: bool) -> Result<bool, EdError> {
        // Check if undo is possible (matches buffer.c:620-621)
        if self.undo_stack.is_empty() || self.u_current_addr < 0 || self.u_last_addr < 0 {
            return Err(EdError::NothingToUndo);
        }

        // Save current state before undo (matches buffer.c:616-618)
        let o_current_addr = self.current_addr_;
        let o_last_addr = self.last_addr_;
        let o_modified = self.modified();

        // Perform undo operations (GNU ed buffer.c:624-640)
        // GNU ed: for( n = u_len - 1; n >= 0; --n ) - undoes ALL operations
        // Process all operations in reverse order (most recent first)
        while let Some(undo_op) = self.undo_stack.pop() {
            match undo_op {
                UndoOperation::AddLine { position, .. } => {
                    // Undo add: remove the added line
                    if position < self.lines.len() {
                        self.lines.remove(position);
                    }
                },
                UndoOperation::DeleteLine { position, line } => {
                    // Undo delete: restore the deleted line
                    if position <= self.lines.len() {
                        self.lines.insert(position, line);
                    }
                },
                UndoOperation::ModifyLine { position, old_line, .. } => {
                    // Undo modify: restore the old line
                    if position < self.lines.len() {
                        self.lines[position] = old_line;
                    }
                },
            }
        }

        // Update buffer state after undoing all operations
        self.last_addr_ = self.lines.len();

        // Restore undo state (matches buffer.c:648-650)
        self.current_addr_ = self.u_current_addr as usize;
        self.modified_ = if self.u_modified { 1 } else { 0 };

        // Update undo state for next undo (matches buffer.c:648-650)
        self.u_current_addr = o_current_addr as i32;
        self.u_last_addr = o_last_addr as i32;
        self.u_modified = o_modified;

        Ok(true)
    }
    
    // Additional methods for Rust convenience (not in C original)
    
    /// Get total number of lines in buffer
    pub fn len(&self) -> usize {
        self.lines.len()
    }
    
    /// Check if buffer is empty
    pub fn is_empty(&self) -> bool {
        self.lines.is_empty()
    }
    
    /// Get current line number (1-based like GNU ed) - convenience wrapper for current_addr
    pub fn current_line(&self) -> usize {
        self.current_addr()
    }
    
    /// Set current line number with bounds checking - convenience wrapper for set_current_addr
    pub fn set_current_line(&mut self, line: usize) -> Result<(), EdError> {
        if line > self.last_addr() {
            return Err(EdError::InvalidAddress);
        }
        self.set_current_addr(line);
        Ok(())
    }
    
    /// Load file into buffer - matches GNU ed file reading behavior
    /// Returns Ok(bytes_read) on success, Err for I/O errors (NOT for missing files)
    /// For missing files, prints to stderr and returns Ok with special marker
    pub fn load_file(&mut self, filename: &str) -> Result<usize, EdError> {
        use std::fs;
        use std::io::{BufRead, BufReader};

        // Try to open file
        let file = match fs::File::open(filename) {
            Ok(f) => f,
            Err(e) => {
                // GNU ed behavior: print error to stderr but continue with empty buffer
                // GNU ed io.c:299 - show_strerror() prints: "filename: strerror(errno)"
                if !crate::quiet() {
                    // Format error message to match GNU ed exactly
                    // GNU ed uses strerror() which produces "No such file or directory"
                    // Rust's io::Error::to_string() adds " (os error 2)"
                    use std::io::ErrorKind;
                    let error_msg = match e.kind() {
                        ErrorKind::NotFound => "No such file or directory",
                        _ => "Cannot open input file",
                    };
                    eprintln!("{}: {}", filename, error_msg);
                }
                // Return error to indicate file doesn't exist
                // Main.rs should NOT print byte count and should NOT exit
                return Err(EdError::FileNotFound);
            }
        };

        // Clear existing buffer
        self.lines.clear();
        self.current_addr_ = 0;
        self.last_addr_ = 0;

        // Read file line by line
        let reader = BufReader::new(file);
        let mut total_bytes = 0;

        for line_result in reader.lines() {
            match line_result {
                Ok(line) => {
                    total_bytes += line.len() + 1; // +1 for newline
                    self.lines.push_back(line);
                },
                Err(_) => return Err(EdError::InvalidAddress),
            }
        }

        // Set current line to last line (GNU ed behavior)
        self.last_addr_ = self.lines.len();
        self.current_addr_ = self.last_addr_;

        Ok(total_bytes)
    }
    
    /// Set filename
    pub fn set_filename(&mut self, filename: String) {
        self.filename = Some(filename);
    }
    
    /// Get current filename
    
    /// Get line by number (1-based like GNU ed) - convenience wrapper for get_sbuf_line
    pub fn get_line(&self, line_num: usize) -> Option<&str> {
        self.get_sbuf_line(line_num)
    }
    
    /// Append single line to buffer - convenience wrapper for append_lines
    pub fn append_line(&mut self, line: String) {
        let _ = self.append_lines(&[line], self.last_addr_);
    }
    
    /// Check if buffer has been modified - convenience wrapper for modified()
    pub fn is_modified(&self) -> bool {
        self.modified()
    }
    
    /// Delete single line by number - convenience wrapper for delete_lines
    pub fn delete_line(&mut self, line_num: usize) -> Result<(), EdError> {
        self.delete_lines(line_num, line_num, false)?;
        Ok(())
    }
    
    /// Insert line at position (0-based for internal use)
    pub fn insert_line(&mut self, position: usize, line: String) -> Result<(), EdError> {
        if position > self.lines.len() {
            return Err(EdError::InvalidAddress);
        }

        // Insert the line
        self.lines.insert(position, line.clone());

        // Create undo record for the inserted line (matches GNU ed push_undo_atom)
        self.undo_stack.push(UndoOperation::AddLine {
            position,
            line: line.clone(),
        });

        self.last_addr_ = self.lines.len();
        self.current_addr_ = position + 1; // Set current to newly inserted line (1-based)
        self.modified_ = 1;

        // Adjust marks that point to lines at or after the insertion point (GNU ed behavior)
        let insert_line_1based = position + 1; // Convert to 1-based addressing
        for i in 0..26 {
            if let Some(marked_line) = self.marks[i] {
                if marked_line >= insert_line_1based {
                    // Adjust mark to point to new line number after insertion
                    self.marks[i] = Some(marked_line + 1);
                }
            }
        }

        Ok(())
    }
    
    /// Clear buffer (for 'e' command)
    pub fn clear_buffer(&mut self) {
        self.lines.clear();
        self.current_addr_ = 0;
        self.last_addr_ = 0;
        self.modified_ = 0;
        self.clear_undo_stack(); // Clear undo history when buffer is cleared
    }
    
    /// Clear modified flag (after file operations) - convenience wrapper for set_modified
    pub fn clear_modified_flag(&mut self) {
        self.set_modified(false);
    }
    
    /// Undo last operation - convenience wrapper for undo
    pub fn undo_last_operation(&mut self) -> Result<(), EdError> {
        match self.undo(false) {
            Ok(_) => Ok(()),
            Err(err) => Err(err)
        }
    }
    
    // Search functions moved to regex.rs to match C structure
    // Use regex::next_matching_node_addr for search functionality
    
    /// Modify line with undo tracking (for substitute command)
    pub fn modify_line(&mut self, line_num: usize, new_content: String) -> Result<(), EdError> {
        if line_num == 0 || line_num > self.last_addr_ {
            return Err(EdError::InvalidAddress);
        }

        // Record undo operation before modifying
        if let Some(old_line) = self.lines.get(line_num - 1) {
            self.undo_stack.push(UndoOperation::ModifyLine {
                position: line_num - 1,
                old_line: old_line.clone(),
                new_line: new_content.clone(),
            });
        }

        // Actually modify the line
        self.lines[line_num - 1] = new_content;
        self.modified_ = 1;
        self.current_addr_ = line_num;

        Ok(())
    }

    // Mark-related functions - matches main_loop.c:91-116

    /// mark_line_node - matches main_loop.c:91
    /// Mark a line with a given character (a-z)
    pub fn mark_line_node(&mut self, line_addr: usize, c: char) -> Result<(), EdError> {
        // Convert character to index (GNU ed logic: c -= 'a')
        let index = (c as u8).wrapping_sub(b'a') as usize;
        if index >= 26 {
            return Err(EdError::InvalidCommand); // Invalid mark character
        }

        // Validate line address
        if line_addr == 0 || line_addr > self.last_addr_ {
            return Err(EdError::InvalidAddress);
        }

        // If this mark wasn't set before, increment mark count
        if self.marks[index].is_none() {
            self.markno += 1;
        }

        // Set the mark to point to this line (GNU ed: mark[c] = lp)
        self.marks[index] = Some(line_addr);

        Ok(())
    }

    /// unmark_line_node - matches main_loop.c:101
    /// Remove marks pointing to a specific line (called when line is deleted)
    pub fn unmark_line_node(&mut self, line_addr: usize) {
        for i in 0..26 {
            if self.markno > 0 && self.marks[i] == Some(line_addr) {
                self.marks[i] = None;
                self.markno -= 1;
            }
        }
    }

    /// get_marked_node_addr - matches main_loop.c:111
    /// Return the line address of a marked line
    pub fn get_marked_node_addr(&self, c: char) -> Result<usize, EdError> {
        // Convert character to index (GNU ed logic: c -= 'a')
        let index = (c as u8).wrapping_sub(b'a') as usize;
        if index >= 26 {
            return Err(EdError::InvalidCommand); // Invalid mark character
        }

        // Return the marked line address, or error if not set
        match self.marks[index] {
            Some(addr) => Ok(addr),
            None => Err(EdError::InvalidAddress), // Mark not set or line deleted
        }
    }
}