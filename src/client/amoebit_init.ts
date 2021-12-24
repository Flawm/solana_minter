import {
    Account,
    Keypair,
    Connection,
    PublicKey,
    LAMPORTS_PER_SOL,
    SystemProgram,
    TransactionInstruction,
    Transaction,
    sendAndConfirmTransaction,
} from '@solana/web3.js';
import { MintLayout, AccountLayout, Token, TOKEN_PROGRAM_ID } from '@solana/spl-token';
import fs from 'mz/fs';
import path from 'path';
import * as borsh from 'borsh';

import {getPayer, getRpcUrl, createKeypairFromFile} from './utils';

/**
 * Connection to the network
 */
let connection: Connection;

/**
 * Keypair associated to the fees' payer
 */
let payer: Keypair;

/**
 * Hello world's program id
 */
let programId: PublicKey;

/**
 * The public key of the account tracking the mint index
 */
let indexPubkey: PublicKey;

let indexAccount;

/**
 * Path to program files
 */
const PROGRAM_PATH = path.resolve(__dirname, '../../dist/program');

/**
 * Path to program shared object file which should be deployed on chain.
 * This file is created when running either:
 *     - `npm run build:program-c`
 *     - `npm run build:program-rust`
 */
const PROGRAM_SO_PATH = path.join(PROGRAM_PATH, 'test.so');

const PROGRAM_KEYPAIR_PATH = path.join(PROGRAM_PATH, 'amoebit_minter-keypair.json');

class AmoebitIndexAccount {
    counter = 0;
    constructor(fields: {counter: number} | undefined = undefined) {
        if (fields) {
            this.counter = fields.counter;
        }
    }
}

const AmoebitIndexSchema = new Map([
    [AmoebitIndexAccount, {kind: 'struct', fields: [['counter', 'u16']]}],
]);

const INDEX_SIZE = borsh.serialize(
    AmoebitIndexSchema,
    new AmoebitIndexAccount(),
).length;

export async function establishConnection(): Promise<void> {
    const rpcUrl = await getRpcUrl();
    connection = new Connection(rpcUrl, 'confirmed');
    const version = await connection.getVersion();
    console.log('Connection to cluster established:', rpcUrl, version);
}

export async function establishPayer(): Promise<void> {
    let fees = 0;
    if (!payer) {
        const {feeCalculator} = await connection.getRecentBlockhash();

        // Calculate the cost to fund the index account
        fees += await connection.getMinimumBalanceForRentExemption(INDEX_SIZE);

        // Calculate the cost of sending transactions
        fees += feeCalculator.lamportsPerSignature * 100; // wag

        payer = await getPayer();
    }

    let lamports = await connection.getBalance(payer.publicKey);
    if (lamports < fees) {
        // If current balance is not enough to pay for fees, request an airdrop
        const sig = await connection.requestAirdrop(
            payer.publicKey,
            fees - lamports,
        );
        await connection.confirmTransaction(sig);
        lamports = await connection.getBalance(payer.publicKey);
    }

    console.log(
        'Using account',
        payer.publicKey.toBase58(),
        'containing',
        lamports / LAMPORTS_PER_SOL,
        'SOL to pay for fees',
    );
}

/**
 * Make sure the accounts for the program are available
 */
export async function checkAccounts(): Promise<void> {
    // Read program id from keypair file
    const programKeypair = await createKeypairFromFile(PROGRAM_KEYPAIR_PATH);
    programId = programKeypair.publicKey;

    // Check if the program has been deployed
    const programInfo = await connection.getAccountInfo(programId);
    if (programInfo === null) {
        if (fs.existsSync(PROGRAM_SO_PATH)) {
            throw new Error(
                'Program needs to be deployed with `solana program deploy dist/program/test.so`',
            );
        } else {
            throw new Error('Program needs to be built and deployed');
        }
    } else if (!programInfo.executable) {
        throw new Error(`Program is not executable`);
    }
    console.log(`Using program ${programId.toBase58()}`);

    // Derive the address (public key) of the account from the program so that it's easy to find later.
    const INDEX_SEED = 'amoebit_minter';
    indexPubkey = await PublicKey.createWithSeed(
        payer.publicKey,
        INDEX_SEED,
        programId,
    );

    // Check if the account has already been created
    indexAccount = await connection.getAccountInfo(indexPubkey);
    if (indexAccount === null) {
        console.log(
            'Creating account',
            indexPubkey.toBase58(),
            'to count mint index',
        );
        const lamports = await connection.getMinimumBalanceForRentExemption(
            INDEX_SIZE,
        );


        const transaction = new Transaction().add(
            SystemProgram.createAccountWithSeed({
                basePubkey: payer.publicKey,
                fromPubkey: payer.publicKey,
                lamports,
                newAccountPubkey: indexPubkey,
                programId: programId,
                seed: INDEX_SEED,
                space: INDEX_SIZE,
            }),
        );
        await sendAndConfirmTransaction(connection, transaction, [payer]);
    }
}

