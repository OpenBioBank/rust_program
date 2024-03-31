///
mod error;
mod instruction;
mod processor;
mod state;

use instruction::Instruction;

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
            // Execute program code to create a note
            todo!()
        }
        Instruction::Update { url } => {
            // Execute program code to update a note
            todo!()
        }
        Instruction::Delete { id } => {
            // Execute program code to delete a note
            todo!()
        }
        Instruction::Authorize { authorize } => {
            //Authorize
            todo!()
        }
    }
}
