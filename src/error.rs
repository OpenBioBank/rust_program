//!Defines program-specific errors

use solana_program::program_error::ProgramError;
use thiserror::Error;

#[derive(Error, Debug, Copy, Clone)]
pub enum MintingError {
    #[error("InvalidInstruction")]
    InvalidInstruction,
    
    #[error("Accounts do not match")]
    IncorrectAccountError,

    #[error("PDA derived does not equal PDA passed in")]
    InvalidPDA,

    #[error("Input data exceeds max length")]
    InvalidDataLength,

    #[error("Rating greater than 5 or less than 1")]
    InvalidRating,

}

impl From<MintingError> for ProgramError {
    fn from(e: MintingError) -> Self {
        ProgramError::Custom(e as u32)
    }
}
