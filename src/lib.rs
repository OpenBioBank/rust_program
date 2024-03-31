///
mod error;
mod instruction;
mod processor;
mod state;

use instruction::Instruction;

use processor::{authorize_account, create_new, delete, update_url};
use solana_program::{
    account_info::AccountInfo, entrypoint, entrypoint::ProgramResult, pubkey::Pubkey,
};

entrypoint!(process_instruction);

fn process_instruction(
    _program_id: &Pubkey,
    _accounts: &[AccountInfo],
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
            create_new(id, owner, creator, description, authorize, url)?;
        }

        Instruction::Delete { id } => {
            // Execute program code to delete a note
            delete()?;
        }
    };
    Ok(())
}
