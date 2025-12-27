import { getToolchainArgument } from './scripts/utils.mjs';
import { assertIsNode, numberTypeNode } from 'codama';

export default {
    idl: 'program/idl.json',
    before: [
        {
            from: 'codama#updateProgramsVisitor',
            args: [
                {
                    solanaLoaderV3Program: {
                        name: 'loaderV3',
                        publicKey: 'BPFLoaderUpgradeab1e11111111111111111111111',
                    },
                },
            ],
        },
        {
            from: 'codama#bottomUpTransformerVisitor',
            args: [
                [
                    {
                        select: '[instructionNode]write.[instructionArgumentNode]bytes',
                        transform: node => {
                            assertIsNode(node, 'instructionArgumentNode');
                            assertIsNode(node.type, 'sizePrefixTypeNode');
                            return { ...node, type: { ...node.type, prefix: numberTypeNode('u64') } };
                        },
                    },
                ],
            ],
        },
    ],
    scripts: {
        js: {
            from: '@codama/renderers-js',
            args: ['clients/js/src/generated', { packageFolder: 'clients/js', syncPackageJson: true }],
        },
        rust: {
            from: '@codama/renderers-rust',
            args: [
                'clients/rust/src/generated',
                {
                    crateFolder: 'clients/rust',
                    formatCode: true,
                    toolchain: getToolchainArgument('format'),
                },
            ],
        },
    },
};
