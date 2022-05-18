module.exports = {
    env: {
        es6: true,
        browser: false,
        node: true,
        es2021: true,
    },
    extends: ['plugin:@typescript-eslint/recommended', 'prettier'],
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
    },
    plugins: ['prettier', '@typescript-eslint'],
};
