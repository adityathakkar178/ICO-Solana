use {
    borsh::{BorshDeserialize, BorshSerialize},
    merkletreers::utils::hash_it,
    solana_program::{
        account_info::{next_account_info, AccountInfo},
        entrypoint::ProgramResult,
        keccak, msg,
    },
};

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
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();

    let buyer_account = next_account_info(accounts_iter)?;
    
    msg!("Pre sale has started!");
    msg!("{:?}", args.proof);
    msg!("{:?}", args.root);

    let buyer = buyer_account.key.to_string();
    let mut leaf = [0u8; 32];
    hash_it(buyer.as_bytes(), &mut leaf);

    let is_whitelist = merkle_verify(args.proof, args.root, leaf);
    msg!("{:?}", is_whitelist);

    Ok(())
}
