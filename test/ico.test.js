const {
    Connection,
    Keypair,
    PublicKey,
    SystemProgram,
    SYSVAR_RENT_PUBKEY,
    TransactionInstruction,
    Transaction,
    sendAndConfirmTransaction,
} = require('@solana/web3.js');
const {
    ASSOCIATED_TOKEN_PROGRAM_ID,
    getAssociatedTokenAddress,
    TOKEN_PROGRAM_ID,
} = require('@solana/spl-token');
const { Buffer } = require('buffer');
const {
    CreateTokenArgs,
    MintSplArgs,
    TransferTokensArgs,
    MyInstruction,
} = require('./instruction');
const { BN } = require('bn.js');
const fs = require('fs');

function createKeypairFromFile(path) {
    return Keypair.fromSecretKey(
        Buffer.from(JSON.parse(fs.readFileSync(path, 'utf-8')))
    );
}

describe('Transferring Tokens', () => {
    const connection = new Connection(
        `https://api.devnet.solana.com/`,
        'confirmed'
    );
    const payer = createKeypairFromFile(
        require('os').homedir() + '/.config/solana/id.json'
    );
    const program = createKeypairFromFile(
        './ico/target/deploy/ico-keypair.json'
    );

    const tokenMintKeypair = Keypair.generate();

    const recipientWallet = Keypair.generate();

    it('Create an SPL Token!', async () => {
        const instructionData = new CreateTokenArgs({
            instruction: MyInstruction.Create,
            decimals: 9,
        });

        let ix = new TransactionInstruction({
            keys: [
                {
                    pubkey: tokenMintKeypair.publicKey,
                    isSigner: true,
                    isWritable: true,
                },
                { pubkey: payer.publicKey, isSigner: false, isWritable: true },
                { pubkey: payer.publicKey, isSigner: true, isWritable: true },
                {
                    pubkey: SYSVAR_RENT_PUBKEY,
                    isSigner: false,
                    isWritable: false,
                },
                {
                    pubkey: SystemProgram.programId,
                    isSigner: false,
                    isWritable: false,
                },
                {
                    pubkey: TOKEN_PROGRAM_ID,
                    isSigner: false,
                    isWritable: false,
                },
            ],
            programId: program.publicKey,
            data: instructionData.toBuffer(),
        });
        const ts = new Transaction().add(ix);
        const sx = await sendAndConfirmTransaction(connection, ts, [
            payer,
            tokenMintKeypair,
        ]);

        console.log('Success!');
        console.log(`Mint Address: ${tokenMintKeypair.publicKey}`);
        console.log(`Tx Signature: ${sx}`);
    });

    it('Mint some tokens to your wallet!', async () => {
        const associatedTokenAccountAddress = await getAssociatedTokenAddress(
            tokenMintKeypair.publicKey,
            payer.publicKey
        );

        const instructionData = new MintSplArgs({
            instruction: MyInstruction.MintSpl,
            quantity: new BN(150),
        });

        let ix = new TransactionInstruction({
            keys: [
                {
                    pubkey: tokenMintKeypair.publicKey,
                    isSigner: false,
                    isWritable: true,
                },
                { pubkey: payer.publicKey, isSigner: false, isWritable: true },
                {
                    pubkey: associatedTokenAccountAddress,
                    isSigner: false,
                    isWritable: true,
                },
                { pubkey: payer.publicKey, isSigner: true, isWritable: true },
                {
                    pubkey: SystemProgram.programId,
                    isSigner: false,
                    isWritable: true,
                },
                {
                    pubkey: TOKEN_PROGRAM_ID,
                    isSigner: false,
                    isWritable: false,
                },
                {
                    pubkey: ASSOCIATED_TOKEN_PROGRAM_ID,
                    isSigner: false,
                    isWritable: false,
                },
            ],
            programId: program.publicKey,
            data: instructionData.toBuffer(),
        });
        const ts = new Transaction().add(ix);

        const sx = await sendAndConfirmTransaction(connection, ts, [payer]);

        console.log('Success!');
        console.log(`ATA Address: ${associatedTokenAccountAddress}`);
        console.log(`Tx Signature: ${sx}`);
    });

    // it("Prep a new test wallet for transfers", async () => {

    //     await connection.confirmTransaction(
    //         await connection.requestAirdrop(
    //             recipientWallet.publicKey,
    //             await connection.getMinimumBalanceForRentExemption(0),
    //         )
    //     );
    //     console.log(`Recipient Pubkey: ${recipientWallet.publicKey}`);
    // });

    it('Transfer tokens to another wallet!', async () => {
        const fromAssociatedTokenAddress = await getAssociatedTokenAddress(
            tokenMintKeypair.publicKey,
            payer.publicKey
        );
        console.log(`Owner Token Address: ${fromAssociatedTokenAddress}`);
        const toAssociatedTokenAddress = await getAssociatedTokenAddress(
            tokenMintKeypair.publicKey,
            recipientWallet.publicKey
        );
        console.log(`Recipient Token Address: ${toAssociatedTokenAddress}`);

        const transferToInstructionData = new TransferTokensArgs({
            instruction: MyInstruction.TransferTokens,
            quantity: new BN(15),
        });

        let ix = new TransactionInstruction({
            keys: [
                {
                    pubkey: tokenMintKeypair.publicKey,
                    isSigner: false,
                    isWritable: true,
                },
                {
                    pubkey: fromAssociatedTokenAddress,
                    isSigner: false,
                    isWritable: true,
                },
                {
                    pubkey: toAssociatedTokenAddress,
                    isSigner: false,
                    isWritable: true,
                },
                { pubkey: payer.publicKey, isSigner: true, isWritable: true },
                {
                    pubkey: recipientWallet.publicKey,
                    isSigner: true,
                    isWritable: true,
                },
                { pubkey: payer.publicKey, isSigner: true, isWritable: true },
                {
                    pubkey: SystemProgram.programId,
                    isSigner: false,
                    isWritable: false,
                },
                {
                    pubkey: TOKEN_PROGRAM_ID,
                    isSigner: false,
                    isWritable: false,
                },
                {
                    pubkey: ASSOCIATED_TOKEN_PROGRAM_ID,
                    isSigner: false,
                    isWritable: false,
                },
            ],
            programId: program.publicKey,
            data: transferToInstructionData.toBuffer(),
        });
        const ts = new Transaction().add(ix);
        await sendAndConfirmTransaction(
            connection,
            ts,
            [payer, recipientWallet],
            { skipPreflight: true }
        );
    });
});
