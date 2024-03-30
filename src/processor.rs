//!Contains the transaction processing logic

use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, program_error::ProgramError,
    pubkey::Pubkey,
};

use crate::instruction::MintNftInstruction;

pub fn process(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let instruction = MintNftInstruction::unpack(instruction_data)?;

    // Verify logic such as project party address...

    //Minting NFT logic...

    Ok(())
}
