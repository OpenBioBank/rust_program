use solana_program::{account_info::{next_account_info, AccountInfo}, entrypoint::ProgramResult, program::invoke_signed, pubkey::Pubkey, rent::Rent, sysvar::Sysvar};


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
    let account_info_iter = &mut accounts.iter();
    let wallet_account = next_account_info(account_info_iter)?;

    //创建pda账户

    //用program_id和钱包的acount 创建一个mint Nft 的账户

    //find_program_address()
    //计算rent length
    //invoke_signed(create_account(创建账户), account_infos, signers_seeds)

    //用wallet account + mint NFT生成一个 接收新铸造Nft的账户

    //铸造Nft

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

pub fn find_cid() -> ProgramResult {
    //delete account
    todo!()
}
