#!/usr/bin/env zx
import 'zx/globals';
import { generateIdl } from '@metaplex-foundation/shank-js';
import { getCargo, getProgramFolders } from './utils.mjs';

const binaryInstallDir = path.join(__dirname, '..', '.cargo');

getProgramFolders().forEach((folder) => {
  const cargo = getCargo(folder);
  const isShank = Object.keys(cargo.dependencies).includes('shank');
  const programDir = path.join(__dirname, '..', folder);

  generateIdl({
    generator: isShank ? 'shank' : 'anchor',
    programName: cargo.package.name.replace(/-/g, '_'),
    programId: cargo.package.metadata.solana['program-id'],
    idlDir: programDir,
    idlHook: (idl) => ({
      ...idl,
      instructions: idl.instructions.map((instruction) => ({
        ...instruction,
        discriminant: {
          ...instruction.discriminant,
          type: "u32", // The legacy native program only accepts 4-byte instruction discriminants.
        },
      })),
    }),
    idlName: 'idl',
    programDir,
    binaryInstallDir,
  });
});
