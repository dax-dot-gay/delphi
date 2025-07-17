import { defineConfig } from "vite";
import react from "@vitejs/plugin-react-swc";
import { readFileSync } from "node:fs";
import { run } from "vite-plugin-run";

// https://vite.dev/config/
export default defineConfig({
  plugins: [react(), run([
    {
      name: "OpenAPI Watcher",
      pattern: "openapi.json",
      run: ["bash", "../openapi.sh"]
    }
  ])],
  server: {
    https: {
      key: readFileSync("/workspaces/delphi/certs/client/key.pem"),
      cert: readFileSync("/workspaces/delphi/certs/client/cert.pem"),
    },
    host: "0.0.0.0",
    port: 8080,
    strictPort: true,
    proxy: {
      "/api": {
        target: "https://0.0.0.0:8081",
        changeOrigin: true,
        rewrite: (path) => path.replace(/^\/api/, ""),
        secure: false,
      },
    },
  },
});
