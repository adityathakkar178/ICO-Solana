use {
    crate::instructions::{transfer_tokens, TransferTokensArgs},
    borsh::{BorshDeserialize, BorshSerialize},
    merkletreers::utils::hash_it,
    solana_program::{
        account_info::{next_account_info, AccountInfo},
        clock::Clock,
        entrypoint::ProgramResult,
        keccak, msg,
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

fn merkle_verify(proof: Vec<[u8; 32]>, root: [u8; 32], leaf: [u8; 32]) -> bool {
    let mut computed_hash = leaf;
    for proof_element in proof.into_iter() {
        if computed_hash <= proof_element {
            computed_hash = keccak::hashv(&[&computed_hash, &proof_element]).0;
        } else {
            computed_hash = keccak::hashv(&[&proof_element, &computed_hash]).0;
        }
    }
    computed_hash == root
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct Tree {
    proof: Vec<[u8; 32]>,
    root: [u8; 32],
}

pub fn pre_sale(
    accounts: &[AccountInfo],
    args: Tree,
    pre_sale_args: PreSaleArgs,
    buyer_args: BuyerArgs,
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();

    let seller_account = next_account_info(accounts_iter)?;
    let buyer_account = next_account_info(accounts_iter)?;
    let system_program = next_account_info(accounts_iter)?;

    let current_time = Clock::get()?.unix_timestamp as u64;
    if current_time < pre_sale_args.pre_sale_start_time
        && current_time > pre_sale_args.pre_sale_end_time
    {
        return Err(ProgramError::InvalidArgument);
    }

    msg!("Pre sale has started!");
    msg!("{:?}", args.proof);
    msg!("{:?}", args.root);

    let buyer = buyer_account.key.to_string();
    let mut leaf = [0u8; 32];
    hash_it(buyer.as_bytes(), &mut leaf);

    let is_whitelist = merkle_verify(args.proof, args.root, leaf);
    msg!("{:?}", is_whitelist);

    let amount = TransferTokensArgs {
        quantity: buyer_args.buy_quantity,
    };

    let total_cost = amount.quantity * pre_sale_args.pre_sale_price;

    let transfer_sol =
        system_instruction::transfer(buyer_account.key, seller_account.key, total_cost);
    invoke(
        &transfer_sol,
        &[
            buyer_account.clone(),
            seller_account.clone(),
            system_program.clone(),
        ],
    )?;

    transfer_tokens(accounts, amount)?;

    Ok(())
}
