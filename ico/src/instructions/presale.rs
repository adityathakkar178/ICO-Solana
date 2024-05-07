use {
    super::WhitelistArgs,
    crate::instructions::{transfer_tokens, whitelist::Tree, TransferTokensArgs},
    borsh::{BorshDeserialize, BorshSerialize},
    merkletreers::{merkle_proof_check::merkle_proof_check, utils::hash_it,},
    solana_program::{
        account_info::{next_account_info, AccountInfo},
        clock::Clock,
        entrypoint::ProgramResult,
        msg,
        program::invoke,
        program_error::ProgramError,
        system_instruction,
        sysvar::Sysvar,
    },
};

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct PreSaleArgs {
    pub pre_sale_price: u64,
    pub pre_sale_limit: u64,
    pub pre_sale_start_time: u64,
    pub pre_sale_end_time: u64,
    pub quantity: u64,
    pub price: u64,
    pub account: bool,
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct BuyerArgs {
    pub buy_quantity: u64,
}

pub fn pre_sale(
    accounts: &[AccountInfo],

    whitelist_tree: &Tree, 
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();

    let seller_account = next_account_info(accounts_iter)?;
    let buyer_account = next_account_info(accounts_iter)?;
    let system_program = next_account_info(accounts_iter)?;

    // let current_time = Clock::get()?.unix_timestamp as u64;
    // if current_time < args.pre_sale_start_time || current_time > args.pre_sale_end_time {
    //     return Err(ProgramError::InvalidArgument);
    // }

    msg!("Pre sale has started!");

    let root = whitelist_tree.merkle_tree.root; 

    msg!("{:?}", root);

    let buyer = buyer_account.key.to_string();

    let mut leaf = [0u8; 32];
    hash_it(buyer.as_bytes(), &mut leaf);
    let proof = whitelist_tree.merkle_tree.make_proof(leaf);
    msg!("Proof: {:?}", proof);

    let result = merkle_proof_check(proof, leaf);
    msg!("result: {:?}", result);


    // let amount = TransferTokensArgs {
    //     quantity: buyer_args.buy_quantity,
    // };
    // let total_cost = amount.quantity * args.pre_sale_price;

    // let transfer_sol =
    //     system_instruction::transfer(buyer_account.key, seller_account.key, total_cost);
    // invoke(
    //     &transfer_sol,
    //     &[
    //         buyer_account.clone(),
    //         seller_account.clone(),
    //         system_program.clone(),
    //     ],
    // )?;

    // transfer_tokens(accounts, amount)?;

    Ok(())
}
