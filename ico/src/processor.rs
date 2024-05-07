use solana_program::program_error::ProgramError;

use {
    borsh::{BorshDeserialize, BorshSerialize},
    solana_program::{account_info::AccountInfo, entrypoint::ProgramResult, pubkey::Pubkey},
};

use crate::instructions::{
    self, 
    create::{create_token, CreateTokenArgs}, 
    mint::{mint_token, MintSplArgs}, 
    presale::{pre_sale, BuyerArgs, PreSaleArgs}, 
    transfer::{transfer_tokens, TransferTokensArgs}, 
    whitelist::{whitelist_account, WhitelistArgs}
};

#[derive(BorshSerialize, BorshDeserialize, Debug)]
enum MyInstruction {
    Create(CreateTokenArgs),
    MintSpl(MintSplArgs),
    TransferTokens(TransferTokensArgs),
    WhiteListAccount(WhitelistArgs, String),
    PreSale(),
}

pub(crate) static mut WHITELIST_TREE: Option<&instructions::Tree> = None;

impl AsRef<instructions::Tree> for instructions::Tree {
    fn as_ref(&self) -> &instructions::Tree {
        self
    }
}

pub fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    
    let instruction = MyInstruction::try_from_slice(instruction_data)?;

    match instruction {
        MyInstruction::Create(args) => create_token(accounts, args),
        MyInstruction::MintSpl(args) => mint_token(accounts, args),
        MyInstruction::TransferTokens(args) => transfer_tokens(accounts, args),
        MyInstruction::WhiteListAccount(args, admin_account) => {
            match whitelist_account(args, admin_account) {
                Ok(_) => Ok(()), 
                Err(err) => Err(err), 
            }
        }
        MyInstruction::PreSale() => {
            let tree = unsafe { WHITELIST_TREE.unwrap() }; 
            pre_sale(accounts, tree )
        }   
    }
}
