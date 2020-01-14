module.exports = {
  root: true,
  env: {
    node: true,
  },
  extends: [
    'plugin:vue/essential',
    '@vue/airbnb',
    '@vue/typescript',
  ],
  rules: {
    'no-console': process.env.NODE_ENV === 'production' ? 'error' : 'off',
    'no-debugger': process.env.NODE_ENV === 'production' ? 'error' : 'off',
    'lines-between-class-members': 'off',
    'class-methods-use-this': 'off',
    'import/no-unresolved': [2, { ignore: ['bloom_native'] }],
    'no-unused-vars': 'off',
    '@typescript-eslint/no-unused-vars': 'error',
  },
  parserOptions: {
    parser: '@typescript-eslint/parser',
  },
  plugins: ['@typescript-eslint'],
  overrides: [
    {
      files: [
        '**/__tests__/*.{j,t}s?(x)',
      ],
      env: {
        jest: true,
      },
    },
  ],
  settings: {
    'import/core-modules': ['electron', 'bloom_native'],
    'import/resolver': { // see https://github.com/benmosher/eslint-plugin-import/issues/1396
      node: {},
    },
  },
};
