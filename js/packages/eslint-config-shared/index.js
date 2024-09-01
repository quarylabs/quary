module.exports = {
  plugins: ['@typescript-eslint', 'import', 'react'],
  extends: [
    'eslint:recommended',
    'plugin:@typescript-eslint/recommended',
    'plugin:react/recommended',
    'plugin:storybook/recommended',
    'plugin:storybook/csf-strict',
    'plugin:jsx-a11y/recommended',
    'plugin:import/typescript',
  ],
  rules: {
    'arrow-body-style': ['error', 'as-needed'],
    semi: ['error', 'never'],
    'no-new-wrappers': 'error',
    camelcase: 'off',
    'new-cap': 'off',
    'no-restricted-globals': 'error',
    'prefer-const': 'error',
    'no-var': 'error',
    'object-shorthand': 'error',
    'quote-props': ['error', 'as-needed'],
    'no-else-return': 'error',
    'no-duplicate-imports': 'error',
    'jsx-quotes': ['warn', 'prefer-double'],
    'no-console': 'error',
    'no-use-before-define': 'off',
    'no-throw-literal': 'error',
    curly: 'error',
    eqeqeq: 'error',

    'import/no-self-import': 'error',
    'import/no-mutable-exports': 'error',
    'import/no-relative-packages': 'error',
    'import/no-cycle': 'error',
    'import/no-unresolved': 'off',
    'import/order': [
      'warn',
      {
        groups: [
          'builtin',
          'external',
          'internal',
          'parent',
          'sibling',
          'index',
        ],
      },
    ],
    'import/extensions': 'off',
    'import/prefer-default-export': 'off',
    'import/no-extraneous-dependencies': [
      'warn',
      {
        // Due to importing '@testing-library/jest-dom/extend-expect' and stuff in tests
        devDependencies: [
          '.storybook/***',
          '**/*.stories.{tsx,ts}',
          '**/*.test.{tsx,ts}, uicomponents',
        ],
      },
    ],

    '@typescript-eslint/no-unused-vars': 'warn',

    'react/function-component-definition': ['off'],
    'react/no-unstable-nested-components': ['off'],
    'react/no-unused-prop-types': ['error'],
    'react/no-danger': ['warn'],
    'react/prop-types': 'off',
    'react/react-in-jsx-scope': 'off',
    'react/require-default-props': 'off',
    'react/jsx-curly-brace-presence': [
      'error',
      {
        props: 'never',
        children: 'never',
      },
    ],
    'react/jsx-filename-extension': [
      'warn',
      {
        extensions: ['.js', '.jsx', '.ts', '.tsx'],
      },
    ],
  },
}
