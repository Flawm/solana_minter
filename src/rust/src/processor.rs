use {
    crate::error::MintError,
    borsh::{BorshDeserialize, BorshSerialize},
    solana_program::{
        native_token::LAMPORTS_PER_SOL,
        account_info::{next_account_info, AccountInfo},
        entrypoint::ProgramResult,
        program_error::ProgramError,
        system_instruction,
        pubkey::Pubkey,
        program,
        fee_calculator,
        program_pack::Pack,
        clock::Clock,
        sysvar::Sysvar,
        msg,
    },
    metaplex_token_metadata::{
        instruction::{create_metadata_accounts, update_metadata_accounts}
    },
    spl_token::{
        state::{
            Account,
            Mint
        },
    },
};

const PREFIX: &str             = "amoebit_minter";
const TOKEN_PROGRAM: &str      = "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA";
const TOKEN_META_PROGRAM: &str = "metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s";
const OUR_PUB_KEY: &str        = "82UV2Exn5FaS4Ys1fxiHBPW1tQVeU1R76BbVkAE7WaTL";
const OUR_WALLET: &str         = "AmbtTL5LS42RFL1ZL5QQan8ZSyn27pvVoCbFYF2eTwyH";
const RENT_ACCOUNT: &str       = "SysvarRent111111111111111111111111111111111";
const INDEX_KEY: &str          = "";

// DEVNET
const MINT_KEY: &str           = "DwuhyNAQYjJHKZJEkVLy5Phoz83Tty6whVcZ79eQ7rXs";
const DISCOUNT_KEY: &str       = "Hzn4ehrSbJstaGTjx1MP7K8EGJTSzjFDqZ3yFcWny332";

// MAIN NET
//const MINT_KEY: &str           = "FZHVSqXQkJ5cwoBAoztKicFpGvvLkHNzNos22B8kx7cF";
//const DISCOUNT_KEY: &str       = "DtwxbDVacacZLqUwgZjQtz2aYTAaCKiZXkisMeDX7bkU";

const PRICE: u64               = 300000000; // .3

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct AmoebitIndex {
    pub counter: u16,
}

