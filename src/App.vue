<template>
	<div class = 'dex no-scrollbar' v-if = 'show'>
		<List @select = '(db : string, code : number) => { select.db = db; select.code = code; }'/>
		<Transition name = 'opacity'>
			<Card :db = 'select.db' :code = 'select.code' v-if = 'select.code >= 0 || select.code === -2'/>
		</Transition>
	</div>
</template>
<script setup lang = 'ts'>
	import { onBeforeMount, reactive, ref } from 'vue';
	import { Themes, StyleProvider } from '@varlet/ui'
	import config from './script/config';
	import List from './page/list.vue';
	import Card from './page/card.vue';

	const show = ref(false);
	const select = reactive({
		db : '',
		code : -1
	});

	onBeforeMount(async () => {
		StyleProvider(Themes.md3Light);
		await config.init();
		show.value = true;
	});
</script>
<style scoped lang = 'scss'>
	.dex {
		position: relative;
		> div {
			position: absolute;
			left: 0;
			right: 0;
		}
		> div:first-of-type {
			width: calc(var(--width) * 0.3);
			height: var(--height);
			transition: all 0.1s ease;
			z-index: 999;
		}
		> div:nth-child(2) {
			width: calc(var(--width) * 0.67);
			height: var(--height);
			transform: translateX(calc(var(--width) * 0.32));
			transition: all 0.1s ease;
			z-index: 0;
		}
		@media (max-aspect-ratio: 1/1) {
			> div:first-of-type {
				width: var(--width);
			}
			> div:nth-child(2) {
				width: var(--width);
				transform: translateX(0);
			}
		}
	}
	.opacity {
		&-enter-active,
		&-leave-active {
			transition: opacity 0.1s ease;
		}

		&-enter-from,
		&-leave-to {
			opacity: 0;
		}

		&-enter-to,
		&-leave-from {
			opacity: 1;
		}
	}
</style>
<style lang = 'scss'>
	.no-scrollbar, textarea, .var-select__scroller {
		&::-webkit-scrollbar {
			display: none;
		}
	}
	.pointer, .var-chip {
		&:hover {
			cursor: pointer;
		}
	}
	body {
		overflow: hidden;
		transition: background-color .25s, color .25s;
		color: var(--color-text);
		background-color: var(--color-body);
		color-scheme: var(--color-scheme);
		#app {
			transform: scale(var(--scale));
			transform-origin: top left;
		}
	}
	:root {
		--cell-border-color: black !important;
	}
</style>