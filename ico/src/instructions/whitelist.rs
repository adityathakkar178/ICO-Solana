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

pub struct Tree {
    pub merkle_tree: MerkleTree,
}

pub(crate) static mut WHITELIST_TREE: Option<Tree> = None;

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

    let tree = Tree {
        merkle_tree: whitelist_tree,
    };

    msg!("Accounts have been added to the whitelist");
    msg!("{:?}", tree.merkle_tree.root);

    unsafe {
        WHITELIST_TREE = Some(tree);
    }

    Ok(())
}

pub fn acc() {
    
}