use crate::instruction::Instruction;

use crate::processor::{create_new, initialize_token_mint, print_func};

use solana_program::{
    account_info::AccountInfo, entrypoint, entrypoint::ProgramResult, msg, pubkey::Pubkey,
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
        Instruction::InitializeMintAccount {
            id,
            owner,
            creator,
            description,
            authorize,
            url,
            cid,
            is_mutable,
        } => {
            msg!("hello");
            initialize_token_mint(
                program_id,
                accounts,
                id,
                owner,
                creator,
                description,
                authorize,
                url,
                cid,
                is_mutable,
            )?;
        }

        Instruction::Create { cid } => create_new(program_id, accounts, cid)?,

        Instruction::Test {
            id,
            description,
            authorize,
        } => print_func(id, description, authorize)?,
    };
    Ok(())
}
