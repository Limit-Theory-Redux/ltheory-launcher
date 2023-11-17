/** @type {import('tailwindcss').Config} */
export default {
  content: [],
  theme: {
    extend: {
      animation: {
        'slide-in-bottom': 'slideInFromBottom 0.5s ease-out forwards',
      }
    }
  },
  plugins: [
    require('tailwind-scrollbar-hide')
  ],
}

