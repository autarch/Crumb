const plugin = require("tailwindcss/plugin");

// [...range(1,5,2)] => [1,3,5]
function* range(start, end, step) {
  if (start > end) {
    return;
  }
  yield start;
  yield* range(start + step, end, step);
}

// For NodeJS < 12
Object.fromEntries =
  Object.fromEntries ||
  ((iterable) => {
    return [...iterable].reduce((obj, [key, val]) => {
      obj[key] = val;
      return obj;
    }, {});
  });

// Copied from
// https://github.com/reslear/tailwind-scrollbar-hide/blob/main/src/index.js
// and adjusted to make it show scrollbar on hover.
const scrollbarOnHover = plugin(function ({ addUtilities }) {
  addUtilities({
    ".scrollbar-on-hover": {
      /* IE and Edge */
      "-ms-overflow-style": "-ms-autohiding-scrollbar",

      /* Firefox */
      "scrollbar-width": "none",
      "scrollbar-gutter": "stable",

      /* Safari and Chrome */
      "&::-webkit-scrollbar": {
        width: "0.5rem",
        "background-color": "transparent",
      },
      "&::-webkit-scrollbar-thumb": {
        visibility: "hidden",
        // Color matches indigo 600 from Tailwind colors
        "background-color": "#4F46E5",
      },

      "&:hover": {
        // Firefox
        "scrollbar-width": "auto",

        /* Safari and Chrome */
        "&::-webkit-scrollbar": {
          "background-color": "black",
        },
        "&::-webkit-scrollbar-thumb": {
          visibility: "visible",
        },
      },
    },
  });
});

module.exports = {
  theme: {
    fontFamily: {
      sans: ["Noto Sans", "ui-sans-serif", "system"],
    },
    extend: {
      height: {
        // The menu height is h_14. The now playing bar height is h_24. The
        // extra 'spacing.6' just seem to be needed to avoid having a scrollbar
        // for the queue page as a whole.
        "fit-in-viewport":
          "calc( 100vh - ( theme('spacing.14') + theme('spacing.24') + theme('spacing.6') ) )",
      },
    },
  },
  variants: {
    extend: {
      borderWidth: ["last"],
    },
  },
  plugins: [scrollbarOnHover],
};
