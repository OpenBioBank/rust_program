//!Defines the program instructions and parameters

use solana_program::{program_error::ProgramError, pubkey::Pubkey};

use borsh::{BorshDeserialize, BorshSchema, BorshSerialize};

#[derive(BorshSerialize, BorshDeserialize, BorshSchema)]
pub struct MintNftInstruction {
    pub user_wallet_address: Pubkey,
    pub cid: String,
}

impl MintNftInstruction {
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        todo!()
    }
}
