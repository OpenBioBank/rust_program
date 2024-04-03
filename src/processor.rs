use crate::error::MintingError;
use borsh::BorshSerialize;
//use borsh::BorshSerialize;
//use solana_program::address_lookup_table::program;
use solana_program::borsh1::try_from_slice_unchecked;
use solana_program::program_error::ProgramError;
use solana_program::system_program;
//use solana_program::native_token::LAMPORTS_PER_SOL;
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program::invoke_signed,
    pubkey::Pubkey,
    rent::Rent,
    system_instruction,
    sysvar::Sysvar,
};
use spl_associated_token_account::get_associated_token_address;
use spl_token::{instruction::initialize_mint, ID as TOKEN_PROGRAM_ID};
//use std::convert::TryInto;
//use mpl_token_metadata::programs;
use crate::state::MetadataAccount;

pub fn initialize_token_mint(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    id: u64,
    description: String,
    owner: String,
    creator: String,
    authorize: bool,
    url: String,
    cid: String,
    is_mutable: bool,
) -> ProgramResult {
    //   verify the program ID from the instruction is in fact
    if system_program::check_id(program_id) {
        return Err(ProgramError::IncorrectProgramId);
    };

    //check number of accounts
    if accounts.len() < 7 {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    let account_info_iter = &mut accounts.iter();

    let initializer = next_account_info(account_info_iter)?;
    let token_mint = next_account_info(account_info_iter)?;
    let mint_auth = next_account_info(account_info_iter)?;
    let token_metadata = next_account_info(account_info_iter)?;
    let system_program = next_account_info(account_info_iter)?;
    let token_program = next_account_info(account_info_iter)?;
    let sysvar_rent = next_account_info(account_info_iter)?;

    //create mint_pda
    msg!("deriving mint authority");
    //mint_pda = initializer + programId
    let (mint_pda, mint_bump) =
        Pubkey::find_program_address(&[initializer.key.as_ref(), &cid.as_ref()], program_id);

    //mint_ayth_pda = mint_pda + programId
    let (mint_auth_pda, _mint_auth_bump) =
        Pubkey::find_program_address(&[token_mint.key.as_ref()], program_id);

    msg!("Token mint: {:?}", mint_pda);
    msg!("Mint authority: {:?}", mint_auth_pda);

    if mint_pda != *token_mint.key {
        msg!("Incorrect token mint account");
        return Err(MintingError::IncorrectAccountError.into());
    }

    if *token_program.key != TOKEN_PROGRAM_ID {
        msg!("Incorrect token program");
        return Err(MintingError::IncorrectAccountError.into());
    }

    if *mint_auth.key != mint_auth_pda {
        msg!("Incorrect mint auth account");
        return Err(MintingError::IncorrectAccountError.into());
    }

    let rent = Rent::get()?;
    let rent_lamports = rent.minimum_balance(82);

    //create mint_pda
    invoke_signed(
        &system_instruction::create_account(
            initializer.key,
            token_mint.key,
            rent_lamports,
            300,
            token_program.key,
        ),
        &[
            initializer.clone(),
            token_mint.clone(),
            system_program.clone(),
        ],
        &[&[initializer.key.as_ref(), cid.as_bytes(), &[mint_bump]]],
    )?;

    msg!("Created token mint account");

    //initialize
    invoke_signed(
        &initialize_mint(
            token_program.key,
            token_mint.key,
            mint_auth.key,
            Option::None,
            0,
        )?,
        &[token_mint.clone(), sysvar_rent.clone(), mint_auth.clone()],
        &[&[initializer.key.as_ref(), &cid.as_ref(), &[mint_bump]]],
    )?;

    msg!("Initialized token mint");

    msg!("Initialized token_metadata...");

    //metadata = token_mint + mint_auth + program_id
    let (metadata_pda, metadata_bump) = Pubkey::find_program_address(
        &[token_mint.key.as_ref(), mint_auth.key.as_ref()],
        program_id,
    );

    if metadata_pda != *token_metadata.key {
        msg!("Incorrect token metadata account");
        return Err(MintingError::IncorrectAccountError.into());
    }

    let account_len: usize = 308
        + (4 + description.len())
        + (4 + owner.len())
        + (4 + owner.len())
        + 1
        + (4 + owner.len())
        + (4 + creator.len())
        + (4 + url.len());

    let rent = Rent::get()?;
    let rent_lamports = rent.minimum_balance(account_len);

    //create metadata
    invoke_signed(
        &system_instruction::create_account(
            initializer.key,
            token_metadata.key,
            rent_lamports,
            300,
            token_program.key,
        ),
        &[
            initializer.clone(),
            token_metadata.clone(),
            system_program.clone(),
        ],
        &[&[
            token_mint.key.as_ref(),
            mint_auth.key.as_ref(),
            &[metadata_bump],
        ]],
    )?;

    msg!("Create metadata");
    let mut account_data =
        try_from_slice_unchecked::<MetadataAccount>(&token_metadata.data.borrow()).unwrap();

    account_data.id = id;
    account_data.description = description;
    account_data.authorize = authorize;
    account_data.cid = cid;
    account_data.creator = creator;
    account_data.owner = owner;
    account_data.url = url;
    account_data.is_mutable = is_mutable;

    account_data.serialize(&mut &mut token_metadata.data.borrow_mut()[..])?;

    Ok(())
}

pub fn create_new(program_id: &Pubkey, accounts: &[AccountInfo]) -> ProgramResult {
    //解析账户
    let account_info_iter = &mut accounts.iter();

    let initializer = next_account_info(account_info_iter)?;

    //fronted:
    // const [tokenMint] = await web3.PublicKey.findProgramAddress(
    //     [Buffer.from("token_mint")],
    //     new web3.PublicKey(MOVIE_REVIEW_PROGRAM_ID)
    // )

    // const [mintAuth] = await web3.PublicKey.findProgramAddress(
    //     [Buffer.from("token_auth")],
    //     new web3.PublicKey(MOVIE_REVIEW_PROGRAM_ID)
    // )
    let token_mint = next_account_info(account_info_iter)?;
    let mint_auth = next_account_info(account_info_iter)?;

    //fronted:
    // const userAta = await getAssociatedTokenAddress(tokenMint, publicKey)
    // const ataAccount = await connection.getAccountInfo(userAta)

    // if (!ataAccount) {
    //     const ataInstruction = createAssociatedTokenAccountInstruction(
    //         publicKey,
    //         userAta,
    //         publicKey,
    //         tokenMint
    //     )

    //     transaction.add(ataInstruction)
    // }
    let user_ata = next_account_info(account_info_iter)?;
    let token_program = next_account_info(account_info_iter)?;

    // To create a new account in our plan we must:
    // Calculate the space and rent required for the account
    // Have an address to assign new accounts to
    // Call the system program to create a new account

    // Calculate rent required

    msg!("deriving mint authority");
    let (mint_pda, _mint_bump) =
        Pubkey::find_program_address(&[initializer.key.as_ref()], program_id);
    let (mint_auth_pda, mint_auth_bump) =
        Pubkey::find_program_address(&[mint_pda.as_ref()], program_id);

    if *token_mint.key != mint_pda {
        msg!("Incorrect token mint");
        return Err(MintingError::IncorrectAccountError.into());
    }

    if *mint_auth.key != mint_auth_pda {
        msg!("Mint passed in and mint derived do not match");
        return Err(MintingError::InvalidPDA.into());
    }

    if *user_ata.key != get_associated_token_address(initializer.key, token_mint.key) {
        msg!("Incorrect token mint");
        return Err(MintingError::IncorrectAccountError.into());
    }

    if *token_program.key != TOKEN_PROGRAM_ID {
        msg!("Incorrect token program");
        return Err(MintingError::IncorrectAccountError.into());
    }

    msg!("Minting 1 tokens to User associated token account");
    invoke_signed(
        // instruction
        &spl_token::instruction::mint_to(
            token_program.key,
            token_mint.key,
            user_ata.key, //
            mint_auth.key,
            &[],
            1,
        )?,
        // account_infos
        &[token_mint.clone(), user_ata.clone(), mint_auth.clone()],
        // seeds
        &[&[token_mint.key.as_ref(), &[mint_auth_bump]]],
    )?;

    Ok(())
}

pub fn find_cid() -> ProgramResult {
    //delete account
    todo!()
}
