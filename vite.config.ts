import tailwindcss from '@tailwindcss/vite'
import { sveltekit } from '@sveltejs/kit/vite'
import { defineConfig } from 'vite'

export default defineConfig({
    plugins: [tailwindcss(), sveltekit()],

    // Exclude plugins directory from Vite's file watching
    server: {
        watch: {
            ignored: [
                'plugins/', // Don't watch plugin directory
                '**/*.wasm', // Don't watch WASM files
                '**/src-tauri/target/**', // Don't watch Rust build output
            ],
        },
        fs: {
            // Don't serve these files
            deny: ['**/plugins/**', '**/*.wasm'],
        },
    },

    // Don't try to optimize WASM files
    optimizeDeps: {
        exclude: ['*.wasm'],
    },
})
