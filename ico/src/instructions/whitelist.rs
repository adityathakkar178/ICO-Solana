use {
    borsh::{BorshDeserialize, BorshSerialize},
    solana_program::{entrypoint::ProgramResult, msg, pubkey::Pubkey},
};

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct WhitelistArgs {
    pub accounts: Vec<Pubkey>,
    pub is_whitelist: bool,
}

pub fn whitelist_account(_args: &mut WhitelistArgs, accounts: Pubkey) -> ProgramResult {
    _args.accounts.push(accounts);
    _args.is_whitelist = true;
    msg!("Account has bee added to the whitelist");
    Ok(())
}
