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
        coolBlue: {
          DEFAULT: '#1879BD',
          HOVER: "#1260A4",
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
        classCardGreen: { 
          DEFAULT: '#4F8A77',
          HOVER:   '#426F61'
        },
        classCardBrown: { 
          DEFAULT: '#C78544',
          HOVER: '#A66E39'
        },
        classCardBlue: { 
          DEFAULT: '#6178CF',
          HOVER: '#526ABC'
        },
        classCardPurple: { 
          DEFAULT: '#8769C3',
          HOVER: '#7259A5'
        },
        warningNotification: {
          bg: '#FFEDB4',
          details: '#B47E29',
        },
        instructorYellow: {
          DEFAULT: '#F9EBAF',
          details: '#B47E29'
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