pub fn process_instruction<'a>(
    program_id: &'a Pubkey,
    accounts: &'a [AccountInfo<'a>],
    _input: &[u8],
) -> ProgramResult {
    let accounts_iter          = &mut accounts.iter();

    let index_account          = next_account_info(accounts_iter)?; // 0
    let payer_account          = next_account_info(accounts_iter)?; // 1
    let wallet_account         = next_account_info(accounts_iter)?; // 2
    let sys_account            = next_account_info(accounts_iter)?; // 3
    let token_account          = next_account_info(accounts_iter)?; // 4
    let mint_account           = next_account_info(accounts_iter)?; // 5
    let meta_account           = next_account_info(accounts_iter)?; // 6
    let meta_program_account   = next_account_info(accounts_iter)?; // 7
    let rent_account           = next_account_info(accounts_iter)?; // 8
    let auth_account           = next_account_info(accounts_iter)?; // 9
    let token_program_account  = next_account_info(accounts_iter)?; // 10
    let wl_mint_account        = next_account_info(accounts_iter)?; // 11
    let wl_payer_account       = next_account_info(accounts_iter)?; // 12
    let discount_mint_account  = next_account_info(accounts_iter)?; // 13
    let discount_payer_account = next_account_info(accounts_iter)?; // 14

    // SAFETY CHECKS
    // you want to uncomment this with your index account & key.
    //if index_account.key.to_string()         != INDEX_KEY          { return Err(ProgramError::InvalidAccountData); }
    if wallet_account.key.to_string()        != OUR_WALLET         { return Err(ProgramError::InvalidAccountData); }
    if wl_mint_account.key.to_string()       != MINT_KEY           { return Err(ProgramError::InvalidAccountData); }
    if discount_mint_account.key.to_string() != DISCOUNT_KEY       { return Err(ProgramError::InvalidAccountData); }

    let mut series_index = AmoebitIndex::try_from_slice(&index_account.data.borrow())?;

    if series_index.counter == 5000 {
        return Err(MintError::NoneLeft.into());
    }

    let token_data: Account = Pack::unpack(&token_account.data.borrow())?;
    let mint_data: Mint     = Pack::unpack(&mint_account.data.borrow())?;

    // Make sure client sent a proper NFT
    if token_data.amount           != 1                 { return Err(MintError::EmptyToken.into()); }
    if mint_data.decimals          != 0                 { return Err(MintError::InvalidMint.into()); }
    if mint_data.supply            != 1                 { return Err(MintError::InvalidMint.into()); }
    if !mint_data.freeze_authority.is_none()            { return Err(MintError::InvalidMint.into()); }
    if token_data.mint             != *mint_account.key { return Err(MintError::InvalidMint.into()); }

    let release_time: i64   = 1636758000;
    // TEST TIME
    //let release_time: i64   = 1535084000;

    let current_time: Clock = Sysvar::get()?;

    if payer_account.key.to_string() != OUR_PUB_KEY {
        let mut divide_by = 0;
        let mut gas_burn = 0;

        let whitelisted         = &wl_payer_account;
        let discount            = &discount_payer_account;

        let wl_or_discount = discount.data.borrow().len() > 0 || whitelisted.data.borrow().len() > 0;

        // check WL / presales
        if wl_or_discount {
            if discount.data.borrow().len() > 0 && discount.data.borrow()[64] > 0 && true {
                program::invoke(
                    &spl_token::instruction::burn(
                        &token_program_account.key,
                        &discount_payer_account.key,
                        &discount_mint_account.key,
                        &payer_account.key,
                        &[&payer_account.key],
                        1
                    )?,
                    &[
                        token_program_account.clone(),
                        discount_payer_account.clone(),
                        discount_mint_account.clone(),
                        payer_account.clone(),
                    ]
                )?;

                divide_by = 2;
                gas_burn = 9127600;
            } else if false {
                program::invoke(
                    &spl_token::instruction::burn(
                        &token_program_account.key,
                        &wl_payer_account.key,
                        &wl_mint_account.key,
                        &payer_account.key,
                        &[&payer_account.key],
                        1
                    )?,
                    &[
                        token_program_account.clone(),
                        wl_payer_account.clone(),
                        wl_mint_account.clone(),
                        payer_account.clone(),
                    ]
                )?;
                //gas_burn = 9127600;
            } else if current_time.unix_timestamp < release_time {
                return Err(MintError::Presale.into());
            }
        } else if current_time.unix_timestamp < release_time {
            return Err(MintError::Unavailable.into());
        }

        // the final price to the payer deducted from all the account fees so they pay a flat fee
        // 5616720 - cost of metadata account
        // 0010000 - tx fees
        let mut transaction_p    = PRICE - token_account.lamports() - mint_account.lamports() - (fee_calculator::DEFAULT_TARGET_LAMPORTS_PER_SIGNATURE * 2) - 5606720 - gas_burn;
        let mut total_p          = PRICE;

        if divide_by > 0 {
            transaction_p /= divide_by;
            total_p /= divide_by;
        }

        // check the payer account can fund entire transaction
        if payer_account.lamports() < total_p {
            return Err(ProgramError::InsufficientFunds);
        }

        program::invoke(
            &system_instruction::transfer(
                &payer_account.key,
                &wallet_account.key,
                transaction_p,
            ),
            &[
                payer_account.clone(),
                wallet_account.clone(),
                sys_account.clone()
            ]
        )?;
    }

    let mut uri = "https://api.amoebits.io/get/amoebits_".to_string();
    let mut name = "Amoebit #".to_string();

    uri += &series_index.counter.to_string();
    name += &series_index.counter.to_string();

    let auth_seeds = &[
        PREFIX.as_bytes(),
        program_id.as_ref(),
        PREFIX.as_bytes(),
    ];

    let (auth_key, bump_seed) = 
        Pubkey::find_program_address(auth_seeds, program_id);

    let authority_seeds: &[&[_]] = &[
        PREFIX.as_bytes(),
        program_id.as_ref(),
        PREFIX.as_bytes(),
        &[bump_seed]
    ];

    // safety check (may be not needed because tx will fail(?))
    if auth_key != *auth_account.key {
        return Err(MintError::AuthKeyFailure.into());
    }

    let creators = vec![
        metaplex_token_metadata::state::Creator {
            address: *auth_account.key,
            verified: true,
            share: 0
        },
        metaplex_token_metadata::state::Creator {
            address: *wallet_account.key,
            verified: false,
            share: 100
        },
    ];

    let cmda_instruction = create_metadata_accounts(
        *meta_program_account.key,
        *meta_account.key,
        *mint_account.key,
        *payer_account.key,
        *payer_account.key,
        *auth_account.key,
        name.to_string(),
        "AMBT".to_string(),
        uri.to_string(),
        Some(creators),
        500,
        true,
        true
    );

    let metadata_infos = vec![
        meta_account.clone(),
        mint_account.clone(),
        payer_account.clone(),
        meta_program_account.clone(),
        rent_account.clone(),
        auth_account.clone()
    ];

    // create meta data accounts
    program::invoke_signed(
        &cmda_instruction,
        metadata_infos.as_slice(),
        &[&authority_seeds]
    )?;

    let update_infos = vec![
        meta_program_account.clone(),
        meta_account.clone(),
        auth_account.clone(),
    ];

    // denote that the primary sale has happened
    program::invoke_signed(
        &update_metadata_accounts(
            *meta_program_account.key,
            *meta_account.key,
            *auth_account.key,
            None,
            None,
            Some(true),
        ),
        update_infos.as_slice(),
        &[&authority_seeds],
    )?;

    // disable mint
    program::invoke(
        &spl_token::instruction::set_authority(
            &token_program_account.key,
            &mint_account.key,
            None,
            spl_token::instruction::AuthorityType::MintTokens,
            &payer_account.key,
            &[&payer_account.key]
        )?,
        &[
            payer_account.clone(),
            mint_account.clone(),
            token_program_account.clone()
        ]
    )?;

    msg!("{}", series_index.counter);

    series_index.counter += 1;
    series_index.serialize(&mut &mut index_account.data.borrow_mut()[..])?;

    Ok(())
}
