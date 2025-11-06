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

/// Error handling for rust-ed - must match GNU ed error behavior exactly

#[derive(Debug, thiserror::Error)]
pub enum EdError {
    #[error("?")]  // GNU ed style error message
    InvalidCommand,

    #[error("?")]
    InvalidAddress,

    #[error("?")]
    InvalidFilename,

    #[error("?")]
    IoError(String),

    #[error("?")]  // GNU ed: No match error for substitute
    NoMatch,

    #[error("?")]  // GNU ed: Nothing to undo error
    NothingToUndo,

    #[error("?")]  // GNU ed: Nothing to put error (empty yank buffer)
    NothingToPut,

    #[error("?")]  // GNU ed: Pattern not found in search
    PatternNotFound,

    #[error("?")]  // GNU ed: Warning - buffer modified (first quit attempt)
    WarningUnsavedChanges,

    #[error("")]  // Quit is not an error, just a control flow
    Quit,
}

impl EdError {
    /// Get exit code that matches GNU ed exactly
    pub fn exit_code(&self) -> i32 {
        match self {
            EdError::InvalidCommand => 1,
            EdError::InvalidAddress => 1,
            EdError::InvalidFilename => 1,
            EdError::IoError(_) => 1,
            EdError::NoMatch => 1,
            EdError::NothingToUndo => 1,
            EdError::NothingToPut => 1,
            EdError::PatternNotFound => 1,
            EdError::WarningUnsavedChanges => 1,  // Exit code 1 to indicate error
            EdError::Quit => 0,
        }
    }
}