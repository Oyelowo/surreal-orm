module.exports = {
    env: {
        es2021: true,
        node: true,
        browser: false,
    },
    extends: ['plugin:unicorn/recommended', 'plugin:@typescript-eslint/recommended', 'prettier'],
    parserOptions: {
        ecmaVersion: 12,
        sourceType: 'module',
        parser: '@typescript-eslint/parser',
    },
    rules: {
        // 'prettier/prettier': 'error',
        indent: 'off',
        '@typescript-eslint/indent': 'off',
        '@typescript-eslint/explicit-function-return-type': 'off',
        '@typescript-eslint/no-explicit-any': 'off',
        '@typescript-eslint/no-unused-vars': 'off',
        '@typescript-eslint/no-empty-interface': 'off',
        'unicorn/prefer-module': 'error',
        'unicorn/consistent-function-scoping': 'off',
        'unicorn/filename-case': [
            'error',
            {
                case: 'camelCase',
            },
        ],
        'unused-imports/no-unused-imports': 'error',
        'unicorn/no-array-callback-reference': 'off',
        'unicorn/no-array-for-each': 'off',
        'unicorn/prevent-abbreviations': 'off',
        'no-restricted-syntax': [
            'error',
            {
                selector: 'ClassDeclaration[superClass]',
                message: "Extending other classes via inheritance isn't allowed. Use composition instead.",
            },
        ],
    },
    plugins: ['prettier', '@typescript-eslint', 'eslint-plugin-unused-imports', 'unicorn'],
};
