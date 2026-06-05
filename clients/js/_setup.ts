import { systemProgram } from '@solana-program/system';
import { Instruction, TransactionSigner, createClient, lamports } from '@solana/kit';
import { litesvm } from '@solana/kit-plugin-litesvm';
import { airdropSigner, generatedSigner } from '@solana/kit-plugin-signer';

import { LOADER_V3_PROGRAM_ADDRESS, loaderV3Program } from './src';

export const BUFFER_HEADER_SIZE = 37n;

// The loader-v3 program (`BPFLoaderUpgradeable`) is a native runtime program
// that is available in LiteSVM as a builtin, so no `.so` needs to be loaded.
export const createTestClient = () => {
    return createClient()
        .use(generatedSigner())
        .use(litesvm())
        .use(airdropSigner(lamports(1_000_000_000n)))
        .use(systemProgram())
        .use(loaderV3Program());
};

export type TestClient = Awaited<ReturnType<typeof createTestClient>>;

export const getCreateBufferInstructions = async (
    client: TestClient,
    input: {
        payer: TransactionSigner;
        buffer: TransactionSigner;
        dataLength: number | bigint;
    },
): Promise<Instruction[]> => {
    const bufferSize = BUFFER_HEADER_SIZE + BigInt(input.dataLength);
    const bufferLamports = await client.rpc.getMinimumBalanceForRentExemption(bufferSize).send();
    return [
        client.system.instructions.createAccount({
            payer: input.payer,
            newAccount: input.buffer,
            lamports: bufferLamports,
            space: bufferSize,
            programAddress: LOADER_V3_PROGRAM_ADDRESS,
        }),
        client.loaderV3.instructions.initializeBuffer({
            sourceAccount: input.buffer.address,
            bufferAuthority: input.payer.address,
        }),
    ];
};
