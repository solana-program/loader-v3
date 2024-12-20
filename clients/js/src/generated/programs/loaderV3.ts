/**
 * This code was AUTOGENERATED using the kinobi library.
 * Please DO NOT EDIT THIS FILE, instead use visitors
 * to add features, then rerun kinobi to update it.
 *
 * @see https://github.com/kinobi-so/kinobi
 */

import {
  containsBytes,
  getU32Encoder,
  type Address,
  type ReadonlyUint8Array,
} from '@solana/web3.js';
import {
  type ParsedCloseInstruction,
  type ParsedDeployWithMaxDataLenInstruction,
  type ParsedExtendProgramInstruction,
  type ParsedInitializeBufferInstruction,
  type ParsedSetAuthorityCheckedInstruction,
  type ParsedSetAuthorityInstruction,
  type ParsedUpgradeInstruction,
  type ParsedWriteInstruction,
} from '../instructions';

export const LOADER_V3_PROGRAM_ADDRESS =
  'BPFLoaderUpgradeab1e11111111111111111111111' as Address<'BPFLoaderUpgradeab1e11111111111111111111111'>;

export enum LoaderV3Instruction {
  InitializeBuffer,
  Write,
  DeployWithMaxDataLen,
  Upgrade,
  SetAuthority,
  Close,
  ExtendProgram,
  SetAuthorityChecked,
}

export function identifyLoaderV3Instruction(
  instruction: { data: ReadonlyUint8Array } | ReadonlyUint8Array
): LoaderV3Instruction {
  const data = 'data' in instruction ? instruction.data : instruction;
  if (containsBytes(data, getU32Encoder().encode(0), 0)) {
    return LoaderV3Instruction.InitializeBuffer;
  }
  if (containsBytes(data, getU32Encoder().encode(1), 0)) {
    return LoaderV3Instruction.Write;
  }
  if (containsBytes(data, getU32Encoder().encode(2), 0)) {
    return LoaderV3Instruction.DeployWithMaxDataLen;
  }
  if (containsBytes(data, getU32Encoder().encode(3), 0)) {
    return LoaderV3Instruction.Upgrade;
  }
  if (containsBytes(data, getU32Encoder().encode(4), 0)) {
    return LoaderV3Instruction.SetAuthority;
  }
  if (containsBytes(data, getU32Encoder().encode(5), 0)) {
    return LoaderV3Instruction.Close;
  }
  if (containsBytes(data, getU32Encoder().encode(6), 0)) {
    return LoaderV3Instruction.ExtendProgram;
  }
  if (containsBytes(data, getU32Encoder().encode(7), 0)) {
    return LoaderV3Instruction.SetAuthorityChecked;
  }
  throw new Error(
    'The provided instruction could not be identified as a loaderV3 instruction.'
  );
}

export type ParsedLoaderV3Instruction<
  TProgram extends string = 'BPFLoaderUpgradeab1e11111111111111111111111',
> =
  | ({
      instructionType: LoaderV3Instruction.InitializeBuffer;
    } & ParsedInitializeBufferInstruction<TProgram>)
  | ({
      instructionType: LoaderV3Instruction.Write;
    } & ParsedWriteInstruction<TProgram>)
  | ({
      instructionType: LoaderV3Instruction.DeployWithMaxDataLen;
    } & ParsedDeployWithMaxDataLenInstruction<TProgram>)
  | ({
      instructionType: LoaderV3Instruction.Upgrade;
    } & ParsedUpgradeInstruction<TProgram>)
  | ({
      instructionType: LoaderV3Instruction.SetAuthority;
    } & ParsedSetAuthorityInstruction<TProgram>)
  | ({
      instructionType: LoaderV3Instruction.Close;
    } & ParsedCloseInstruction<TProgram>)
  | ({
      instructionType: LoaderV3Instruction.ExtendProgram;
    } & ParsedExtendProgramInstruction<TProgram>)
  | ({
      instructionType: LoaderV3Instruction.SetAuthorityChecked;
    } & ParsedSetAuthorityCheckedInstruction<TProgram>);
