import { createApp } from "vue";

import { createPinia } from 'pinia';
import { createPlugin } from '@tauri-store/pinia';

import PrimeVue from 'primevue/config';

import App from "./App.vue";
import router from "./router/router";

import "./main.css";

import navbar from "./components/nav/navbar.vue";
import statusbar from "./components/nav/statusbar.vue";

const app = createApp(App);
const pinia = createPinia();

app.component('navbar', navbar)
app.component('statusbar', statusbar)

pinia.use(createPlugin());

app.use(pinia);
app.use(router);
app.use(PrimeVue);

app.mount("#app");