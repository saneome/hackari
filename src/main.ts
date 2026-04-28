import { createApp } from 'vue'
import { gsap } from 'gsap'
import App from './App.vue'
import router from './router'
import './styles/main.scss'

;(window as unknown as { gsap: typeof gsap }).gsap = gsap

const app = createApp(App)
app.use(router)
app.mount('#app')
