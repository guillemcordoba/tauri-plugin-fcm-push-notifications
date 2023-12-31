// vite.config.js
import { defineConfig } from "file:///home/guillem/projects/tauri-plugin-push-notifications/examples/tauri-app/node_modules/vite/dist/node/index.js";
import { svelte } from "file:///home/guillem/projects/tauri-plugin-push-notifications/examples/tauri-app/node_modules/@sveltejs/vite-plugin-svelte/dist/index.js";
import { internalIpV4Sync } from "file:///home/guillem/projects/tauri-plugin-push-notifications/examples/tauri-app/node_modules/internal-ip/index.js";
var mobile = !!/android|ios/.exec(process.env.TAURI_ENV_PLATFORM);
var vite_config_default = defineConfig({
  plugins: [svelte()],
  clearScreen: false,
  server: {
    host: mobile ? "0.0.0.0" : false,
    port: 1420,
    strictPort: true,
    hmr: mobile ? {
      protocol: "ws",
      host: internalIpV4Sync(),
      port: 1421
    } : void 0
  }
});
export {
  vite_config_default as default
};
//# sourceMappingURL=data:application/json;base64,ewogICJ2ZXJzaW9uIjogMywKICAic291cmNlcyI6IFsidml0ZS5jb25maWcuanMiXSwKICAic291cmNlc0NvbnRlbnQiOiBbImNvbnN0IF9fdml0ZV9pbmplY3RlZF9vcmlnaW5hbF9kaXJuYW1lID0gXCIvaG9tZS9ndWlsbGVtL3Byb2plY3RzL3RhdXJpLXBsdWdpbi1wdXNoLW5vdGlmaWNhdGlvbnMvZXhhbXBsZXMvdGF1cmktYXBwXCI7Y29uc3QgX192aXRlX2luamVjdGVkX29yaWdpbmFsX2ZpbGVuYW1lID0gXCIvaG9tZS9ndWlsbGVtL3Byb2plY3RzL3RhdXJpLXBsdWdpbi1wdXNoLW5vdGlmaWNhdGlvbnMvZXhhbXBsZXMvdGF1cmktYXBwL3ZpdGUuY29uZmlnLmpzXCI7Y29uc3QgX192aXRlX2luamVjdGVkX29yaWdpbmFsX2ltcG9ydF9tZXRhX3VybCA9IFwiZmlsZTovLy9ob21lL2d1aWxsZW0vcHJvamVjdHMvdGF1cmktcGx1Z2luLXB1c2gtbm90aWZpY2F0aW9ucy9leGFtcGxlcy90YXVyaS1hcHAvdml0ZS5jb25maWcuanNcIjtpbXBvcnQgeyBkZWZpbmVDb25maWcgfSBmcm9tIFwidml0ZVwiO1xuaW1wb3J0IHsgc3ZlbHRlIH0gZnJvbSBcIkBzdmVsdGVqcy92aXRlLXBsdWdpbi1zdmVsdGVcIjtcbmltcG9ydCB7IGludGVybmFsSXBWNFN5bmMgfSBmcm9tICdpbnRlcm5hbC1pcCdcblxuY29uc3QgbW9iaWxlID0gISEvYW5kcm9pZHxpb3MvLmV4ZWMocHJvY2Vzcy5lbnYuVEFVUklfRU5WX1BMQVRGT1JNKTtcblxuLy8gaHR0cHM6Ly92aXRlanMuZGV2L2NvbmZpZy9cbmV4cG9ydCBkZWZhdWx0IGRlZmluZUNvbmZpZyh7XG4gIHBsdWdpbnM6IFtzdmVsdGUoKV0sXG5cbiAgLy8gVml0ZSBvcHRvbnMgdGFpbG9yZWQgZm9yIFRhdXJpIGRldmVsb3BtZW50IGFuZCBvbmx5IGFwcGxpZWQgaW4gYHRhdXJpIGRldmAgb3IgYHRhdXJpIGJ1aWxkYFxuICAvLyBwcmV2ZW50IHZpdGUgZnJvbSBvYnNjdXJpbmcgcnVzdCBlcnJvcnNcbiAgY2xlYXJTY3JlZW46IGZhbHNlLFxuICAvLyB0YXVyaSBleHBlY3RzIGEgZml4ZWQgcG9ydCwgZmFpbCBpZiB0aGF0IHBvcnQgaXMgbm90IGF2YWlsYWJsZVxuICBzZXJ2ZXI6IHtcbiAgICBob3N0OiBtb2JpbGUgPyBcIjAuMC4wLjBcIiA6IGZhbHNlLFxuICAgIHBvcnQ6IDE0MjAsXG4gICAgc3RyaWN0UG9ydDogdHJ1ZSxcbiAgICBobXI6IG1vYmlsZSA/IHtcbiAgICAgIHByb3RvY29sOiAnd3MnLFxuICAgICAgaG9zdDogaW50ZXJuYWxJcFY0U3luYygpLFxuICAgICAgcG9ydDogMTQyMVxuICAgIH0gOiB1bmRlZmluZWQsXG4gIH0sXG59KVxuIl0sCiAgIm1hcHBpbmdzIjogIjtBQUE2WSxTQUFTLG9CQUFvQjtBQUMxYSxTQUFTLGNBQWM7QUFDdkIsU0FBUyx3QkFBd0I7QUFFakMsSUFBTSxTQUFTLENBQUMsQ0FBQyxjQUFjLEtBQUssUUFBUSxJQUFJLGtCQUFrQjtBQUdsRSxJQUFPLHNCQUFRLGFBQWE7QUFBQSxFQUMxQixTQUFTLENBQUMsT0FBTyxDQUFDO0FBQUEsRUFJbEIsYUFBYTtBQUFBLEVBRWIsUUFBUTtBQUFBLElBQ04sTUFBTSxTQUFTLFlBQVk7QUFBQSxJQUMzQixNQUFNO0FBQUEsSUFDTixZQUFZO0FBQUEsSUFDWixLQUFLLFNBQVM7QUFBQSxNQUNaLFVBQVU7QUFBQSxNQUNWLE1BQU0saUJBQWlCO0FBQUEsTUFDdkIsTUFBTTtBQUFBLElBQ1IsSUFBSTtBQUFBLEVBQ047QUFDRixDQUFDOyIsCiAgIm5hbWVzIjogW10KfQo=
