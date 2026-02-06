import * as c from 'codama';

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
            args: ['clients/js/src/generated', { packageFolder: 'clients/js', syncPackageJson: true }],
        },
        rust: {
            from: '@codama/renderers-rust',
            args: [
                'clients/rust/src/generated',
                {
                    crateFolder: 'clients/rust',
                    formatCode: true,
                    toolchain: '+nightly-2024-05-02',
                },
            ],
        },
    },
};
