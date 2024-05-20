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
    WhiteListArgs,
    MyInstruction,
    PreSaleArgs,
    SaleArgs,
} = require('./instruction');
const { BN } = require('bn.js');
const fs = require('fs');

function createKeypairFromFile(path) {
    return Keypair.fromSecretKey(
        Buffer.from(JSON.parse(fs.readFileSync(path, 'utf-8')))
    );
}

describe('Intitial Coin Offering!', () => {
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

    const whiteListAccount1 = Keypair.generate();

    const whiteListAccount2 = Keypair.generate();

    const buyer = Keypair.generate();

    it('Create a Token!', async () => {
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

    it('Mint tokens!', async () => {
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

    it('Transfer tokens!', async () => {
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

        const sx = await sendAndConfirmTransaction(
            connection,
            ts,
            [payer, recipientWallet],
            { skipPreflight: true }
        );

        console.log(`Tx Signature: ${sx}`);
    });

    it('Add To Whitelist!', async () => {
        const instructionData = new WhiteListArgs({
            instruction: MyInstruction.WhiteList,
            accounts: [
                '2v2jcNq1NDLWMq3JZR2YnJDnLGW8aZGv4K6RQjT6ZB61',
                'ED7fL3bnAXLSDup5fVHxv8dKELLpEw5hCaA1yE1FBwfV',
            ],
            admin_account: payer.publicKey.toString(),
        });

        let ix = new TransactionInstruction({
            keys: [
                {
                    pubkey: whiteListAccount1.publicKey,
                    isSigner: true,
                    isWritable: true,
                },
                {
                    pubkey: whiteListAccount2.publicKey,
                    isSigner: true,
                    isWritable: true,
                },
            ],
            programId: program.publicKey,
            data: instructionData.toBuffer(),
        });

        const ts = new Transaction().add(ix);

        const sx = await sendAndConfirmTransaction(connection, ts, [
            payer,
            whiteListAccount1,
            whiteListAccount2,
        ]);

        console.log(`Tx Signature: ${sx}`);
    });

    it('Pre sale', async () => {
        const instructionData = new PreSaleArgs({
            instruction: MyInstruction.PreSale,
            proof: [
                [
                    137, 175, 121, 40, 189, 184, 36, 44, 158, 159, 97, 140, 165,
                    89, 136, 246, 100, 181, 18, 236, 163, 184, 120, 134, 171,
                    200, 22, 138, 48, 240, 106, 62,
                ],
            ],
            root: [
                232, 71, 144, 69, 52, 247, 125, 54, 56, 49, 68, 22, 86, 122,
                162, 109, 246, 62, 87, 131, 204, 40, 211, 129, 31, 139, 4, 213,
                191, 242, 129, 97,
            ],
            pre_sale_price: new BN(5),
            pre_sale_limit: new BN(100),
            pre_sale_start_time: new BN(Date.now() / 1000),
            pre_sale_end_time: new BN(Date.now() / 1000 + 3600),
            quantity: new BN(100),
            buy_quantity: new BN(5),
        });

        console.log(instructionData);

        let ix = new TransactionInstruction({
            keys: [
                {
                    pubkey: payer.publicKey,
                    isSigner: true,
                    isWritable: true,
                },
                {
                    pubkey: buyer.publicKey,
                    isSigner: true,
                    isWritable: true,
                },
                { pubkey: payer.publicKey, isSigner: true, isWritable: true },
                {
                    pubkey: recipientWallet.publicKey,
                    isSigner: true,
                    isWritable: true,
                },
                {
                    pubkey: TOKEN_PROGRAM_ID.programId,
                    isSigner: false,
                    isWritable: false,
                },
            ],
            programId: program.publicKey,
            data: instructionData.toBuffer(),
        });

        console.log(ix);

        const ts = new Transaction().add(ix);
        console.log(ts);
        const sx = await sendAndConfirmTransaction(
            connection,
            ts,
            [payer, buyer, recipientWallet],
            { skipPreflight: true }
        ).catch((err) => console.error('err', err));

        console.log(`Tx Signature: ${sx}`);
    });

    it('Sale', async () => {
        const instructionData = new SaleArgs({
            instruction: MyInstruction.Sale,
            pre_sale_price: new BN(5),
            pre_sale_limit: new BN(100),
            pre_sale_start_time: new BN(Date.now() / 1000 + 3600),
            pre_sale_end_time: new BN(Date.now() / 1000 + 7200),
            quantity: new BN(100),
            buy_quantity: new BN(5),
        });

        let ix = new TransactionInstruction({
            keys: [
                {
                    pubkey: payer.publicKey,
                    isSigner: true,
                    isWritable: true,
                },
                {
                    pubkey: buyer.publicKey,
                    isSigner: true,
                    isWritable: true,
                },
                { pubkey: payer.publicKey, isSigner: true, isWritable: true },
                {
                    pubkey: recipientWallet.publicKey,
                    isSigner: true,
                    isWritable: true,
                },
                {
                    pubkey: TOKEN_PROGRAM_ID.programId,
                    isSigner: false,
                    isWritable: false,
                },
            ],
            programId: program.publicKey,
            data: instructionData.toBuffer(),
        });

        const ts = new Transaction().add(ix);

        const sx = await sendAndConfirmTransaction(
            connection,
            ts,
            [payer, buyer, recipientWallet],
            { skipPreflight: true }
        ).catch((err) => console.error('err', err));

        console.log(`Tx Signature: ${sx}`);
    });
});

jest.setTimeout(3000);
