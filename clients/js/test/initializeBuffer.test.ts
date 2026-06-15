import { fetchEncodedAccount, generateKeyPairSigner, getAddressEncoder } from '@solana/kit';
import { expect, it } from 'vitest';

import { BUFFER_HEADER_SIZE, createTestClient } from '../_setup';
import { LOADER_V3_PROGRAM_ADDRESS } from '../src';

it('can initialize a new buffer account', async () => {
    const client = await createTestClient();
    const buffer = await generateKeyPairSigner();
    const space = BUFFER_HEADER_SIZE + 10n;
    const bufferLamports = await client.rpc.getMinimumBalanceForRentExemption(space).send();

    await client.sendTransaction([
        client.system.instructions.createAccount({
            newAccount: buffer,
            lamports: bufferLamports,
            space,
            programAddress: LOADER_V3_PROGRAM_ADDRESS,
        }),
        client.loaderV3.instructions.initializeBuffer({
            sourceAccount: buffer.address,
            bufferAuthority: client.payer.address,
        }),
    ]);

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
            ...getAddressEncoder().encode(client.payer.address),
            // [37-46] Zeroed data.
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ]),
    });
});
