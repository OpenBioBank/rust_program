//!Defines program-specific errors

use solana_program::program_error::ProgramError;
use thiserror::Error;

#[derive(Error, Debug, Copy, Clone)]
pub enum MintingError {
    #[error("InvalidInstruction")]
    InvalidInstruction,
    #[error("Accounts do not match")]
    IncorrectAccountError,
}

impl From<MintingError> for ProgramError {
    fn from(e: MintingError) -> Self {
        ProgramError::Custom(e as u32)
    }
}
