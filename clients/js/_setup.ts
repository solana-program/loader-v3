import { getCreateAccountInstruction } from '@solana-program/system';
import {
    BaseTransactionMessage,
    Commitment,
    Instruction,
    Rpc,
    RpcSubscriptions,
    SolanaRpcApi,
    SolanaRpcSubscriptionsApi,
    TransactionMessageWithBlockhashLifetime,
    TransactionMessageWithFeePayer,
    TransactionSigner,
    airdropFactory,
    appendTransactionMessageInstructions,
    assertIsSendableTransaction,
    assertIsTransactionWithBlockhashLifetime,
    createSolanaRpc,
    createSolanaRpcSubscriptions,
    createTransactionMessage,
    generateKeyPairSigner,
    getSignatureFromTransaction,
    lamports,
    pipe,
    sendAndConfirmTransactionFactory,
    setTransactionMessageFeePayerSigner,
    setTransactionMessageLifetimeUsingBlockhash,
    signTransactionMessageWithSigners,
} from '@solana/kit';
import { getInitializeBufferInstruction, LOADER_V3_PROGRAM_ADDRESS } from './src';

export const BUFFER_HEADER_SIZE = 37n;

type Client = {
    rpc: Rpc<SolanaRpcApi>;
    rpcSubscriptions: RpcSubscriptions<SolanaRpcSubscriptionsApi>;
};

export const createDefaultSolanaClient = (): Client => {
    const rpc = createSolanaRpc('http://127.0.0.1:8899');
    const rpcSubscriptions = createSolanaRpcSubscriptions('ws://127.0.0.1:8900');
    return { rpc, rpcSubscriptions };
};

export const generateKeyPairSignerWithSol = async (client: Client, putativeLamports: bigint = 1_000_000_000n) => {
    const signer = await generateKeyPairSigner();
    await airdropFactory(client)({
        recipientAddress: signer.address,
        lamports: lamports(putativeLamports),
        commitment: 'confirmed',
    });
    return signer;
};

export const createDefaultTransactionMessage = async (
    client: Client,
    feePayer: TransactionSigner,
    instructions?: Instruction[],
) => {
    const { value: latestBlockhash } = await client.rpc.getLatestBlockhash().send();
    return pipe(
        createTransactionMessage({ version: 0 }),
        tx => setTransactionMessageFeePayerSigner(feePayer, tx),
        tx => setTransactionMessageLifetimeUsingBlockhash(latestBlockhash, tx),
        tx => (instructions ? appendTransactionMessageInstructions(instructions, tx) : tx),
    );
};

export const signAndSendTransaction = async (
    client: Client,
    transactionMessage: BaseTransactionMessage &
        TransactionMessageWithFeePayer &
        TransactionMessageWithBlockhashLifetime,
    commitment: Commitment = 'confirmed',
) => {
    const signedTransaction = await signTransactionMessageWithSigners(transactionMessage);
    const signature = getSignatureFromTransaction(signedTransaction);
    assertIsSendableTransaction(signedTransaction);
    assertIsTransactionWithBlockhashLifetime(signedTransaction);
    await sendAndConfirmTransactionFactory(client)(signedTransaction, {
        commitment,
    });
    return signature;
};

export async function getCreateBufferInstructions(
    client: Client,
    input: {
        payer: TransactionSigner;
        buffer: TransactionSigner;
        dataLength: number | bigint;
    },
) {
    const bufferSize = BUFFER_HEADER_SIZE + BigInt(input.dataLength);
    const bufferLamports = await client.rpc.getMinimumBalanceForRentExemption(bufferSize).send();
    return [
        getCreateAccountInstruction({
            payer: input.payer,
            newAccount: input.buffer,
            lamports: bufferLamports,
            space: bufferSize,
            programAddress: LOADER_V3_PROGRAM_ADDRESS,
        }),
        getInitializeBufferInstruction({
            sourceAccount: input.buffer.address,
            bufferAuthority: input.payer.address,
        }),
    ];
}
