import { createApp } from "vue";
import App from "./App.vue";
import router from "./router";
import "./styles/main.css";
// 通用字体
import 'vfonts/Lato.css'
// 等宽字体
import 'vfonts/FiraCode.css'

const app = createApp(App);
app.use(router as any);
app.mount("#app");
