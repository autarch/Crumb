module.exports = {
  content: {
    files: ["index.html", "**/*.rs"],
    extract: {
      rs: (content) => {
        const rs_to_tw = (rs) => {
          if (rs.startsWith("two_")) {
            rs = rs.replace("two_", "2");
          }
          return rs
            .replaceAll("_of_", "\\/")
            .replaceAll("_p_", "\\.")
            .replaceAll("_", "-");
        };

        let classes = [];
        let class_re = /C\.[^ ]+\.([^\. ]+)\b/g;
        let mod_re = /(?:M\.([^\. ]+)\s*,\s*)+C\.[^ ]+\.([^\. ]+)\b/g;
        let matches = [...content.matchAll(mod_re)];
        if (matches.length > 0) {
          classes.push(
            ...matches.map((m) => {
              let pieces = m.slice(1, m.length);
              return pieces.map((p) => rs_to_tw(p)).join(":");
            })
          );
        }
        classes.push(
          ...[...content.matchAll(class_re)].map((m) => {
            return rs_to_tw(m[1]);
          })
        );
        return classes;
      },
    },
  },
  theme: {
    extend: {},
  },
  plugins: [require("@tailwindcss/forms")],
};
