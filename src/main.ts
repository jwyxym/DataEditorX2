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
	const h = document.documentElement.clientHeight;
	const w = document.documentElement.clientWidth
	const height = Math.min(720, h - 10);
	const width = Math.min(1280, w - 10);
	document.documentElement.style.setProperty('--height', `${height}px`);
	document.documentElement.style.setProperty('--width', `${width}px`);
	let scale;
	if (height === 720 && width === 1280) {
		scale = h / w < 72 / 128 ? h / 720
			: w / 1280;
	} else scale = 1
	document.documentElement.style.setProperty('--scale', `${scale}`);
};
window.addEventListener('resize', resize);
resize();