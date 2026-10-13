import { defineConfig } from "vite";
import react from "@vitejs/plugin-react";

export default defineConfig({
  plugins: [react()],

  // Local development only — Vercel ignores this in production
  server: {
    proxy: {
      "/api": {
        target: "http://localhost:8000",
        changeOrigin: true,
        secure: false,
      },
    },
  },

  // Production build upgrades for Vercel
  build: {
    sourcemap: false, // smaller build, faster deploy
    minify: "esbuild", // fastest minifier
    chunkSizeWarningLimit: 800, // prevents warnings for chess assets
  },
});
