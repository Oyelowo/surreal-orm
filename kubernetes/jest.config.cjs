/** @type {import('ts-jest/dist/types').InitialOptionsTsJest} */
module.exports = {
    preset: 'ts-jest/presets/default-esm', // or other ESM presets
    globals: {
        'ts-jest': {
            useESM: true,
            diagnostics: false,
        },
    },
    moduleNameMapper: {
        '^(\\.{1,2}/.*)\\.js$': '$1',
        '#(.*)': '<rootDir>/node_modules/$1',
    },
    testEnvironment: 'node',
};
