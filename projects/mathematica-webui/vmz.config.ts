import { defineConfig } from "@vmz/vmz";

/** WebUI for Tauri — browser/static delivery (vmz-framework 当前合同). */
export default defineConfig({
  delivery: {
    default: "static",
    profiles: {
      static: { host: "browser", assembly: "web-static" },
    },
  },
});
