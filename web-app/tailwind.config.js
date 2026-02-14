/** @type {import('tailwindcss').Config} */
export default {
  content: [
    "./index.html",
    "./src/**/*.{js,ts,jsx,tsx}",
  ],
  theme: {
    extend: {
      colors: {
        bitcoin: {
          50: '#fff8f0',
          100: '#ffedd5',
          200: '#fed7aa',
          300: '#fdba74',
          400: '#fb923c',
          500: '#f7931a', // Bitcoin Orange
          600: '#e58910',
          700: '#c2770f',
          800: '#9a6110',
          900: '#7c4f10',
        },
        lightning: {
          50: '#fef9c3',
          100: '#fef08a',
          200: '#fde047',
          300: '#facc15',
          400: '#eab308',
          500: '#ca8a04',
          600: '#a16207',
          700: '#854d0e',
          800: '#713f12',
          900: '#422006',
        },
        dark: {
          950: '#0a0e27',
          900: '#151932',
          800: '#1e2440',
          700: '#2d3454',
          600: '#3d4468',
          500: '#4a5568',
        },
      },
      backgroundImage: {
        'gradient-radial': 'radial-gradient(var(--tw-gradient-stops))',
        'gradient-bitcoin': 'linear-gradient(135deg, #f7931a 0%, #fbbf24 100%)',
        'gradient-lightning': 'linear-gradient(135deg, #facc15 0%, #fef08a 100%)',
        'gradient-dark': 'linear-gradient(135deg, #0a0e27 0%, #1e2440 50%, #2d1b4e 100%)',
      },
      animation: {
        'float': 'float 6s ease-in-out infinite',
        'pulse-slow': 'pulse 3s cubic-bezier(0.4, 0, 0.6, 1) infinite',
        'gradient': 'gradient 15s ease infinite',
        'shimmer': 'shimmer 2s infinite',
        'glow': 'glow 2s ease-in-out infinite',
      },
      keyframes: {
        float: {
          '0%, 100%': { transform: 'translateY(0px)' },
          '50%': { transform: 'translateY(-20px)' },
        },
        gradient: {
          '0%, 100%': { backgroundPosition: '0% 50%' },
          '50%': { backgroundPosition: '100% 50%' },
        },
        shimmer: {
          '0%': { backgroundPosition: '-1000px 0' },
          '100%': { backgroundPosition: '1000px 0' },
        },
        glow: {
          '0%, 100%': {
            boxShadow: '0 0 20px rgba(247, 147, 26, 0.4), 0 0 40px rgba(247, 147, 26, 0.2)',
          },
          '50%': {
            boxShadow: '0 0 30px rgba(247, 147, 26, 0.6), 0 0 60px rgba(247, 147, 26, 0.4)',
          },
        },
      },
      boxShadow: {
        'bitcoin': '0 0 20px rgba(247, 147, 26, 0.4), 0 0 40px rgba(247, 147, 26, 0.2)',
        'bitcoin-lg': '0 0 30px rgba(247, 147, 26, 0.5), 0 0 60px rgba(247, 147, 26, 0.3)',
        'glass': '0 8px 32px 0 rgba(31, 38, 135, 0.37)',
      },
      backdropBlur: {
        'xs': '2px',
      },
    },
  },
  plugins: [],
}
