/** @type {import('tailwindcss').Config} */
module.exports = {
    content: [
        "./index.html",
        "./src/**/*.rs",
    ],
    theme: {
        colors: {
            "white": "#fff",
            "primary": "#7c3aed",
            "secondary": "#006bff",
            "accent": "#009100",
            "neutral": "#051a15",
            "base": "#2e2a2e",
            "fore": "#1F1C1F",
            "info": "#00ebff",
            "success": "#2bd50f",
            "warning": "#ff7300",
            "error": "#d60015",
        },
    },
    plugins: [],
}

