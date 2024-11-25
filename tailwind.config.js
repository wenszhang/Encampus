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
        customGreen: {
          DEFAULT: '#EFF6EC',
          details: '#5D993E',
        },
        customYellow: { 
          DEFAULT: '#F9EBAF',
          details: '#B47E29'
        },
        customOrange: { 
          DEFAULT: '#FFB74D',
        },
        warningNotification: {
          bg: '#FFEDB4',
          details: '#B47E29',
        },
        errorNotification: {
          bg: '#FDDFDD',
          details: '#F23B2F',
        },
        card: {
          bg: "#F8C1C0",
          header: "#EEEEEE",
        },
       
        plugins: [],
      },
      textColor: {
        'warningNotification-details': '#B47E29',
        'errorNotification-details': '#E54C4C',
      },
      boxShadow: {
        customInset:
          "inset 6px 6px 12px #cacaca, inset -6px -6px 12px #ffffff",
      },
    },
  },
};
