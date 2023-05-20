// https://astro.build/config

import { defineConfig } from 'astro/config';
import react from '@astrojs/react';
import tailwind from '@astrojs/tailwind';
import image from '@astrojs/image';
import vercel from '@astrojs/vercel/serverless';

export default defineConfig({
  output: 'server',
  adapter: vercel(),
  // base: '.', // Set a path prefix.
  site: 'https://example.com/',
  markdown: {
    shikiConfig: {
      // Choose from Shiki's built-in themes (or add your own)
      // https://github.com/shikijs/shiki/blob/main/docs/themes.md
      theme: 'monokai'
    }
  },
  integrations: [
    react(),
    tailwind({}),
    image({
      serviceEntryPoint: '@astrojs/image/sharp'
    })
  ]
});
