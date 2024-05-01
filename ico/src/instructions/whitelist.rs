use borsh::{BorshDeserialize, BorshSerialize};
use merkletreers::tree::MerkleTree;
use merkletreers::utils::hash_it;
use solana_program::{entrypoint::ProgramResult, msg};

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct WhitelistArgs {
    pub accounts: Vec<String>, 
}

pub fn whitelist_account(args: WhitelistArgs) -> ProgramResult {
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
