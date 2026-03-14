<template>
	<div ref = 'coder'></div>
</template>
<script setup lang = 'ts'>
	import { onBeforeUnmount, onMounted, ref } from 'vue';
	import * as monaco from 'monaco-editor';
	import PQueue from 'p-queue';

	import invoke from '../script/invoke';
	import toast from '../script/toast';
	import emitter from '../script/emit';
	const coder = ref<HTMLElement | null>(null);
	const queue = new PQueue({ 
		concurrency: 1,
		autoStart: true
	});

	const props = defineProps<{
		lua : string;
		path : string;
	}>();
	let editor;
	const save = async (e : KeyboardEvent) => {
		if ((e.ctrlKey || e.metaKey) && e.key === 's') {
			e.preventDefault();
			queue.add(
				async () => {
					if (await invoke.save_lua(props.path, editor!.getValue()))
						toast.info('保存成功');
				}
			);
		}
	};

	onMounted(() => {
		editor = monaco.editor.create(coder.value!, {
			value: props.lua,
			language: 'lua',
			theme: 'vs-dark',
			automaticLayout: true,
			fontSize: 16,
			minimap: { enabled: true }
		});
		document.addEventListener('keydown', save);
		emitter.on('save', () => queue.add(
			async () => {
				if (await invoke.save_lua(props.path, editor!.getValue()))
					toast.info('保存成功');
			}
		));
	});

	onBeforeUnmount(() => {
		document.removeEventListener('keydown', save);
		emitter.off('save');
		emitter.off('del');
	});
</script>
<style scoped lang = 'scss'>

</style>
