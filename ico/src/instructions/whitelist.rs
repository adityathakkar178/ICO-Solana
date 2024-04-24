use {
    borsh::{BorshDeserialize, BorshSerialize},
    solana_program::{
        account_info::{next_account_info, AccountInfo},
        entrypoint::ProgramResult,
        msg,
        program::invoke,
    },
    spl_associated_token_account::instruction as associated_token_account_instruction,
    spl_token::instruction as token_instruction, std::string,
};

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct WhitelistArgs<'a> {
    pub accounts: Vec<&'a AccountInfo<'a>>,
}