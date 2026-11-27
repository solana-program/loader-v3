import { readFileSync } from 'node:fs';

import { systemProgram } from '@solana-program/system';
import {
    Address,
    Instruction,
    TransactionSigner,
    createClient,
    generateKeyPairSigner,
    getAddressEncoder,
    getProgramDerivedAddress,
    lamports,
} from '@solana/kit';
import { litesvm } from '@solana/kit-plugin-litesvm';
import { airdropSigner, generatedSigner } from '@solana/kit-plugin-signer';

import { LOADER_V3_PROGRAM_ADDRESS, loaderV3Program } from './src';

export const BUFFER_HEADER_SIZE = 37n;
export const PROGRAM_ACCOUNT_SIZE = 36n;

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

// Deploys a small SBPFv3 program with the client payer as its upgrade authority,
// then moves to the next slot since a program can't be closed in its deployment slot.
// The ELF is agave's `programs/bpf_loader/test_elfs/out/sbpfv3_return_ok.so`.
export const deployProgram = async (client: TestClient): Promise<{ program: Address; programData: Address }> => {
    const elf = new Uint8Array(readFileSync(new URL('./test/fixtures/sbpfv3_return_ok.so', import.meta.url)));
    const buffer = await generateKeyPairSigner();
    const program = await generateKeyPairSigner();
    const [programData] = await getProgramDerivedAddress({
        programAddress: LOADER_V3_PROGRAM_ADDRESS,
        seeds: [getAddressEncoder().encode(program.address)],
    });

    await client.sendTransaction([
        ...(await getCreateBufferInstructions(client, { payer: client.payer, buffer, dataLength: elf.length })),
        client.loaderV3.instructions.write({
            bufferAccount: buffer.address,
            bufferAuthority: client.payer,
            offset: 0,
            bytes: elf,
        }),
    ]);

    const programLamports = await client.rpc.getMinimumBalanceForRentExemption(PROGRAM_ACCOUNT_SIZE).send();
    await client.sendTransaction([
        client.system.instructions.createAccount({
            payer: client.payer,
            newAccount: program,
            lamports: programLamports,
            space: PROGRAM_ACCOUNT_SIZE,
            programAddress: LOADER_V3_PROGRAM_ADDRESS,
        }),
        client.loaderV3.instructions.deployWithMaxDataLen({
            payerAccount: client.payer,
            programDataAccount: programData,
            programAccount: program.address,
            bufferAccount: buffer.address,
            authority: client.payer,
            maxDataLen: elf.length,
        }),
    ]);

    client.svm.warpToSlot(client.svm.getClock().slot + 1n);
    return { program: program.address, programData };
};
