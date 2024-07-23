// vite.config.ts
import { defineConfig } from "file:///X:/dev/projects/tauri-plugin-escpos/examples/app/node_modules/vite/dist/node/index.js";
import vue from "file:///X:/dev/projects/tauri-plugin-escpos/examples/app/node_modules/@vitejs/plugin-vue/dist/index.mjs";
import { internalIpV4 } from "file:///X:/dev/projects/tauri-plugin-escpos/examples/app/node_modules/internal-ip/index.js";
var mobile = !!/android|ios/.exec(process.env.TAURI_ENV_PLATFORM);
var vite_config_default = defineConfig(async () => ({
  plugins: [vue()],
  // Vite options tailored for Tauri development and only applied in `tauri dev` or `tauri build`
  //
  // 1. prevent vite from obscuring rust errors
  clearScreen: false,
  // 2. tauri expects a fixed port, fail if that port is not available
  server: {
    port: 1420,
    strictPort: true,
    host: mobile ? "0.0.0.0" : false,
    hmr: mobile ? {
      protocol: "ws",
      host: await internalIpV4(),
      port: 1421
    } : void 0,
    watch: {
      // 3. tell vite to ignore watching `src-tauri`
      ignored: ["**/src-tauri/**"]
    }
  }
}));
export {
  vite_config_default as default
};
//# sourceMappingURL=data:application/json;base64,ewogICJ2ZXJzaW9uIjogMywKICAic291cmNlcyI6IFsidml0ZS5jb25maWcudHMiXSwKICAic291cmNlc0NvbnRlbnQiOiBbImNvbnN0IF9fdml0ZV9pbmplY3RlZF9vcmlnaW5hbF9kaXJuYW1lID0gXCJYOlxcXFxkZXZcXFxccHJvamVjdHNcXFxcdGF1cmktcGx1Z2luLWVzY3Bvc1xcXFxleGFtcGxlc1xcXFxhcHBcIjtjb25zdCBfX3ZpdGVfaW5qZWN0ZWRfb3JpZ2luYWxfZmlsZW5hbWUgPSBcIlg6XFxcXGRldlxcXFxwcm9qZWN0c1xcXFx0YXVyaS1wbHVnaW4tZXNjcG9zXFxcXGV4YW1wbGVzXFxcXGFwcFxcXFx2aXRlLmNvbmZpZy50c1wiO2NvbnN0IF9fdml0ZV9pbmplY3RlZF9vcmlnaW5hbF9pbXBvcnRfbWV0YV91cmwgPSBcImZpbGU6Ly8vWDovZGV2L3Byb2plY3RzL3RhdXJpLXBsdWdpbi1lc2Nwb3MvZXhhbXBsZXMvYXBwL3ZpdGUuY29uZmlnLnRzXCI7aW1wb3J0IHsgZGVmaW5lQ29uZmlnIH0gZnJvbSBcInZpdGVcIjtcbmltcG9ydCB2dWUgZnJvbSBcIkB2aXRlanMvcGx1Z2luLXZ1ZVwiO1xuaW1wb3J0IHsgaW50ZXJuYWxJcFY0IH0gZnJvbSBcImludGVybmFsLWlwXCI7XG5cbi8vIEB0cy1leHBlY3QtZXJyb3IgcHJvY2VzcyBpcyBhIG5vZGVqcyBnbG9iYWxcbmNvbnN0IG1vYmlsZSA9ICEhL2FuZHJvaWR8aW9zLy5leGVjKHByb2Nlc3MuZW52LlRBVVJJX0VOVl9QTEFURk9STSk7XG5cbi8vIGh0dHBzOi8vdml0ZWpzLmRldi9jb25maWcvXG5leHBvcnQgZGVmYXVsdCBkZWZpbmVDb25maWcoYXN5bmMgKCkgPT4gKHtcbiAgcGx1Z2luczogW3Z1ZSgpXSxcblxuICAvLyBWaXRlIG9wdGlvbnMgdGFpbG9yZWQgZm9yIFRhdXJpIGRldmVsb3BtZW50IGFuZCBvbmx5IGFwcGxpZWQgaW4gYHRhdXJpIGRldmAgb3IgYHRhdXJpIGJ1aWxkYFxuICAvL1xuICAvLyAxLiBwcmV2ZW50IHZpdGUgZnJvbSBvYnNjdXJpbmcgcnVzdCBlcnJvcnNcbiAgY2xlYXJTY3JlZW46IGZhbHNlLFxuICAvLyAyLiB0YXVyaSBleHBlY3RzIGEgZml4ZWQgcG9ydCwgZmFpbCBpZiB0aGF0IHBvcnQgaXMgbm90IGF2YWlsYWJsZVxuICBzZXJ2ZXI6IHtcbiAgICBwb3J0OiAxNDIwLFxuICAgIHN0cmljdFBvcnQ6IHRydWUsXG4gICAgaG9zdDogbW9iaWxlID8gXCIwLjAuMC4wXCIgOiBmYWxzZSxcbiAgICBobXI6IG1vYmlsZVxuICAgICAgPyB7XG4gICAgICAgICAgcHJvdG9jb2w6IFwid3NcIixcbiAgICAgICAgICBob3N0OiBhd2FpdCBpbnRlcm5hbElwVjQoKSxcbiAgICAgICAgICBwb3J0OiAxNDIxLFxuICAgICAgICB9XG4gICAgICA6IHVuZGVmaW5lZCxcbiAgICB3YXRjaDoge1xuICAgICAgLy8gMy4gdGVsbCB2aXRlIHRvIGlnbm9yZSB3YXRjaGluZyBgc3JjLXRhdXJpYFxuICAgICAgaWdub3JlZDogW1wiKiovc3JjLXRhdXJpLyoqXCJdLFxuICAgIH0sXG4gIH0sXG59KSk7XG4iXSwKICAibWFwcGluZ3MiOiAiO0FBQThVLFNBQVMsb0JBQW9CO0FBQzNXLE9BQU8sU0FBUztBQUNoQixTQUFTLG9CQUFvQjtBQUc3QixJQUFNLFNBQVMsQ0FBQyxDQUFDLGNBQWMsS0FBSyxRQUFRLElBQUksa0JBQWtCO0FBR2xFLElBQU8sc0JBQVEsYUFBYSxhQUFhO0FBQUEsRUFDdkMsU0FBUyxDQUFDLElBQUksQ0FBQztBQUFBO0FBQUE7QUFBQTtBQUFBLEVBS2YsYUFBYTtBQUFBO0FBQUEsRUFFYixRQUFRO0FBQUEsSUFDTixNQUFNO0FBQUEsSUFDTixZQUFZO0FBQUEsSUFDWixNQUFNLFNBQVMsWUFBWTtBQUFBLElBQzNCLEtBQUssU0FDRDtBQUFBLE1BQ0UsVUFBVTtBQUFBLE1BQ1YsTUFBTSxNQUFNLGFBQWE7QUFBQSxNQUN6QixNQUFNO0FBQUEsSUFDUixJQUNBO0FBQUEsSUFDSixPQUFPO0FBQUE7QUFBQSxNQUVMLFNBQVMsQ0FBQyxpQkFBaUI7QUFBQSxJQUM3QjtBQUFBLEVBQ0Y7QUFDRixFQUFFOyIsCiAgIm5hbWVzIjogW10KfQo=
