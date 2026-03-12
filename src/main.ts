import { createApp } from "vue";
import App from "./App.vue";
import Varlet from '@varlet/ui';
import '@varlet/ui/es/style';
import Toast from 'vue-toastification';
import 'vue-toastification/dist/index.css';
const app = createApp(App);
app.use(Varlet);
app.use(Toast, {});
app.mount("#app");

const resize = () => {
	document.documentElement.style.setProperty('--height', `${document.documentElement.clientHeight - 10}px`);
	document.documentElement.style.setProperty('--width', `${document.documentElement.clientWidth - 10}px`);
};
window.addEventListener('resize', resize);
resize();