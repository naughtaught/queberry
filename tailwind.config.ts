/** @type {import('tailwindcss').Config} */
export default {
    content: ['./src/**/*.{html,js,svelte,ts}'],
    safelist: ['dark', 'light'],
    theme: {
        extend: {
            colors: {},
            fontFamily: {},
            fontSize: {},
            animation: {},
            fontWeight: {},
            keyframes: {},
        },
    },
    variants: {
        extend: {},
    },
    plugins: [],
}
