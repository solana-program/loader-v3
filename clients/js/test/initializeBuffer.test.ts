import { getCreateAccountInstruction } from '@solana-program/system';
import { fetchEncodedAccount, generateKeyPairSigner, getAddressEncoder } from '@solana/kit';
import { expect, it } from 'vitest';
import {
    BUFFER_HEADER_SIZE,
    createDefaultSolanaClient,
    createDefaultTransactionMessage,
    generateKeyPairSignerWithSol,
    signAndSendTransaction,
} from '../_setup';
import { getInitializeBufferInstruction, LOADER_V3_PROGRAM_ADDRESS } from '../src';

it('can initialize a new buffer account', async () => {
    const client = createDefaultSolanaClient();
    const [payer, buffer] = await Promise.all([generateKeyPairSignerWithSol(client), generateKeyPairSigner()]);
    const space = BUFFER_HEADER_SIZE + 10n;
    const bufferLamports = await client.rpc.getMinimumBalanceForRentExemption(space).send();

    const transactionMessage = await createDefaultTransactionMessage(client, payer, [
        getCreateAccountInstruction({
            payer,
            newAccount: buffer,
            lamports: bufferLamports,
            space,
            programAddress: LOADER_V3_PROGRAM_ADDRESS,
        }),
        getInitializeBufferInstruction({
            sourceAccount: buffer.address,
            bufferAuthority: payer.address,
        }),
    ]);
    await signAndSendTransaction(client, transactionMessage);

    const bufferAccount = await fetchEncodedAccount(client.rpc, buffer.address);
    expect(bufferAccount).toMatchObject({
        address: buffer.address,
        space,
        /* prettier-ignore */
        data: new Uint8Array([
            // [0-3] Discriminator.
            1, 0, 0, 0,
            // [4] Authority option.
            1,
            // [5-36] Authority address.
            ...getAddressEncoder().encode(payer.address),
            // [37-46] Zeroed data.
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ]),
    });
});
