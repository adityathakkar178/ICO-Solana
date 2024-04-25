use {
    borsh::{BorshDeserialize, BorshSerialize},
    solana_program::{
        account_info::{next_account_info, AccountInfo},
        entrypoint::ProgramResult,
        msg,
        pubkey::Pubkey,
    },
};

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct WhitelistArgs {
    pub accounts: Vec<Pubkey>,
    pub is_whitelist: bool,
}

pub fn whitelist_account(accounts: &[AccountInfo], _args: &mut WhitelistArgs) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();

    let whitelist_account = next_account_info(accounts_iter)?;

    _args.accounts.push(*whitelist_account.key);
    _args.is_whitelist = true;
    msg!("Account has bee added to the whitelist");
    Ok(())
}
