import lume from "lume/mod.ts";
import remark from "lume/plugins/remark.ts";
import codeHighlight from "lume/plugins/code_highlight.ts";

import codeImport from "remark-code-import";

import lang_rust from "npm:highlight.js/lib/languages/rust";

const site = lume();

site.use(remark({
  remarkPlugins: [codeImport],
}));

site.use(
  codeHighlight({
    languages: {
      rust: lang_rust,
    },
    theme: [
      {
        name: "atom-one-light",
        cssFile: "/styles.css",
        placeholder: "/* light-theme-here */",
      },
      {
        name: "atom-one-dark",
        cssFile: "/styles.css",
        placeholder: "/* dark-theme-here */",
      },
    ],
  }),
);

export default site;
