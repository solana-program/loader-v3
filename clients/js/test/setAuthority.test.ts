import { assertAccountExists, fetchEncodedAccount } from '@solana/kit';
import { expect, it } from 'vitest';

import { createTestClient, deployProgram } from '../_setup';

it('can remove the upgrade authority of a program', async () => {
    // Given a deployed program whose upgrade authority is the payer.
    const client = await createTestClient();
    const { programData } = await deployProgram(client);

    // When we set its authority without providing a new one.
    await client.sendTransaction([
        client.loaderV3.instructions.setAuthority({
            bufferOrProgramDataAccount: programData,
            currentAuthority: client.payer,
        }),
    ]);

    // Then the program no longer has an upgrade authority.
    const programDataAccount = await fetchEncodedAccount(client.rpc, programData);
    assertAccountExists(programDataAccount);
    // [12] Upgrade authority option (0 = None).
    expect(programDataAccount.data[12]).toBe(0);
});
