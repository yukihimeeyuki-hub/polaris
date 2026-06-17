import { createApp } from "vue";
import App from "./App.vue";
import "@/styles/index.css";
import router from "@/router";
import store from "@/pinia";

createApp(App).use(store).use(router).mount("#app");
