import { createApp } from "vue";
import App from "./App.vue";
import router from "./router";
import "./styles/main.css";
// 通用字体
import 'vfonts/Lato.css'
// 等宽字体
import 'vfonts/FiraCode.css'
import api from './api' // 导入API模块

const app = createApp(App);
app.use(router as any);
app.config.globalProperties.$api = api // 将API模块挂载到全局
app.mount("#app");
