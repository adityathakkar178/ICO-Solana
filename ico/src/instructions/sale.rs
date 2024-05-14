use {
    super::PreSaleArgs,
    crate::instructions::TransferTokensArgs,
    borsh::{BorshDeserialize, BorshSerialize},
    solana_program::{
        account_info::{next_account_info, AccountInfo},
        clock::Clock,
        entrypoint::ProgramResult,
        msg,
        program::invoke,
        program_error::ProgramError,
        sysvar::Sysvar,
    },
    spl_associated_token_account::instruction as associated_token_account_instruction,
    spl_token::instruction as token_instruction,
};

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct SaleArgs {
    pub sale_price: u64,
    pub sale_limit: u64,
    pub sale_start_time: u64,
    pub sale_end_time: u64,
    pub quantity: u64,
    pub account: bool,
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct BuyerSaleArgs {
    pub buy_quantity: u64,
}

pub fn sale(
    accounts: &[AccountInfo],
    pre_sale_args: PreSaleArgs,
    sale_args: SaleArgs,
    buyer_args: BuyerSaleArgs,
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();

    let mint_account = next_account_info(accounts_iter)?;
    let seller_account = next_account_info(accounts_iter)?;
    let buyer_account = next_account_info(accounts_iter)?;
    let owner = next_account_info(accounts_iter)?;
    let recipient = next_account_info(accounts_iter)?;
    let payer = next_account_info(accounts_iter)?;
    let system_program = next_account_info(accounts_iter)?;
    let token_program = next_account_info(accounts_iter)?;
    let associated_token_program = next_account_info(accounts_iter)?;

    let current_time = Clock::get()?.unix_timestamp as u64;
    if sale_args.sale_start_time > pre_sale_args.pre_sale_start_time
        && current_time < sale_args.sale_end_time
    {
        return Err(ProgramError::InvalidArgument);
    }

    let amount = TransferTokensArgs {
        quantity: buyer_args.buy_quantity,
    };

    let total_cost = amount.quantity * sale_args.sale_price;

    if buyer_account.lamports() == 0 {
        msg!("Creating associated token account for recipient...");
        invoke(
            &associated_token_account_instruction::create_associated_token_account(
                payer.key,
                recipient.key,
                mint_account.key,
                token_program.key,
            ),
            &[
                mint_account.clone(),
                buyer_account.clone(),
                recipient.clone(),
                payer.clone(),
                system_program.clone(),
                token_program.clone(),
                associated_token_program.clone(),
            ],
        )?;
    } else {
        msg!("Associated token account exists.");
    }

    msg!("Recipient Associated Token Address: {}", buyer_account.key);
    msg!("Transferring {} total cost to seller!!!", total_cost);
    invoke(
        &token_instruction::transfer(
            token_program.key,
            seller_account.key,
            buyer_account.key,
            owner.key,
            &[owner.key, recipient.key],
            total_cost,
        )?,
        &[
            seller_account.clone(),
            buyer_account.clone(),
            owner.clone(),
            recipient.clone(),
            token_program.clone(),
        ],
    )?;
    

    msg!("Transferring {} tokens!!!", buyer_args.buy_quantity);
    msg!("Owner Token Address: {}", seller_account.key);
    msg!("Recipient Token Address: {}", buyer_account.key);
    invoke(
        &token_instruction::transfer(
            token_program.key,
            seller_account.key,
            buyer_account.key,
            owner.key,
            &[owner.key, recipient.key],
            buyer_args.buy_quantity,
        )?,
        &[
            seller_account.clone(),
            buyer_account.clone(),
            owner.clone(),
            recipient.clone(),
            token_program.clone(),
        ],
    )?;

    msg!("Tokens transferred successfully.");

    Ok(())
}
