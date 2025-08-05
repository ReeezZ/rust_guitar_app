/** @type {import('tailwindcss').Config} */
module.exports = {
  content: {
    files: ["*.html", "./src/**/*.rs", "./public/styles/app.css"],
    transform: {
      rs: (content) => content.replace(/(?:^|\s)class:/g, ' '),
    },
  },
  theme: {
    extend: {
      backgroundImage: {
        'fretboard': "url('/img/assets/wood-pattern.png')",
      },
      colors: {
        "primary": "var(--primary-background)",
        "primary-rev": "var(--primary-color)",
        "primary-shades": "var(--navbar-background)",
      }
    },
  },
  plugins: [
    require('tailwindcss-animated')
  ],
}