import { createApp } from 'vue'
import App from './App.vue'
import router from './router'
import { createPinia } from 'pinia';

import ArcoVue from '@arco-design/web-vue/';
import '@arco-design/web-vue/dist/arco.css';

const pinia = createPinia()
const app = createApp(App)


app.use(router);
app.use(ArcoVue);
app.use(pinia);

// document.body.setAttribute('arco-theme', 'dark');

app.mount('#app')
