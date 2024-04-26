const borsh = require('borsh');

class Assignable {
    constructor(properties) {
        Object.keys(properties).map((key) => {
            return (this[key] = properties[key]);
        });
    }
}

const MyInstruction = {
    Create: 0,
    MintSpl: 1,
    TransferTokens: 2,
    WhiteList: 3,
};

class CreateTokenArgs extends Assignable {
    toBuffer() {
        return Buffer.from(borsh.serialize(CreateTokenArgsSchema, this));
    }
}
const CreateTokenArgsSchema = new Map([
    [
        CreateTokenArgs,
        {
            kind: 'struct',
            fields: [
                ['instruction', 'u8'],
                ['decimals', 'u8'],
            ],
        },
    ],
]);

class MintSplArgs extends Assignable {
    toBuffer() {
        return Buffer.from(borsh.serialize(MintSplArgsSchema, this));
    }
}
const MintSplArgsSchema = new Map([
    [
        MintSplArgs,
        {
            kind: 'struct',
            fields: [
                ['instruction', 'u8'],
                ['quantity', 'u64'],
            ],
        },
    ],
]);

class TransferTokensArgs extends Assignable {
    toBuffer() {
        return Buffer.from(borsh.serialize(TransferTokensArgsSchema, this));
    }
}
const TransferTokensArgsSchema = new Map([
    [
        TransferTokensArgs,
        {
            kind: 'struct',
            fields: [
                ['instruction', 'u8'],
                ['quantity', 'u64'],
            ],
        },
    ],
]);

class WhiteListArgs extends Assignable {
    toBuffer() {
        return Buffer.from(borsh.serialize(WhiteListArgsSchema, this));
    }
}

const WhiteListArgsSchema = new Map([
    [
        WhiteListArgs,
        {
            kind: 'struct',
            fields: [
                ['instruction', 'u8'],
                ['accounts', 'u32'],
            ],
        },
    ],
]);

module.exports = {
    MyInstruction,
    CreateTokenArgs,
    MintSplArgs,
    TransferTokensArgs,
    WhiteListArgs,
};
