/** @type {import('tailwindcss').Config} */
module.exports = {
    content: ["*.html", "./src/**/*.rs",],
    theme: {
      extend: {   
        colors: {
          customBlue: {
            DEFAULT: '#1A223A',
            HOVER: '#101930',},
          customRed: {
            DEFAULT: '#FFDADA',
            HOVER: '#FFABAB'
          }
    },
    plugins: [],
  }
}
}