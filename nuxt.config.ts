import tailwindcss from "@tailwindcss/vite";

// https://nuxt.com/docs/api/configuration/nuxt-config
export default defineNuxtConfig({
  compatibilityDate: "2026-08-03",
  devtools: { enabled: false },
  ssr: false,
  srcDir: "src",
  css: ["~/styles.css"],
  vite: {
    plugins: [tailwindcss()],
  },
  app: {
    head: {
      htmlAttrs: { lang: "en" },
      meta: [
        { name: "color-scheme", content: "dark" },
        { name: "theme-color", content: "#071019" },
      ],
    },
  },
});
