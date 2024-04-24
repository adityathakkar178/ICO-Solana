use {
    borsh::{BorshDeserialize, BorshSerialize},
    solana_program::{
        account_info::{next_account_info, AccountInfo},
        entrypoint::ProgramResult,
        msg,
        program::invoke, 
        pubkey::Pubkey,
    },
    spl_token::instruction as token_instruction,
};

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct WhitelistArgs {
    pub accounts: Pubkey,
    pub is_whitelisted: bool,
}

pub fn whitelist_account(
    _args: WhitelistArgs,
) -> ProgramResult {


    Ok(())
}