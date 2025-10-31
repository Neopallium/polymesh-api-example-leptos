import { defineConfig } from 'vite';
import { resolve } from 'path';

export default defineConfig({
    build: {
        lib: {
            entry: resolve(__dirname, 'js/web3.js'),
            name: 'Web3',
            fileName: 'web3',
            formats: ['es']
        },
        outDir: 'assets/js',
        emptyOutDir: true,
        rollupOptions: {
            output: {
                // Preserve the original structure
                preserveModules: false,
                entryFileNames: 'web3.js',
            }
        }
    }
});
