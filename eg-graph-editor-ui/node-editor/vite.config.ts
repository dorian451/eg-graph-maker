import { svelte } from "@sveltejs/vite-plugin-svelte";
import { resolve } from "path";
import { defineConfig } from "vite";

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [svelte()],
  build: {
    // rollupOptions: {
    //   output: {
    //     format: "module",
    //     entryFileNames(chunkInfo) {
    //       return "node-editor.js";
    //     },
    //   },
    // },
    lib: {
      entry: resolve(__dirname, "src/main.ts"),
      name: "thing",
      fileName: "node-editor",
      formats: ["es"],
    },
  },
});
