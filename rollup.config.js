import { terser } from "rollup-plugin-terser";
import { generateSW } from "rollup-plugin-workbox";

export default {
  input: "./templates/assets/main.js",
  output: {
    file: "./content/assets/main.js",
    format: "cjs",
  },
  plugins: [
    generateSW({
      swDest: "./content/sw.js",
      globDirectory: "./content",
      globPatterns: ["**/*.{html,js,css}"],
      clientsClaim: true,
      skipWaiting: true,
      mode: "productino",
    }),
    terser(),
  ],
};
