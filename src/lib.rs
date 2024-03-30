///
mod error;
mod instruction;
mod processor;
mod state;

use instruction::Instruction;
use processor::{mint_nft, transfer_ownership, update_metadata};

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
    let instruction = Instruction::try_from_slice(instruction_data);

    match instruction {
        Instruction::MintNft { metadata } => mint_nft(program_id, accounts, metadata),
        Instruction::UpdateNftMetadata { new_metadata } => {
            update_metadata(program_id, accounts, new_metadata)
        }

        Instruction::TransferOwnership { new_owner } => {
            transfer_ownership(program_id, accounts, &new_owner)
        }
    }
}
