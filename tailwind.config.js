/** @type {import('tailwindcss').Config} */
module.exports = {
    content: ["*.html", "./src/**/*.rs",],
    theme: {
      extend: {   
        colors: {
          customBlue: {
            DEFAULT: '#1A223A',
            hover: '#101930',},
    },
    plugins: [],
  }
}
}