import { type Config } from 'prettier';

const config: Config = {
  plugins: ['prettier-plugin-svelte', 'prettier-plugin-tailwindcss'],
  useTabs: false,
  singleQuote: true,
  trailingComma: 'none',
  printWidth: 120,
  overrides: [
    {
      files: '*.svelte',
      options: {
        parser: 'svelte'
      }
    }
  ],
  tailwindStylesheet: './src/routes/layout.css'
};

export default config;
