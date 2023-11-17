// https://nuxt.com/docs/api/configuration/nuxt-config
export default defineNuxtConfig({
  devtools: { enabled: false },
  ssr: false,
  srcDir: 'src',
  css: [
    '~/styles.css'
  ],
  modules: [
    '@nuxtjs/tailwindcss',
    'vuetify-nuxt-module',
    '@unocss/nuxt',
  ],
  vuetify: {
    vuetifyOptions: {
      icons: {
        defaultSet: 'unocss-mdi'
      }
    }
  }
})
