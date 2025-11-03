/// Error handling for ed-rust - must match GNU ed error behavior exactly

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