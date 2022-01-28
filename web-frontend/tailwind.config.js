module.exports = {
  content: ["./index.html", "src/**/*.rs"],
  theme: {
    extend: {},
  },
  plugins: [require("@tailwindcss/forms")],
};
