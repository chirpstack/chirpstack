import { defineConfig } from "vite";
import react from "@vitejs/plugin-react-swc";

// https://vitejs.dev/config/
export default defineConfig({
  build: {
    outDir: "build",
    sourcemap: true,
  },
  server: {
    proxy: {
      "^/api.*": {
        target: "http://localhost:8080",
        changeOrigin: true,
      },
    },
  },
  plugins: [react()],
  legacy: {
    // Fix AceEditor rendering. See:
    // https://github.com/securingsincity/react-ace/issues/1540#issuecomment-3685386545
    inconsistentCjsInterop: true,
  },
});