export async function testContract(): Promise<void> {
    let our_wallet     = new PublicKey('AmbtTL5LS42RFL1ZL5QQan8ZSyn27pvVoCbFYF2eTwyH'),
    meta_program       = new PublicKey("metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s"),
    minter_program     = new PublicKey(programId),
    associated_program = new PublicKey("ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL"),
    mint_kp  = Keypair.generate(),
    token_key = (await PublicKey.findProgramAddress(
        [
            payer.publicKey.toBuffer(),
            TOKEN_PROGRAM_ID.toBuffer(),
            mint_kp.publicKey.toBuffer()
        ],
        associated_program
    ))[0],
    buff = [
        Buffer.from('metadata'),
        meta_program.toBuffer(),
        mint_kp.publicKey.toBuffer()
    ],
    meta_key = (await PublicKey.findProgramAddress(
        buff,
        meta_program
    ))[0],
    auth_key = (await PublicKey.findProgramAddress(
        [
            Buffer.from('amoebit_minter'),
            minter_program.toBuffer(),
            Buffer.from('amoebit_minter'),
        ],
        minter_program
    ))[0],
    wl_key = new PublicKey("DwuhyNAQYjJHKZJEkVLy5Phoz83Tty6whVcZ79eQ7rXs"),
    wl_account = (await PublicKey.findProgramAddress(
        [
            payer.publicKey.toBuffer(),
            TOKEN_PROGRAM_ID.toBuffer(),
            wl_key.toBuffer()
        ],
        associated_program
    ))[0],
    discount_key = new PublicKey("Hzn4ehrSbJstaGTjx1MP7K8EGJTSzjFDqZ3yFcWny332"),
    discount_account = (await PublicKey.findProgramAddress(
        [
            payer.publicKey.toBuffer(),
            TOKEN_PROGRAM_ID.toBuffer(),
            discount_key.toBuffer()
        ],
        associated_program
    ))[0],
    sys_key = new PublicKey('11111111111111111111111111111111'),
    rent_key = new PublicKey('SysvarRent111111111111111111111111111111111');

    // accounts
    let account_0 = {pubkey: indexPubkey,       isSigner: false, isWritable: true},
    account_1     = {pubkey: payer.publicKey,   isSigner: true,  isWritable: true},
    account_2     = {pubkey: our_wallet,        isSigner: false, isWritable: true},
    account_3     = {pubkey: sys_key,           isSigner: false, isWritable: false},
    account_4     = {pubkey: token_key,         isSigner: false, isWritable: true},
    account_5     = {pubkey: mint_kp.publicKey, isSigner: false, isWritable: true},
    account_6     = {pubkey: meta_key,          isSigner: false, isWritable: true},
    account_7     = {pubkey: meta_program,      isSigner: false, isWritable: false},
    account_8     = {pubkey: rent_key,          isSigner: false, isWritable: false},
    account_9     = {pubkey: auth_key,          isSigner: false, isWritable: true},
    account_10    = {pubkey: TOKEN_PROGRAM_ID,  isSigner: false, isWritable: false},
    account_11    = {pubkey: wl_key,            isSigner: false, isWritable: true},
    account_12    = {pubkey: wl_account,        isSigner: false, isWritable: true},
    account_13    = {pubkey: discount_key,      isSigner: false, isWritable: true},
    account_14    = {pubkey: discount_account,  isSigner: false, isWritable: true};

    let mintRent = await connection.getMinimumBalanceForRentExemption(MintLayout.span),
    tokenRent = await connection.getMinimumBalanceForRentExemption(AccountLayout.span);

    let mintAccount = SystemProgram.createAccount({
        fromPubkey: payer.publicKey,
        newAccountPubkey: mint_kp.publicKey,
        lamports: mintRent,
        space: MintLayout.span,
        programId: TOKEN_PROGRAM_ID
    }),
    tokenAccount = Token.createAssociatedTokenAccountInstruction(
        associated_program,
        TOKEN_PROGRAM_ID,
        mint_kp.publicKey,
        token_key,
        payer.publicKey,
        payer.publicKey,
    ),
    create_token = Token.createInitMintInstruction(
        TOKEN_PROGRAM_ID,
        mint_kp.publicKey,
        0,
        payer.publicKey,
        null,
    ),
    mint_into_token_account = Token.createMintToInstruction(
        TOKEN_PROGRAM_ID,
        mint_kp.publicKey,
        token_key,
        payer.publicKey,
        [],
        1,
    ),
    instruction = new TransactionInstruction({
        keys: [account_0, account_1, account_2, account_3, account_4, account_5, account_6, account_7, account_8, account_9, account_10, account_11, account_12, account_13, account_14],
        programId,
        data: Buffer.alloc(0)
    });


    let transaction = new Transaction().add(
        mintAccount,
        create_token,
        tokenAccount,
        mint_into_token_account,
        instruction,
    );


    let a = await sendAndConfirmTransaction(
        connection,
        transaction,
        [payer, mint_kp]
      ,{ skipPreflight: true }
    );

    console.log(indexPubkey.toBase58());
    console.log(a);
}

/**
 * confirm index count
 */
export async function readIndexAccount(): Promise<void> {
    const accountInfo = await connection.getAccountInfo(indexPubkey);
    if (accountInfo === null) {
        throw 'Error: index account not created';
    }
    const account_data = borsh.deserialize(
        AmoebitIndexSchema,
        AmoebitIndexAccount,
        accountInfo.data,
    );
    console.log(
        'there have been',
        account_data.counter,
        'mints',
    );
}
