use solana_program::{entrypoint::ProgramResult, rent::Rent, sysvar::Sysvar};

pub fn create_new( 
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    id: u64,
    description: String,
    owner: String,
    creator: String,
    authorize: bool,
    url: String,
) -> ProgramResult {
    //解析账户
let wallet_account="wallet_account";
    //创建pda账户

    //用program_id和钱包的acount

    // To create a new account in our plan we must:
    // Calculate the space and rent required for the account
    // Have an address to assign new accounts to
    // Call the system program to create a new account
    let account_len: usize =
        8 + (4 + description.len()) + (4 + owner.len()) + (4 + owner.len()) + 1 + (4 + owner.len());

    // Calculate rent required
    let rent = Rent::get()?;
    let rent_lamports = rent.minimum_balance(account_len);
    todo!()
}

pub fn delete() -> ProgramResult {
    //delete account
    todo!()
}
