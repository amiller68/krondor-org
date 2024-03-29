/** @type {import('tailwindcss').Config} */

module.exports = {
  content: { 
    files: ["*.html", "./src/**/*.rs"],
  },
  theme: {
    extend: {
      fontFamaily: {
        sans: ['Inter var', 'sans-serif'],
      },
      colors: {
        black: '#000',
        white: '#fff',
        accent: '#f56565',
        gray: {
          100: '#f7fafc',
          200: '#edf2f7',
          300: '#e2e8f0',
          400: '#cbd5e0',
          500: '#a0aec0',
          600: '#718096',
          700: '#4a5568',
          800: '#2d3748',
          900: '#1a202c'
        },
      },
      userSelect: ['none']
    },
  },
  plugins: [],
};