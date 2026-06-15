import { assertAccountExists, fetchEncodedAccount, generateKeyPairSigner } from '@solana/kit';
import { expect, it } from 'vitest';

import { BUFFER_HEADER_SIZE, createTestClient, getCreateBufferInstructions } from '../_setup';

it('can write to a buffer account', async () => {
    const client = await createTestClient();
    const buffer = await generateKeyPairSigner();

    const createBufferInstructions = await getCreateBufferInstructions(client, {
        payer: client.payer,
        buffer,
        dataLength: 10,
    });
    await client.sendTransaction([
        ...createBufferInstructions,
        client.loaderV3.instructions.write({
            bufferAccount: buffer.address,
            bufferAuthority: client.payer,
            offset: 3,
            bytes: new Uint8Array([0xff, 0xff, 0xff, 0xff]),
        }),
    ]);

    const bufferAccount = await fetchEncodedAccount(client.rpc, buffer.address);
    assertAccountExists(bufferAccount);
    expect(bufferAccount.data.slice(Number(BUFFER_HEADER_SIZE))).toStrictEqual(
        new Uint8Array([0, 0, 0, 255, 255, 255, 255, 0, 0, 0]),
    );
});
