/** @type {import('tailwindcss').Config} */
export default {
  content: [
    "./index.html",
    "./src/**/*.{js,ts,jsx,tsx}",
  ],
  theme: {
    extend: {
      colors: {
        primaryDark: 'rgb(18 55 64)',
        primary: 'rgb(84 154 171)',
        primaryLight: 'rgb(176 215 225)',
        background: 'rgb(246 246 246)',
        accent: 'rgb(241 128 45)',
      },
      fontFamily: {
        sans: ['Inter', 'system-ui', 'sans-serif'],
      },
    },
  },
  plugins: [],
}
