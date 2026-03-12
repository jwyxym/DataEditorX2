<template>
	<div class = 'db' :style = "{ '--x' : card.select < 0 ? '0' : '-200%'}">
		<var-app-bar :title = "db.select < 0 ? '' : db.content[db.select].name">
			<template #left>
				<var-button
					color = 'transparent'
					text-color = '#fff'
					round
					text
					@click = 'db.unselect'
					v-if = 'db.select >= 0'
				>
					<var-icon name = 'chevron-left' :size = '24' />
				</var-button>
			</template>
			<template #right>
				<var-menu v-if = 'db.select < 0'>
					<var-button
						color = 'transparent'
						text-color = '#fff'
						round
						text
					>
						<var-icon name = 'menu' :size = '24' />
					</var-button>
					<template #menu>
						<var-cell ripple @click = 'db.open' class = 'pointer'>打开</var-cell>
						<var-cell ripple @click = 'db.save' class = 'pointer'>新建</var-cell>
					</template>
				</var-menu>
				<var-menu v-else>
					<var-button
						color = 'transparent'
						text-color = '#fff'
						round
						text
					>
						<var-icon name = 'menu' :size = '24' />
					</var-button>
					<template #menu>
						<var-cell ripple class = 'pointer'>搜索</var-cell>
						<var-cell ripple class = 'pointer'>新建</var-cell>
					</template>
				</var-menu>
			</template>
		</var-app-bar>
		<TransitionGroup tag = 'div' name = 'move_out'>
			<TransitionGroup
				tag = 'div'
				name = 'move_out'
				v-if = 'db.select < 0'
				key = '0'
				class = 'db_list no-scrollbar'
			>
				<var-cell v-for = '(i, v) in db.content'
					:title = 'i.name'
					:description = 'i.path'
					:border = 'true'
					:key = 'i.path'
					class = 'pointer'
					@click = 'db.click($event, v)'
				>
					<template #extra>
						<var-icon
							name = 'close-circle-outline'
						/>
					</template>
				</var-cell>
			</TransitionGroup>
			<div v-else key = '1' class = 'card_list'>
				<TransitionGroup
					tag = 'div'
					name = 'move_out'
					class = 'no-scrollbar'
				>
					<var-cell v-for = '(i, v) in card.content'
						:title = 'i[1]'
						:description = 'i[0].toString()'
						:border = 'true'
						:key = 'i[0]'
						class = 'pointer'
						@click = 'card.click(v)'
					/>
				</TransitionGroup>
				<var-pagination
					:current = '1'
					:total = 'db.content[db.select].cards?.length ?? 0'
					@change = 'card.change'
				/>
			</div>
		</TransitionGroup>
	</div>
</template>
<script setup lang = 'ts'>
	import { onBeforeMount, reactive, watch } from 'vue';
	import * as dialog from '@tauri-apps/plugin-dialog';
	import invoke from '../script/invoke';
	import DB from '../script/db';
	import sleep from '../script/sleep';

	const card = reactive({
		content : [] as Array<[number, string]>,
		select : -1,
		click : async (index : number) => {
			const code = card.content[index][0];
			if (card.select > -1 && card.select !== code) {
				card.select = -1;
				await sleep(100);
				card.select = code;
			} else
				card.select = card.select === code ? -1 : code;
		},
		change : async (current : number, size : number) => {
			if (card.content.length > 0) {
				card.content.length = 0;
				await sleep(100);
			}
			card.content = db
				.content[db.select].cards?.slice(current * size, (current + 1) * size) ?? []
		}
	});

	const db = reactive({
		content : [] as Array<DB>,
		select : -1,
		click : async (event : Event, index : number) => {
			(event.target as HTMLElement).classList.contains('var-icon') ? await db.remove(index)
				: await (async () => {
					if (!db.content[index].cards)
						db.content[index].cards = await invoke.get_list(db.content[index].path);
					db.select = index;
					await card.change(1, 10);
				})();
		},
		remove : async (index : number) => {
			const path = db.content[index].path;
			db.content.splice(index, 1);
			await invoke.close_db(path);
		},
		open : async () => {
			const file = await dialog.open({
				multiple: false,
				directory: false,
				extensions: ['cdb']
			});
			if (file && await invoke.read_db(file))
				db.content.push(new DB(file));
		},
		save : async () => {
			const file = await dialog.save({
				filters: [{
					name: '',
					extensions: ['cdb']
				}]
			});
			if (file && await invoke.create_db(file))
				db.content.push(new DB(file));
		},
		unselect : () => {
			db.select = -1;
			card.select = -1;
			card.content.length = 0;
		}
	});

	onBeforeMount(async () => {
		db.content = (await invoke.get_dbs())
			.map(i => reactive(new DB(i)));
	});

	watch(() => card.select, () => emit('select', db.content[db.select]?.path ?? '', card.select));

	const emit = defineEmits<{
		select : [db : string, card : number];
	}>();

</script>

<style scoped lang = 'scss'>
	.db {
		border-right: 1px solid black;
		> div:nth-child(2) {
			top: 5px;
			position: relative;
			width: 100%;
			height: calc(100% - 60px);
			> div {
				width: 100%;
				height: 100%;
				position: absolute;
				left: 0;
				top: 0;
			}
			.db_list {
				height: 100%;
				overflow-y: auto;
			}
			.card_list {
				> div:first-child {
					height: calc(100% - 50px);
					overflow-y: auto;
				}
				> div:last-child {
					height: 50px;
				}
			}
			.var-cell {
				.var-icon {
					&:hover {
						color: red;
					}
				}
			}
		}
	}
	.move_out {
		&-enter-active,
		&-leave-active {
			transition: all 0.1s ease;
		}

		&-enter-from,
		&-leave-to {
			transform: translateX(-100%);
		}

		&-enter-to,
		&-leave-from {
			transform: translateX(0);
		}
	}
	@media (max-aspect-ratio: 1/1) {
		.db {
			transform: translateX(var(--x));
		}
	}
</style>