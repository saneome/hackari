import { defineConfig } from 'vite'
import vue from '@vitejs/plugin-vue'
import { resolve } from 'path'

const allowedHosts = (process.env.VITE_ALLOWED_HOSTS || 'localhost,127.0.0.1')
  .split(',')
  .map((host) => host.trim())
  .filter(Boolean)

export default defineConfig({
  base: process.env.VITE_BASE_PATH || '/',
  plugins: [vue()],
  resolve: {
    alias: {
      '@': resolve(__dirname, 'src'),
    },
  },
  css: {
    preprocessorOptions: {
      scss: {
        additionalData: `@use "@/styles/variables.scss" as *;`,
      },
    },
  },
  build: {
    sourcemap: false,
    minify: 'esbuild',
    cssMinify: true,
  },
  preview: {
    allowedHosts,
  },
})
