//!Contains the transaction processing logic

use solana_program::{account_info::AccountInfo, entrypoint::ProgramResult, pubkey::Pubkey};

use crate::instruction::MintNftInstruction;

pub fn process(
    _program_id: &Pubkey,
    _accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let _instruction = MintNftInstruction::unpack(instruction_data)?;

    // Verify logic such as project party address...

    //Minting NFT logic...

    Ok(())
}
