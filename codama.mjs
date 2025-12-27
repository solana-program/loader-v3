import { getToolchainArgument } from './scripts/utils.mjs';

export default {
    idl: 'program/idl.json',
    before: [
        './scripts/fix-write-data-prefix.mjs',
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
