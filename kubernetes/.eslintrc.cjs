module.exports = {
    env: {
        es6: true,
        browser: false,
        node: true,
        es2021: true,
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
        'unicorn/prfer-module': 'error',
        'unused-imports/no-unused-imports': 'error',
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
