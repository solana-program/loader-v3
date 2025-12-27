import { assertAccountExists, fetchEncodedAccount, generateKeyPairSigner } from '@solana/kit';
import { expect, it } from 'vitest';
import {
    BUFFER_HEADER_SIZE,
    createDefaultSolanaClient,
    createDefaultTransactionMessage,
    generateKeyPairSignerWithSol,
    getCreateBufferInstructions,
    signAndSendTransaction,
} from '../_setup';
import { getWriteInstruction } from '../src';

it('can write to a buffer account', async () => {
    const client = createDefaultSolanaClient();
    const [payer, buffer] = await Promise.all([generateKeyPairSignerWithSol(client), generateKeyPairSigner()]);

    const createBufferInstructions = await getCreateBufferInstructions(client, { payer, buffer, dataLength: 10 });
    const transactionMessage = await createDefaultTransactionMessage(client, payer, [
        ...createBufferInstructions,
        getWriteInstruction({
            bufferAccount: buffer.address,
            bufferAuthority: payer,
            offset: 3,
            bytes: new Uint8Array([0xff, 0xff, 0xff, 0xff]),
        }),
    ]);
    await signAndSendTransaction(client, transactionMessage);

    const bufferAccount = await fetchEncodedAccount(client.rpc, buffer.address);
    assertAccountExists(bufferAccount);
    expect(bufferAccount.data.slice(Number(BUFFER_HEADER_SIZE))).toStrictEqual(
        new Uint8Array([0, 0, 0, 255, 255, 255, 255, 0, 0, 0]),
    );
});
