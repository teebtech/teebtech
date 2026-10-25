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
    extend: {},
  },
  plugins: [],
}
