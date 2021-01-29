use thiserror::Error;
use solana_program::program_error::ProgramError;

#[derive(Error, Debug, Copy, Clone)]
pub enum EscrowError {
    /// Invalid instruction
    #[error("Invalid instruction")]
    InvalidInstruction,
    /// Not Rent Exempt
    #[error("Not rent exempt")]
    NotRentExempt,
    /// Expected amount mismatch
    #[error("Expected amount mismatch")]
    ExpectedAmountMisMatch,
    /// Amount overflow
    #[error("Amount overlow")]
    AmountOverflow,
}

impl From<EscrowError> for ProgramError {
    fn from(e: EscrowError) -> Self {
        ProgramError::Custom(e as u32)
    }
}
