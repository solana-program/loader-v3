import { fetchEncodedAccount, generateKeyPairSigner } from '@solana/kit';
import { expect, it } from 'vitest';

import { createTestClient, deployProgram } from '../_setup';

it('can close a program', async () => {
    // Given a deployed program whose upgrade authority is the payer.
    const client = await createTestClient();
    const { program, programData } = await deployProgram(client);
    const { address: recipient } = await generateKeyPairSigner();
    const { value: programDataLamports } = await client.rpc.getBalance(programData).send();

    // When we close its ProgramData account.
    await client.sendTransaction([
        client.loaderV3.instructions.close({
            bufferOrProgramDataAccount: programData,
            destinationAccount: recipient,
            authority: client.payer,
            programAccount: program,
        }),
    ]);

    // Then the ProgramData account is gone and its lamports went to the recipient.
    const programDataAccount = await fetchEncodedAccount(client.rpc, programData);
    expect(programDataAccount.exists).toBe(false);
    const { value: recipientLamports } = await client.rpc.getBalance(recipient).send();
    expect(recipientLamports).toBe(programDataLamports);
});
