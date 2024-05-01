use {
    borsh::{BorshDeserialize, BorshSerialize},
    merkletreers::{tree::MerkleTree, utils::hash_it},
    solana_program::{entrypoint::ProgramResult, msg, program_error::ProgramError},
};

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct WhitelistArgs {
    pub accounts: Vec<String>,
}

const ADMIN_ACCOUNT: &str = "Cv66dQwcSJDdXNVHybchYqp73d75Y8XWj6pKcjN3ffAy";

pub fn whitelist_account(args: WhitelistArgs, admin_account: String) -> ProgramResult {
    if admin_account != ADMIN_ACCOUNT {
        return Err(ProgramError::IllegalOwner);
    }

    let leaves: Vec<[u8; 32]> = args
        .accounts
        .iter()
        .map(|account_str| {
            let account_bytes = account_str.as_bytes();
            let mut buffer = [0u8; 32];
            hash_it(account_bytes, &mut buffer);
            buffer
        })
        .collect();

    let whitelist_tree = MerkleTree::new(leaves);

    let root = whitelist_tree.root;

    msg!("Accounts have been added to the whitelist");
    msg!("{:?}", root);

    let mut leaf = [0u8; 32];
    hash_it(args.accounts[0].as_bytes(), &mut leaf);
    let proof = whitelist_tree.make_proof(leaf);
    msg!("Proof: {:?}", proof);

    let result = whitelist_tree.check_proof(proof, leaf);
    msg!("result: {:?}", result);

    Ok(())
}
