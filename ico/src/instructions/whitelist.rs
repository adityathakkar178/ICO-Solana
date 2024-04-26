use {
    borsh::{BorshDeserialize, BorshSerialize},
    merkletreers::tree::MerkleTree,
    merkletreers::utils::hash_it,
    solana_program::{entrypoint::ProgramResult, msg, pubkey::Pubkey},
};

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct WhitelistArgs {
    pub accounts: Vec<Pubkey>,
}

pub struct WhiteListTree {
    pub tree: MerkleTree,
}

pub fn whitelist_account(_args: &mut WhitelistArgs) -> ProgramResult {
    let leaves: Vec<[u8; 32]> = _args
        .accounts
        .iter()
        .map(|data| {
            let pubkey_bytes = data.to_bytes();
            let mut buffer = [0u8; 32];
            hash_it(&pubkey_bytes[..], &mut buffer);
            buffer
        })
        .collect();

    let whitelist_tree = MerkleTree::new(leaves);
    
    WhiteListTree {
        tree: whitelist_tree,
    };

    msg!("Accounts have been added to the whitelist");

    Ok(())
}
