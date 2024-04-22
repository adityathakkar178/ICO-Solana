const {
    Connection,
    Keypair,
    SystemProgram,
    SYSVAR_RENT_PUBKEY,
    TransactionInstruction,
    sendAndConfirmTransaction,
    Transaction,
} = require('@solana/web3.js');
const {
    TOKEN_PROGRAM_ID,
} = require('@solana/spl-token');
const borsh = require("borsh");
const { Buffer } = require("buffer");

function createKeypairFromFile(path) {
    return Keypair.fromSecretKey(
        Buffer.from(JSON.parse(require('fs').readFileSync(path, "utf-8")))
    );
}

class Assignable {
    constructor(properties) {
        Object.keys(properties).map((key) => {
            return (this[key] = properties[key]);
        });
    }
}

class CreateTokenArgs extends Assignable {
    toBuffer() {
        return Buffer.from(borsh.serialize(CreateTokenArgsSchema, this));
    }
}
const CreateTokenArgsSchema = new Map([
    [
        CreateTokenArgs, {
            kind: 'struct',
            fields: [
                ['token_decimals', 'u8'],
            ]
        }
    ]
]);

describe("Create Tokens!",  () => {

    const connection = new Connection('https://api.devnet.solana.com', 'confirmed');
    const payer = createKeypairFromFile(require('os').homedir() + '/.config/solana/id.json');
    const program = createKeypairFromFile('./ico/target/deploy/ico-keypair.json');

    it("Mint token!",  async () => {

        const mintKeypair = Keypair.generate();

        const instructionData = new CreateTokenArgs({
            token_decimals: 9,
        });

        let ix = new TransactionInstruction({
            keys: [
                { pubkey: mintKeypair.publicKey, isSigner: true, isWritable: true },           
                { pubkey: payer.publicKey, isSigner: false, isWritable: true },                
                { pubkey: payer.publicKey, isSigner: true, isWritable: true }, 
                { pubkey: SYSVAR_RENT_PUBKEY, isSigner: false, isWritable: false },                 
                { pubkey: SystemProgram.programId, isSigner: false, isWritable: false },        
                { pubkey: TOKEN_PROGRAM_ID, isSigner: false, isWritable: false },               
            ],
            programId: program.publicKey,
            data: instructionData.toBuffer(),
        });

        const ts = new Transaction().add(ix);

        const sx = await sendAndConfirmTransaction(
            connection,
            ts,
            [payer, mintKeypair]
        );

        console.log("Success!");
        console.log(`Mint Address: ${mintKeypair.publicKey}`);
        console.log(ix);
        console.log(ts);
        console.log(sx);
    });

});

