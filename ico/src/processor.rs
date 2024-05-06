use {
    borsh::{BorshDeserialize, BorshSerialize},
    solana_program::{account_info::AccountInfo, entrypoint::ProgramResult, pubkey::Pubkey},
};

use crate::instructions::{
    create::{create_token, CreateTokenArgs},
    mint::{mint_token, MintSplArgs},
    transfer::{transfer_tokens, TransferTokensArgs},
    whitelist::{whitelist_account, WhitelistArgs},
    // presale::{pre_sale, PreSaleArgs, BuyerArgs},
};

#[derive(BorshSerialize, BorshDeserialize, Debug)]
enum MyInstruction {
    Create(CreateTokenArgs),
    MintSpl(MintSplArgs),
    TransferTokens(TransferTokensArgs),
    WhiteListAccount(WhitelistArgs, String),
    // PreSale(Tree),
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
                Ok(_) => Ok(()), // Return Ok(()) if whitelisting succeeds
                Err(err) => Err(err), // Return the error if whitelisting fails
            }
        }
        // MyInstruction::PreSale(args, whitelist_args, buyers_args) => pre_sale(accounts, args, whitelist_args, buyers_args),
    }
}
