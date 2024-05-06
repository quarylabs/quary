module.exports = {
  core: {
    disableTelemetry: true,
  },

  addons: ['@storybook/addon-a11y', '@storybook/addon-essentials'],
  stories: ['../src/**/*.stories.@(js|jsx|ts|tsx)'],

  framework: {
    name: '@storybook/react-vite',
    options: {},
  },

  typescript: {
    reactDocgen: false,
  },

  docs: {
    autodocs: true,
  },
}
