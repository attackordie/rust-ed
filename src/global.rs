/// Global command routines - Safe Rust implementation 
/// This file provides the same functionality as global.c but with memory safety
/// C source: global.c (96 lines, 3,084 bytes) - IMMUTABLE REFERENCE
/// 
/// 🔒 MEMORY SAFETY: Converted from C raw pointers to safe Rust collections
/// The original C code uses manual memory management for a dynamic array of line_node pointers.
/// Our Rust version uses line indices with Vec<Option<usize>> for identical functionality.

use std::sync::Mutex;

// Safe Rust replacement for C's dynamic array of line_node pointers
// In ed-rust we use line indices instead of pointers since our buffer is Vec-based
static ACTIVE_LIST: Mutex<Vec<Option<usize>>> = Mutex::new(Vec::new());
static ACTIVE_IDX: Mutex<usize> = Mutex::new(0);    // current iteration index
static ACTIVE_IDXM: Mutex<usize> = Mutex::new(0);   // modulo index for removal

/// clear_active_list - matches global.c:36 (now memory safe)
pub fn clear_active_list() {
    // Safe Rust implementation - no manual memory management needed
    crate::signal::disable_interrupts();
    
    // Clear all data safely using Mutex guards
    if let Ok(mut list) = ACTIVE_LIST.lock() {
        list.clear();
    }
    if let Ok(mut idx) = ACTIVE_IDX.lock() {
        *idx = 0;
    }
    if let Ok(mut idxm) = ACTIVE_IDXM.lock() {
        *idxm = 0;
    }
    
    crate::signal::enable_interrupts();
}

/// next_active_node - matches global.c:47 (now memory safe)
pub fn next_active_line() -> Option<usize> {
    // Safe implementation using line indices instead of raw pointers
    let list = ACTIVE_LIST.lock().ok()?;
    let mut idx = ACTIVE_IDX.lock().ok()?;
    
    // Find next active line, skipping None entries
    while *idx < list.len() {
        let current_idx = *idx;
        *idx += 1;
        
        if let Some(line_addr) = list[current_idx] {
            return Some(line_addr);
        }
    }
    
    None
}

/// set_active_node - matches global.c:56 (now memory safe) 
pub fn set_active_line(line_addr: usize) -> bool {
    // Safe implementation - Vec automatically manages memory
    crate::signal::disable_interrupts();
    
    let result = if let Ok(mut list) = ACTIVE_LIST.lock() {
        // Check for reasonable limits to prevent DoS
        if list.len() >= 10_000_000 {
            // TODO: set_error_msg("Too many matching lines");
            false
        } else {
            list.push(Some(line_addr));
            true
        }
    } else {
        false
    };
    
    crate::signal::enable_interrupts();
    result
}

/// unset_active_lines - matches global.c:82 (now memory safe)
pub fn unset_active_lines(start_addr: usize, end_addr: usize) {
    // Safe implementation using line address ranges instead of raw pointers
    crate::signal::disable_interrupts();
    
    if let (Ok(mut list), Ok(mut idxm)) = (ACTIVE_LIST.lock(), ACTIVE_IDXM.lock()) {
        // Remove all lines in the range [start_addr, end_addr)
        for line_addr in start_addr..end_addr {
            // Find and remove matching line addresses - matches global.c:86-92 logic
            for _ in 0..list.len() {
                if *idxm >= list.len() {
                    *idxm = 0;
                }
                
                if list[*idxm] == Some(line_addr) {
                    list[*idxm] = None;  // Set to None instead of null pointer
                    break;
                }
                *idxm += 1;
            }
        }
    }
    
    crate::signal::enable_interrupts();
}