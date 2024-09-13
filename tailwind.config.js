/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["*.html", "./src/**/*.rs"],
  theme: {
    extend: {
      colors: {
        customBlue: {
          DEFAULT: "#101930",
          HOVER: "#213873",
        },
        customRed: {
          DEFAULT: "#FFDADA",
          HOVER: "#FFABAB",
        },
        customPurple: {
          DEFAULT: "#D8C4F7",
        },
        card: {
          bg: "#FFFFFF",
          header: "#EEEEEE",
        },
        plugins: [],
      },
      boxShadow: {
        customInset:
          "inset 6px 6px 12px #cacaca, inset -6px -6px 12px #ffffff",
      },
    },
  },
};
