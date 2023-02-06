import { createApp } from "vue";
import "./style.css";
import App from "./App.vue";
import router from "@/plugins/router.js";
import "element-plus/theme-chalk/dark/css-vars.css";
import * as ElementPlusIconsVue from "@element-plus/icons-vue";

let app = createApp(App);
   


for (const [key, component] of Object.entries(ElementPlusIconsVue)) {
  app.component(key, component);
}
app.use(router);
app.mount("#app");
