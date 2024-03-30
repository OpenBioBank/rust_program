//!Contains the transaction processing logic

use solana_program::{account_info::AccountInfo, entrypoint::ProgramResult, pubkey::Pubkey};

pub fn transfer_ownership(
    _program_id: &Pubkey,
    _accounts: &[AccountInfo<'_>],
    _new_owner: &Pubkey,
) -> ProgramResult {
    todo!()
}

pub fn update_metadata(
    _program_id: &Pubkey,
    _accounts: &[AccountInfo<'_>],
    _new_metadata: String,
) -> ProgramResult {
    todo!()
}

pub fn mint_nft(
    _program_id: &Pubkey,
    _accounts: &[AccountInfo<'_>],
    _metadata: String,
) -> ProgramResult {
    todo!()
}
