import { defineConfig } from 'vite';
import { svelte } from '@sveltejs/vite-plugin-svelte';
import { viteStaticCopy } from 'vite-plugin-static-copy';

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [
    svelte(),
    viteStaticCopy({
      targets: [
        {
          src: './weave.config.json', // Ensure this path is correct
          dest: '.' // This will copy to the root of the output directory
        },
        {
          src: 'public/**/*', // Copy all files from the public directory
          dest: '.' // Copy to the root of the output directory
        }
      ]
    })
  ]
});