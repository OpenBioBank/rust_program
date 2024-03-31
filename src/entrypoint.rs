use crate::instruction::Instruction;

use crate::processor::{create_new, find_cid};
use solana_program::program_error::ProgramError;
use solana_program::{
    account_info::AccountInfo, entrypoint, entrypoint::ProgramResult, pubkey::Pubkey,
};

entrypoint!(process_instruction);

fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    // Parse instruction data into NftInstruction enumeration
    // Call unpack to deserialize instruction_data
    let instruction = Instruction::unpack(instruction_data)?;
    // Match the returned data struct to what you expect
    match instruction {
        Instruction::Create {
            id,
            owner,
            creator,
            description,
            authorize,
            url,
        } => {
            create_new(program_id, accounts, id, owner, creator, description, authorize, url)?;
        },

        Instruction::FindCid => find_cid()?,

        _ => Err(ProgramError::InvalidInstructionData)?,

        
    };
    Ok(())
}
