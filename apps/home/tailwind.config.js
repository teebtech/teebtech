/** @type {import('tailwindcss').Config} */
module.exports = {
  content: {
    files: [
      "src/**/*.rs",
      "../../packages/ui/src/**/*.rs",
      "../../packages/design-system/src/**/*.rs",
    ],
  },
  theme: {
    extend: {
      fontFamily: {
        display: ['Fraunces', 'Georgia', 'serif'],
        sans: ['Inter', 'system-ui', 'sans-serif'],
        mono: ['"JetBrains Mono"', '"Courier New"', 'monospace'],
      },
      colors: {
        navy: {
          950: '#060c18',
          900: '#0b1220',
          800: '#0f1a2e',
          700: '#152238',
          600: '#1c2e4a',
          500: '#24395e',
        },
        brand: {
          50: '#eff6ff',
          100: '#dbeafe',
          200: '#bfdbfe',
          300: '#93c5fd',
          400: '#60a5fa',
          500: '#3b82f6',
          600: '#2563eb',
          700: '#1d4ed8',
          800: '#1e40af',
          900: '#1e3a8a',
        },
      },
    },
  },
  plugins: [],
}
