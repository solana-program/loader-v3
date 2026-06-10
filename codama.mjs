import { execSync } from 'node:child_process';
import * as c from 'codama';

const nightly = execSync('make --no-print-directory rust-toolchain-nightly').toString().trim();

export default {
    idl: 'idl.json',
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
                            c.assertIsNode(node, 'instructionArgumentNode');
                            c.assertIsNode(node.type, 'sizePrefixTypeNode');
                            return { ...node, type: { ...node.type, prefix: c.numberTypeNode('u64') } };
                        },
                    },
                ],
            ],
        },
    ],
    scripts: {
        js: {
            from: '@codama/renderers-js',
            args: ['clients/js', { kitImportStrategy: 'rootOnly', syncPackageJson: true }],
        },
        rust: {
            from: '@codama/renderers-rust',
            args: [
                'clients/rust',
                {
                    anchorTraits: false,
                    formatCode: true,
                    toolchain: `+${nightly}`,
                },
            ],
        },
    },
};
