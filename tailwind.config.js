/** @type {import('tailwindcss').Config} */
module.exports = {
  mode: "all",
  content: ["./src/**/*.{rs,html,css}", "./dist/**/*.html"],
  theme: {
    fontSize: {
      sm: "14px",
      base: "16px",
      lg: "19px",
      xl: "23px",
      "2xl": "28px",
      "3xl": "34px",
      "4xl": "42px",
      "5xl": "50px",
    },
    extend: {
      screens: {
        b: "0px",
      },
    },
  },
  plugins: [],
};
